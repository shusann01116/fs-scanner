use std::fmt::Debug;

use tokio::sync::mpsc::Receiver;

use crate::{error::Error, stream::MonitorStream, MonitorEvent};

pub trait Watcher: Debug {
    fn start(&mut self) -> Result<Receiver<MonitorEvent>>;
}

pub trait Aggregator: Debug {
    fn start(&mut self, rx: Receiver<MonitorEvent>) -> Result<MonitorStream>;
}

pub type Result<T> = std::result::Result<T, Error>;
