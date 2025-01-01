use std::{fmt::Debug, path::Path};

use tokio::sync::mpsc::Receiver;

use crate::{error::Error, stream::MonitorStream, MonitorEvent};

pub trait Watcher: Debug {
    fn start(&mut self) -> Result<Receiver<MonitorEvent>>;
}

pub trait Aggregator: Debug {
    fn start(&mut self, rx: Receiver<MonitorEvent>) -> Result<MonitorStream>;
    fn get_directory_size(&self, path: &Path) -> Result<u64>;
}

pub type Result<T> = std::result::Result<T, Error>;
