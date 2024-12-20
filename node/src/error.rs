use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum NodeError {
    #[error("invalid Node configuration: {0}")]
    InvalidConfig(String),
}
