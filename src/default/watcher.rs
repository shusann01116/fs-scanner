use std::{path::Path, sync::Arc};

use tokio::sync::mpsc::{Receiver, Sender};

use crate::core::Watcher as CoreWatcher;
use crate::error::Error;
use crate::{MonitorEvent, Result};

#[derive(Debug, Default)]
pub struct Watcher {
    config: WatchConfig,
    sender: Option<Sender<MonitorEvent>>,
}

impl Watcher {
    pub fn new(config: WatchConfig) -> Self {
        Self {
            config,
            sender: None,
        }
    }
}

impl CoreWatcher for Watcher {
    fn start(&mut self) -> Result<Receiver<MonitorEvent>> {
        Ok(self
            .config
            .root
            .exists()
            .then(|| {
                let (tx, rx) = tokio::sync::mpsc::channel(100);
                self.sender = Some(tx);
                rx
            })
            .ok_or(Error::RootDirectoryDoesNotExist)?)
    }
}

#[derive(Debug)]
pub struct WatchConfig {
    pub root: Arc<Path>,
    pub watch: bool,
}

impl Default for WatchConfig {
    fn default() -> Self {
        Self {
            root: Arc::from(Path::new(".")),
            watch: true,
        }
    }
}
