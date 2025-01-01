use std::path::Path;

use tokio::sync::mpsc::Receiver;

use crate::{core::Aggregator as CoreAggregator, stream::MonitorStream};
use crate::{MonitorEvent, Result};

#[derive(Debug, Default)]
pub struct Aggregator<'a> {
    rx: Option<Receiver<MonitorEvent<'a>>>,
}

impl Aggregator<'_> {
    pub fn new() -> Self {
        Self { rx: None }
    }
}

impl CoreAggregator for Aggregator<'_> {
    fn start(&mut self, rx: Receiver<MonitorEvent>) -> Result<MonitorStream> {
        todo!()
    }

    fn get_directory_size(&self, path: &Path) -> Result<u64> {
        todo!()
    }
}

pub struct AggregateConfig {}
