use speech_center_client::{Client, Topic};
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
    #[structopt(short = "T", long = "topic")]
    topic: Option<String>,

    /// Path to the grammar file to use for the recognision
    #[structopt(short = "g", long = "grammar")]
    grammar: Option<String>,

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

#[tokio::main]
async fn main() {
    let opts = Args::from_args();

    let token = std::fs::read_to_string(&opts.token_file).expect("Error reading token from file");
    let token = token.trim().to_string();
    if token.is_empty() {
        panic!("Token cannot be empty");
    }

    let audio = std::fs::read(&opts.audio).expect("Error reading audio file");
    if audio.is_empty() {
        panic!("Audio cannot be empty");
    }

    let mut client = Client::new(&opts.url, &token)
        .await
        .expect("Error creating client");

    match (opts.grammar, opts.topic) {
        (Some(grammar), _) => {
            let grammar = std::fs::read_to_string(&grammar).expect("Error reading grammar file");
            let res = client
                .recognise_with_grammar(&grammar, &opts.language, audio)
                .await
                .expect("Error in recognision");
            println!("Res: {}", res);
        }
        (_, Some(topic)) => {
            let topic = Topic::from_name(&topic).expect("Error converting topic");
            let res = client
                .recognise_with_topic(&opts.language, topic, audio)
                .await
                .expect("Error in recognision");
            println!("Res: {}", res);
        }
        _ => {
            panic!("Either grammar or topic must be defined");
        }
    }
}
