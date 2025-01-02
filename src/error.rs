use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to start monitor: {0}")]
    FailedToStart(String),
    #[error("No aggregator")]
    NoAggregator,
    #[error("No watcher")]
    NoWatcher,
    #[error("Root directory does not exist")]
    RootDirectoryDoesNotExist,
    #[error("Other: {0}")]
    Other(#[source] Box<dyn std::error::Error>),
}
