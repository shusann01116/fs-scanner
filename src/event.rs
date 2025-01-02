use std::path::Path;

pub enum MonitorEvent {
    DirectoryFound(Box<Path>),
    FileFound { path: Box<Path>, size: u64 },
}

pub enum WatcherEvent {
    DirectoryFound(Box<Path>),
    FileFound { path: Box<Path>, size: u64 },
}

impl From<WatcherEvent> for MonitorEvent {
    fn from(event: WatcherEvent) -> Self {
        match event {
            WatcherEvent::DirectoryFound(path) => MonitorEvent::DirectoryFound(path),
            WatcherEvent::FileFound { path, size } => MonitorEvent::FileFound { path, size },
        }
    }
}
