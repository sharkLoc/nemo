use std::io;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum NemoError {
    #[error("stdin not detected")]
    StdinNotDetected,

    #[error("failed to open file: {0}")]
    IoError(#[from] io::Error),

    #[error("failed to serde json: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
}
