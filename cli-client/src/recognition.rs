use speech_center_client::{RecognitionClient, Topic};
use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt)]
/// Run a Speech Center gRPC recognition client
pub struct Recognition {
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
    #[structopt(short = "T", long = "topic")]
    topic: Option<String>,

    /// Path to the ABNF grammar file to use for the recognition
    #[structopt(short = "g", long = "grammar")]
    grammar: Option<String>,

    /// Path to a .wav audio in 8kHz and PCM16 encoding to use for the recognition
    #[structopt(short = "a", long = "audio", required = true)]
    audio: String,

    /// IETF BCP-47 Language to use for the recognition. Supported en-US | es-ES | pt-BR
    #[structopt(
        short = "l",
        long = "language",
        required = true,
        default_value = "en-US"
    )]
    language: String,
}

pub async fn process_subcommand(opts: Recognition) {
    let token = std::fs::read_to_string(&opts.token_file).expect("Error reading token from file");
    let token = token.trim().to_string();
    if token.is_empty() {
        panic!("Token cannot be empty");
    }

    let audio = std::fs::read(&opts.audio).expect("Error reading audio file");
    if audio.is_empty() {
        panic!("Audio cannot be empty");
    }

    let mut client = RecognitionClient::new(&opts.url, &token)
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
