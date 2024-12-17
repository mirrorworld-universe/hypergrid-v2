use thiserror::Error;

#[derive(Error, Debug)]
pub enum NodeError {
    #[error("invalid Node configuration: {0}")]
    InvalidNodeConfig(String),
}
