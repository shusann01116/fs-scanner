use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to start monitor: {0}")]
    FailedToStart(String),
}
