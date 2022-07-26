use bytes::{Buf, Bytes};
use speech_center_client::{AudioFormat, SampleRate, Speaker, SynthesisClient};
use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt)]
/// Run a Speech Center gRPC synthesis client
pub struct Synthesis {
    /// Path to the JWT authentication token file
    #[structopt(short = "t", long = "token-file", required = true)]
    token_file: String,

    /// The URL of the gRPC host or server trying to reach
    #[structopt(
        short = "u",
        long = "url",
        required = true,
        default_value = "https://tts.api.speechcenter.verbio.com"
    )]
    url: String,

    /// Voice to use for the synthesis. Supported Tommy | Annie | Aurora | Luma | David
    #[structopt(short = "v", long = "voice", required = true)]
    voice: String,

    /// Output audio sample rate in Hz. Available 8000
    #[structopt(
        short = "s",
        long = "sample-rate",
        required = true,
        default_value = "8000"
    )]
    sample_rate: u32,

    /// Output audio encoding algorithm. Supported PCM (Signed 16-bit little endian PCM)
    #[structopt(short = "e", long = "encoding", required = true, default_value = "PCM")]
    encoding: String,

    /// Output audio header. Supported: WAV (Wav audio header) | RAW (No header)
    #[structopt(short = "h", long = "header", required = true, default_value = "WAV")]
    header: String,

    /// Text to synthesize to audio.
    #[structopt(short = "T", long = "text", required = true)]
    text: String,

    /// IETF BCP-47 Language to use for the recognition. Supported en-US | es-ES | pt-BR | ca-ES
    #[structopt(
        short = "l",
        long = "language",
        required = true,
        default_value = "en-US"
    )]
    language: String,

    /// Path to store the synthesis resulting audio
    #[structopt(short = "o", long = "output", required = true)]
    output: String,
}

pub async fn process_subcommand(opts: Synthesis) {
    let token = std::fs::read_to_string(&opts.token_file).expect("Error reading token from file");
    let token = token.trim().to_string();
    if token.is_empty() {
        panic!("Token cannot be empty");
    }

    let speaker = Speaker::from_name(&opts.voice, &opts.language)
        .expect("Unknown Voice/Language combination");

    let sample_rate = SampleRate::try_from(opts.sample_rate).expect("Unknown sample rate");

    let audio_format = AudioFormat::from_str(&opts.encoding, &opts.header)
        .expect("Unknown Audio Encoding/Audio Header combination");

    if opts.text.is_empty() {
        panic!("Text cannot be empty");
    }

    let mut client = SynthesisClient::new(&opts.url, &token)
        .await
        .expect("Error creating client");

    let audio = client
        .synthesize(speaker, sample_rate.clone(), audio_format, &opts.text)
        .await
        .expect("Error in recognision");

    println!("Writing: {}B of audio into {}", audio.len(), &opts.output);
    match opts.header.to_lowercase().as_str() {
        "wav" => save_wav(&opts.output, audio, sample_rate),
        "raw" => save_raw(&opts.output, audio),
        _ => panic!("Unknown audio header"),
    }
}

fn save_wav(filename: &str, mut audio: Bytes, sample_rate: SampleRate) {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: sample_rate.into(),
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(filename, spec)
        .expect("Could not write the audio into the provided file output");
    (0..(audio.len() / 2)).for_each(|_| {
        writer
            .write_sample(audio.get_i16_le())
            .expect("Error writing data into provided file output");
    });
    writer.finalize().expect("Error saving the file output");
}

fn save_raw(filename: &str, audio: Bytes) {
    std::fs::write(filename, audio)
        .expect("Could not write the audio into the provided file output");
}
