#[macro_use]
extern crate tracing;

use crate::worker::{Payload, Worker};
use anyhow::{anyhow, Result};
use async_channel::Sender;
use speech_center_client::{SpeechCenterError, Topic};
use std::path::Path;
use structopt::StructOpt;
use tokio::fs::DirEntry;
use tracing::Instrument;

mod log;
mod worker;

#[derive(Clone, Debug, StructOpt)]
struct Args {
    /// Log level. Must be TRACE | DEBUG | INFO | WARN | ERROR
    #[structopt(
        short = "L",
        long = "log-level",
        required = false,
        default_value = "info"
    )]
    log_level: String,

    /// Path to the JWT authentication token file
    #[structopt(short = "t", long = "token-file", required = true)]
    token_file: String,

    /// The URL of the gRPC host or server trying to reach
    #[structopt(
        short = "u",
        long = "url",
        required = true,
        default_value = "csr.api.speechcenter.verbio.com"
    )]
    url: String,

    /// Topic to use for the recognition. Must be GENERIC | BANKING | TELCO
    #[structopt(short = "T", long = "topic", required = true)]
    topic: String,

    /// Directory containing .wav audios in 8kHz and PCM16 encoding to use for the recognition
    #[structopt(short = "d", long = "dir", required = true)]
    source_dir: String,

    /// Destination directory for the transcriptions
    #[structopt(short = "D", long = "dest-dir", required = true)]
    dest_dir: String,

    /// IETF BCP-47 Language to use for the recognition. Supported en-US | es-ES | pt-BR
    #[structopt(
        short = "l",
        long = "language",
        required = true,
        default_value = "en-US"
    )]
    language: String,

    /// Number of workers to use for the recognition
    #[structopt(short = "w", long = "workers", default_value = "4")]
    workers: u16,
}

async fn start_workers(url: &str, token: &str, count: u16) -> Result<Sender<Payload>> {
    let (tx, rx) = async_channel::bounded(count as usize);

    for idx in 0..count {
        let url = url.to_string();
        let token = token.to_string();
        let rx = rx.clone();
        tokio::spawn(async move {
            let span = info_span!("Worker", worker=%idx);
            let w = Worker::new(&url, &token, rx)
                .await
                .expect("Error starting worker");
            w.start().instrument(span).await;
        });
    }
    Ok(tx)
}

async fn ensure_dir_exists(dir: &str) -> Result<()> {
    let p = Path::new(dir);
    if p.exists() {
        if p.is_dir() {
            Ok(())
        } else {
            Err(anyhow::anyhow!(format!(
                "Path exists but it's not a dir: {}",
                dir
            )))
        }
    } else {
        tokio::fs::create_dir_all(dir).await.map_err(|e| {
            SpeechCenterError::Unknown(format!("Error creating dirs [dir={}]: {}", dir, e))
        })?;
        Ok(())
    }
}

fn entry_to_payload(
    f: &DirEntry,
    language: String,
    topic: Topic,
    dest_dir: &str,
) -> Result<Option<Payload>> {
    let file_path = f.path();

    let extension = file_path.extension().ok_or_else(|| {
        anyhow!(
            "File must contain an extension [file={}]",
            file_path.display()
        )
    })?;
    let extension = extension.to_str().ok_or_else(|| {
        anyhow!(
            "File extension not convertible to str [file={}]",
            file_path.display()
        )
    })?;
    if extension != "wav" {
        return Ok(None);
    }

    let source = format!("{}", file_path.display());
    let stem = file_path.file_stem().ok_or_else(|| {
        anyhow!(
            "File name does not contain stem [file={}]",
            file_path.display()
        )
    })?;
    let stem = stem.to_str().ok_or_else(|| {
        anyhow!(
            "File stem not convertible to str [file={}]",
            file_path.display()
        )
    })?;

    let dest = Path::new(dest_dir).join(format!("{}.txt", stem));
    if dest.exists() {
        return Ok(None);
    }

    let dest = format!("{}", dest.display());

    Ok(Some(Payload::File {
        source,
        dest,
        language,
        topic,
    }))
}

async fn run(
    url: &str,
    token: &str,
    language: &str,
    source_dir: &str,
    dest_dir: &str,
    topic: Topic,
    workers: u16,
) -> Result<()> {
    debug!("Ensuring directories exist");
    ensure_dir_exists(source_dir).await?;
    ensure_dir_exists(dest_dir).await?;

    info!("Starting {} workers", workers);
    let tx = start_workers(url, token, workers).await?;
    info!("Workers started");

    let mut dir = tokio::fs::read_dir(source_dir)
        .await
        .map_err(|e| anyhow::anyhow!(format!("Error iterating dir: {}", e)))?;

    while let Ok(Some(f)) = dir.next_entry().await {
        let payload = entry_to_payload(&f, language.to_string(), topic.clone(), dest_dir)
            .map_err(|e| anyhow!("Error creating Payload: {}", e))?;
        if let Some(payload) = payload {
            info!("Sending file {}", f.path().display());
            if let Err(e) = tx.send(payload).await {
                return Err(anyhow::anyhow!(format!("Error sending task: {}", e)));
            }
        }
    }

    for _ in 0..workers {
        let (close_tx, close_rx) = async_channel::unbounded();
        let _ = tx.send(Payload::Close(close_tx)).await;
        let _ = close_rx.recv().await;
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let opts = Args::from_args();

    log::init_logger(&opts.log_level);
    debug!("Args: {:?}", opts);

    let topic = Topic::from_name(&opts.topic).expect("Error converting topic");

    let token = std::fs::read_to_string(&opts.token_file).expect("Error reading token from file");
    let token = token.trim().to_string();
    if token.is_empty() {
        panic!("Token cannot be empty");
    }

    if let Err(e) = run(
        &opts.url,
        &token,
        &opts.language,
        &opts.source_dir,
        &opts.dest_dir,
        topic,
        opts.workers,
    )
    .await
    {
        panic!("Error in execution: {}", e)
    }
}
