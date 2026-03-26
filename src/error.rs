use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid argument: {0}")]
    InvalidArgument(String),
}
