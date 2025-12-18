use thiserror::Error;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum AppError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Image processing error: {0}")]
    ImageProcessing(#[from] image::ImageError),
    #[error("Failed to send data to receiver")]
    ChannelSend,
    #[error("Failed to receive data from sender")]
    ChannelReceive,
    #[error("Command execution failed: {0}")]
    Command(String),
    #[error("JSON parsing failed: {0}")]
    Json(#[from] serde_json::Error),
    #[error("UTF-8 conversion failed: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("Home directory not found")]
    HomeDirNotFound,
    #[error("Invalid path: {0}")]
    InvalidPath(String),
}
