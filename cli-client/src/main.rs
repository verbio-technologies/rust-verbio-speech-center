use speech_center_client::{Client, Result, Topic};
use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt)]
struct Args {
    /// Path to the token file
    #[structopt(short = "t", long = "token-file", required = true)]
    token_file: String,

    /// URL of the server
    #[structopt(
        short = "u",
        long = "url",
        required = true,
        default_value = "https://speechcenter.verbio.com:2424"
    )]
    url: String,

    /// Topic to use for the recognision. Must be GENERIC | BANKING | TELCO
    #[structopt(short = "T", long = "topic", required = true)]
    topic: String,

    /// Audio to use for the recognision
    #[structopt(short = "a", long = "audio", required = true)]
    audio: String,

    /// Language to use for the recognision
    #[structopt(
        short = "l",
        long = "language",
        required = true,
        default_value = "en-US"
    )]
    language: String,
}

async fn run(url: &str, token: &str, language: &str, audio: Vec<u8>, topic: Topic) -> Result<()> {
    let mut client = Client::new(url, token)
        .await
        .expect("Error creating client");
    let res = client
        .recognise_with_topic(language, topic, audio)
        .await
        .expect("Error in recognision");
    println!("Res: {}", res);
    Ok(())
}

#[tokio::main]
async fn main() {
    let opts = Args::from_args();
    let topic = Topic::from_name(&opts.topic).expect("Error converting topic");

    let token = std::fs::read_to_string(&opts.token_file).expect("Error reading token from file");
    let token = token.trim().to_string();
    if token.is_empty() {
        panic!("Token cannot be empty");
    }

    let audio_file = std::fs::read(&opts.audio).expect("Error reading audio file");
    if audio_file.is_empty() {
        panic!("Audio cannot be empty");
    }

    if let Err(e) = run(&opts.url, &token, &opts.language, audio_file, topic).await {
        panic!("Error in execution: {}", e)
    }
}
