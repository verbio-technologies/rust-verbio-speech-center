mod client;
mod csr_grpc_gateway;
mod error;

pub use client::{Client, Topic};
pub use error::SpeechCenterError;
pub type Result<T, E = SpeechCenterError> = std::result::Result<T, E>;
