use tokio::sync::mpsc::Receiver;

use crate::{core::Aggregator as CoreAggregator, stream::MonitorStream};
use crate::{MonitorEvent, Result};

#[derive(Debug, Default)]
pub struct Aggregator<'a> {
    rx: Option<Receiver<MonitorEvent<'a>>>,
}

impl<'a> Aggregator<'a> {
    pub fn new() -> Self {
        Self { rx: None }
    }
}

impl<'a> CoreAggregator for Aggregator<'a> {
    fn start(&mut self, rx: Receiver<MonitorEvent>) -> Result<MonitorStream> {
        todo!()
    }
}

pub struct AggregateConfig {}