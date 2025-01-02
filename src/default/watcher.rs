use std::{path::Path, sync::Arc};

use tokio::sync::mpsc::{Receiver, Sender};

use crate::core::Watcher as CoreWatcher;
use crate::error::Error;
use crate::event::WatcherEvent;
use crate::Result;

#[derive(Debug, Default)]
pub struct Watcher {
    config: WatchConfig,
}

impl Watcher {
    pub fn new(config: WatchConfig) -> Self {
        Self { config }
    }

    fn start(&mut self) -> Result<Receiver<WatcherEvent>> {
        if !self.config.root.exists() {
            return Err(Error::RootDirectoryDoesNotExist);
        }
        let rx = self.start_inner()?;

        Ok(rx)
    }

    /// Scan the root directory and send events to the sender
    fn start_inner(&self) -> Result<Receiver<WatcherEvent>> {
        let (tx, mut rx) = tokio::sync::mpsc::channel(100);
        self.initial_scan(tx.clone())?;
        if self.config.watch {
            Watcher::watch(tx.clone())?;
        } else {
            rx.close();
        }
        Ok(rx)
    }

    fn initial_scan(&self, sender: Sender<WatcherEvent>) -> Result<()> {
        todo!()
    }

    /// Initialize parallel watcher
    fn watch(sender: Sender<WatcherEvent>) -> Result<()> {
        tokio::spawn(async move {
            Watcher::watch_inner(sender).await.unwrap();
        });
        Ok(())
    }

    async fn watch_inner(sender: Sender<WatcherEvent>) -> Result<()> {
        todo!()
    }
}

impl CoreWatcher for Watcher {
    fn start(&mut self) -> Result<Receiver<WatcherEvent>> {
        self.start()
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
            watch: false,
        }
    }
}
