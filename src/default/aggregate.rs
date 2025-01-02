use std::path::Path;

use tokio::sync::mpsc::Receiver;

use crate::{core::Aggregator as CoreAggregator, stream::MonitorStream};
use crate::{MonitorEvent, Result};

#[derive(Debug, Default)]
pub struct Aggregator {
    rx: Option<Receiver<MonitorEvent>>,
}

impl Aggregator {
    pub fn new() -> Self {
        Self { rx: None }
    }
}

impl CoreAggregator for Aggregator {
    fn start(&mut self, rx: Receiver<MonitorEvent>) -> Result<MonitorStream> {
        todo!()
    }

    fn get_directory_size(&self, path: &Path) -> Result<u64> {
        todo!()
    }
}

pub struct AggregateConfig {}
