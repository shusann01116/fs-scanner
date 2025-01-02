use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use tokio::sync::mpsc::Receiver;
use tokio::sync::Mutex;

use crate::event::WatcherEvent;
use crate::Result;
use crate::{core::Aggregator as CoreAggregator, stream::MonitorStream};

type PathStr = String;

#[derive(Debug, Default)]
pub struct Aggregator {
    data: Arc<Mutex<HashMap<PathStr, u64>>>,
}

impl Aggregator {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl CoreAggregator for Aggregator {
    fn start(&self, mut rx: Receiver<WatcherEvent>) -> Result<MonitorStream> {
        let (tx, stream_rx) = tokio::sync::mpsc::channel(100);
        let data = self.data.clone();

        tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                match &event {
                    WatcherEvent::FileFound { path, size } => {
                        data.lock()
                            .await
                            .insert(path.to_string_lossy().to_string(), *size);
                        tx.send(event.into()).await.unwrap();
                    }
                    _ => {}
                }
            }
        });

        Ok(MonitorStream::new(stream_rx))
    }

    fn get_directory_size(&self, path: &Path) -> Result<u64> {
        todo!()
    }
}

pub struct AggregateConfig {}
