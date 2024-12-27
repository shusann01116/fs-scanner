use std::io;

use thiserror::Error;
use tokio::sync::mpsc;

use crate::events::FileEvent;

#[derive(Debug, Error)]
pub enum ScannerError {
    #[error("No directory specified")]
    NoDirectorySpecified,
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    #[error("Channel error: {0}")]
    ChannelError(#[from] mpsc::error::SendError<FileEvent>),
    #[error("Other error: {0}")]
    Other(#[from] anyhow::Error),
}
