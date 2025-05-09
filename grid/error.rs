use thiserror::Error;

#[derive(Error, Debug)]
pub enum GridError {
    #[error("unknown error")]
    Unknown,
}
