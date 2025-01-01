use std::{path::Path, sync::Arc};

use tokio::sync::mpsc::Receiver;

use crate::core::Watcher as CoreWatcher;
use crate::{MonitorEvent, Result};

#[derive(Debug, Default)]
pub struct Watcher<'a> {
    config: WatchConfig,
    receiver: Option<Receiver<MonitorEvent<'a>>>,
}

impl<'a> Watcher<'a> {
    pub fn new(config: WatchConfig) -> Self {
        Self {
            config,
            receiver: None,
        }
    }
}

impl<'a> CoreWatcher for Watcher<'a> {
    fn start(&mut self) -> Result<Receiver<MonitorEvent>> {
        todo!()
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
