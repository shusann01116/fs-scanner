use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
/// FileEvent is an event emitted by the scanner.
pub enum FileEvent {
    /// Emitted when the initial scan is complete.
    InitialScanComplete,
    /// Emitted when a file is found in the directory.
    FileFound {
        path: PathBuf,
        size: u64,
    },
    FileAdded {
        path: PathBuf,
        size: u64,
    },
    FileRemoved {
        path: PathBuf,
    },
    FileModified {
        path: PathBuf,
        size: u64,
    },
}
