use thiserror::Error;

#[derive(Error, Debug)]
pub enum HssnError {
    #[error("unknown error")]
    Unknown,
}
