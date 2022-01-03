#[derive(Clone, Debug, thiserror::Error)]
pub enum SpeechCenterError {
    #[error("Connection error: {}", _0)]
    Connection(String),
    #[error("Recognision error: {}", _0)]
    Recognision(String),
    #[error("Unknown error: {}", _0)]
    Unknown(String),
}
