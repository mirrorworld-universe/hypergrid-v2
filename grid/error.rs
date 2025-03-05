use thiserror::Error;

#[derive(Error, Debug)]
pub enum NodeError {
    #[error("unknown error")]
    Unknown,
}
