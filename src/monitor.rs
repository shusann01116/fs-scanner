use crate::{
    core::{Aggregator, Result, Watcher},
    error::Error,
    stream::MonitorStream,
};
use std::{fmt::Debug, path::Path};

#[derive(Debug, Default)]
pub struct Monitor {
    watcher: Option<Box<dyn Watcher>>,
    aggregator: Option<Box<dyn Aggregator>>,
}

impl Monitor {
    pub fn new() -> Self {
        Self {
            watcher: None,
            aggregator: None,
        }
    }

    pub fn watch<T: Watcher + 'static>(mut self, watcher: T) -> Self {
        self.watcher = Some(Box::new(watcher));
        self
    }

    pub fn aggregate<T: Aggregator + 'static>(mut self, aggregator: T) -> Self {
        self.aggregator = Some(Box::new(aggregator));
        self
    }

    pub fn start(&mut self) -> Result<MonitorStream> {
        if self.aggregator.is_none() || self.watcher.is_none() {
            return Err(Error::FailedToStart(
                "Aggregator or Watcher is not set".to_string(),
            ));
        }

        let rx = self.watcher.as_mut().unwrap().start()?;
        self.aggregator.as_mut().unwrap().start(rx)
    }

    pub async fn get_directory_size(&self, path: impl AsRef<Path>) -> Result<u64> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::default::{Aggregator, WatchConfig, Watcher};
    use futures::executor::block_on_stream;

    #[tokio::test]
    async fn err_no_aggregator() {
        let mut monitor = Monitor::new().watch(Watcher::new(WatchConfig::default()));
        let err = monitor.start().err().unwrap();
        assert_eq!(
            err.to_string(),
            "Failed to start monitor: Aggregator or Watcher is not set"
        );
    }

    #[tokio::test]
    async fn err_no_watcher() {
        let mut monitor = Monitor::new().aggregate(Aggregator::new());
        let err = monitor.start().err().unwrap();
        assert_eq!(
            err.to_string(),
            "Failed to start monitor: Aggregator or Watcher is not set"
        );
    }

    #[tokio::test]
    async fn test_start() {
        let mut monitor = Monitor::new()
            .watch(Watcher::new(WatchConfig::default()))
            .aggregate(Aggregator::new());
        let stream = monitor.start().unwrap();
        let mut iter = block_on_stream(stream);
        assert!(iter.next().is_none());
    }
}