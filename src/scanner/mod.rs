mod events;
mod streams;

use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::{result::Result, ScannerError};
pub use events::FileEvent;
use notify::event::{CreateKind, EventKind, ModifyKind};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc as std_mpsc;
pub use streams::ScannerEventStream;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;

#[derive(Default, Debug, Clone)]
pub struct Scanner {
    directory: Option<Arc<PathBuf>>,
    watch: bool,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            directory: None,
            watch: false,
        }
    }

    pub fn with_directory<P: AsRef<Path>>(mut self, directory: P) -> Self {
        self.directory = Some(Arc::new(directory.as_ref().to_owned()));
        self
    }

    pub fn watch_changes(mut self) -> Self {
        self.watch = true;
        self
    }

    pub fn start(&self) -> Result<ScannerEventStream> {
        let directory = self
            .directory
            .as_ref()
            .ok_or(ScannerError::NoDirectorySpecified)?
            .clone();
        let (tx, rx) = mpsc::unbounded_channel();
        let event_stream = ScannerEventStream::new(rx);

        let watch = self.watch;
        tokio::spawn(async move {
            // TODO: handle error properly
            Self::scan_directory(directory.as_ref(), tx.clone())
                .await
                .unwrap();

            // after initial scan completes, shift to watching for changes
            // TODO: handle error properly
            if watch {
                if let Err(e) = Self::watch_fs_loop(directory.as_ref(), tx.clone()).await {
                    eprintln!("Error watching filesystem: {}", e);
                }
            }
        });

        Ok(event_stream)
    }

    async fn scan_directory<P: AsRef<Path>>(
        directory: P,
        tx: mpsc::UnboundedSender<FileEvent>,
    ) -> Result<()> {
        let files = fs::read_dir(directory)?;

        for file in files {
            let file = file?;
            let path = file.path();
            let metadata = file.metadata()?;

            if metadata.is_file() {
                let size = metadata.len();
                tx.send(FileEvent::FileFound {
                    path: path.to_owned(),
                    size,
                })?;
            }

            if metadata.is_dir() {
                // TODO: handle error properly
                Self::spawn_scan_directory(path, tx.clone());
            }
        }

        Ok(())
    }

    fn spawn_scan_directory<P: AsRef<Path>>(directory: P, tx: mpsc::UnboundedSender<FileEvent>) {
        let directory = directory.as_ref().to_owned();
        let tx = tx.clone();
        tokio::spawn(async move {
            Self::scan_directory(&directory, tx).await.unwrap();
        });
    }

    async fn watch_fs_loop<P: AsRef<Path>>(
        directory: P,
        tx: UnboundedSender<FileEvent>,
    ) -> Result<()> {
        let (notify_tx, notify_rx) = std_mpsc::channel();

        // Initialize the watcher
        let mut watcher: RecommendedWatcher = Watcher::new(notify_tx, notify::Config::default())
            .map_err(|e| ScannerError::Other(anyhow::anyhow!(e)))?;

        watcher
            .watch(directory.as_ref(), RecursiveMode::Recursive)
            .map_err(|e| ScannerError::Other(anyhow::anyhow!(e)))?;

        // Keep watcher in scope to prevent it from being dropped
        loop {
            match notify_rx.recv() {
                Ok(event) => {
                    let event = match event {
                        Ok(event) => event,
                        Err(_) => continue,
                    };

                    dbg!(&event);
                    match event.kind {
                        EventKind::Create(CreateKind::File) => {
                            for path in event.paths {
                                if path.is_file() {
                                    if let Ok(metadata) = path.metadata() {
                                        let size = metadata.len();
                                        dbg!("FileEvent::FileAdded emitted: {:?}", &path);
                                        tx.send(FileEvent::FileAdded { path, size }).unwrap();
                                    }
                                }
                            }
                        }
                        EventKind::Modify(ModifyKind::Data(_)) => {
                            for path in event.paths {
                                if path.is_file() {
                                    if let Ok(metadata) = path.metadata() {
                                        let size = metadata.len();
                                        dbg!("FileEvent::FileModified emitted: {:?}", &path);
                                        let _ = tx.send(FileEvent::FileModified { path, size });
                                    }
                                }
                            }
                        }
                        EventKind::Remove(_) => {
                            for path in event.paths {
                                dbg!("FileEvent::FileRemoved emitted: {:?}", &path);
                                let _ = tx.send(FileEvent::FileRemoved { path });
                            }
                        }
                        _ => {}
                    }
                }
                Err(_) => break,
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use futures_util::StreamExt;

    use crate::test_tools;

    use super::*;

    #[tokio::test]
    async fn no_directory_specified() {
        let scanner = Scanner::new();
        let result = scanner.start();
        assert_eq!(
            result.unwrap_err().to_string(),
            ScannerError::NoDirectorySpecified.to_string()
        );
    }

    #[tokio::test]
    async fn scan_with_a_single_file() {
        let dir = test_tools::setup_temp_dir();
        let scanner = Scanner::new().with_directory(&dir.path());

        let mut stream = scanner.start().expect("Failed to start scanner");

        test_tools::create_random_file(&dir.path());

        let event = stream.next().await.expect("No event received");
        match event {
            FileEvent::FileFound { path, .. } => {
                assert_eq!(path.parent().unwrap(), dir.path());
            }
            _ => {
                panic!("Unexpected event received");
            }
        }
    }
}
