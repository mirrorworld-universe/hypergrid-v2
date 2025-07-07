use thiserror::Error;

#[derive(Error, Debug)]
pub enum GridError {
    #[error("unknown error")]
    Unknown,
    #[error("failed to write account {0}")]
    AccountWriteFailed(String),
}
