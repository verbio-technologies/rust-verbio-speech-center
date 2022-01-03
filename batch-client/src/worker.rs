use async_channel::{Receiver, Sender};
use speech_center_client::{Client, Result, SpeechCenterError, Topic};

pub enum Payload {
    File {
        source: String,
        dest: String,
        topic: Topic,
        language: String,
    },
    Close(Sender<()>),
}

pub struct Worker {
    client: Client,
    rx: Receiver<Payload>,
}

impl Worker {
    pub async fn new(url: &str, token: &str, rx: Receiver<Payload>) -> Result<Self> {
        let client = Client::new(url, token).await?;
        Ok(Self { client, rx })
    }

    pub async fn start(mut self) {
        while let Ok(p) = self.rx.recv().await {
            match p {
                Payload::File {
                    source,
                    dest,
                    topic,
                    language,
                } => {
                    debug!("Processing file {}", source);
                    if let Err(e) = self.process(&source, &dest, topic, language).await {
                        eprintln!(
                            "Error processing file [source={}] [dest={}]: {:?}",
                            source, dest, e
                        );
                    }
                }
                Payload::Close(s) => {
                    info!("Shutting worker down");
                    let _ = s.send(()).await;
                    break;
                }
            }
        }
    }

    async fn process(
        &mut self,
        source: &str,
        dest: &str,
        topic: Topic,
        language: String,
    ) -> Result<()> {
        debug!("Reading file contents: {}", source);
        let audio = tokio::fs::read(source).await.map_err(|e| {
            SpeechCenterError::Unknown(format!(
                "Error reading source file [source={}]: {}",
                source, e
            ))
        })?;

        debug!("Performing recognision");
        let res = self
            .client
            .recognise_with_topic(&language, topic, audio)
            .await?;

        debug!("Writing transcription: {}", dest);
        tokio::fs::write(dest, res.as_bytes()).await.map_err(|e| {
            SpeechCenterError::Unknown(format!(
                "Error writing transcription [dest={}]: {}",
                dest, e
            ))
        })?;
        Ok(())
    }
}
