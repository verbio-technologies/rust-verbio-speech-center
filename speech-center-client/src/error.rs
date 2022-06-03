#[derive(Clone, Debug, thiserror::Error)]
pub enum SpeechCenterError {
    #[error("Connection error: {}", _0)]
    Connection(String),
    #[error("Recognision error: {}", _0)]
    Recognision(String),
    #[error("Synthesis error: {}", _0)]
    Synthesis(String),
    #[error("Unknown error: {}", _0)]
    Unknown(String),
}
