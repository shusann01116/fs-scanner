use std::path::PathBuf;

#[derive(Debug)]
/// ScanEvent is an event emitted by the scanner.
pub enum ScanEvent {
    /// Emitted when a file is found in the directory.
    FileFound {
        path: PathBuf,
        size: u64,
    },
    /// Emitted when the initial scan is complete.
    InitialScanComplete,
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
