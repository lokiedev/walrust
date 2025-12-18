use thiserror::Error;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum WalrustError {
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),
}
