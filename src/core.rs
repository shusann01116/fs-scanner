use std::{fmt::Debug, path::Path};

use tokio::sync::mpsc::Receiver;

use crate::{error::Error, event::WatcherEvent, stream::MonitorStream};

pub trait Watcher: Debug {
    fn start(&mut self) -> Result<Receiver<WatcherEvent>>;
}

pub trait Aggregator: Debug {
    fn start(&self, rx: Receiver<WatcherEvent>) -> Result<MonitorStream>;
    fn get_directory_size(&self, path: &Path) -> Result<u64>;
}

pub type Result<T> = std::result::Result<T, Error>;
