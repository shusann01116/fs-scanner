use std::path::Path;

pub enum MonitorEvent {
    DirectoryFound(Box<Path>),
    FileFound { path: Box<Path>, size: u64 },
}
