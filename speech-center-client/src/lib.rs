mod error;
mod recognizer_client;
mod synthesizer_client;

mod csr_grpc_gateway;
#[path = "speechcenter.tts.v1.rs"]
mod speechcenter_tts_v1;

pub use error::SpeechCenterError;
pub use recognizer_client::{Client as RecognitionClient, Topic};
pub use synthesizer_client::{AudioFormat, Client as SynthesisClient, SampleRate, Speaker};
pub type Result<T, E = SpeechCenterError> = std::result::Result<T, E>;
