use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

use tokio::sync::mpsc;

use crate::{error::ScannerError, events::ScanEvent, result::Result, streams::ScannerEventStream};

#[derive(Default, Debug, Clone)]
pub struct Scanner {
    directory: Option<Arc<PathBuf>>,
    watching: bool,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            directory: None,
            watching: false,
        }
    }

    pub fn with_directory<P: AsRef<Path>>(mut self, directory: P) -> Self {
        self.directory = Some(Arc::new(directory.as_ref().to_owned()));
        self
    }

    pub fn watch_changes(mut self) -> Self {
        self.watching = true;
        self
    }

    pub fn start(self) -> Result<ScannerEventStream> {
        let directory = self.directory.ok_or(ScannerError::NoDirectorySpecified)?;
        let (tx, rx) = mpsc::unbounded_channel();
        let event_stream = ScannerEventStream::new(rx);

        tokio::spawn(async move {
            // TODO: handle error properly
            scan_directory(directory.as_ref(), tx.clone())
                .await
                .unwrap();

            // after initial scan completes, shift to watching for changes
            // TODO: handle error properly
            if self.watching {
                if let Err(e) = watch_fs_loop(directory.as_ref(), tx.clone()).await {
                    eprintln!("Error watching filesystem: {}", e);
                }
            }
        });

        Ok(event_stream)
    }
}

async fn scan_directory<P: AsRef<Path>>(
    directory: P,
    tx: mpsc::UnboundedSender<ScanEvent>,
) -> Result<()> {
    let files = fs::read_dir(directory)?;

    for file in files {
        let file = file?;
        let path = file.path();
        let metadata = file.metadata()?;

        if metadata.is_file() {
            let size = metadata.len();
            tx.send(ScanEvent::FileFound {
                path: path.to_owned(),
                size,
            })?;
        }

        if metadata.is_dir() {
            // TODO: handle error properly
            spawn_scan_directory(path, tx.clone());
        }
    }

    Ok(())
}

fn spawn_scan_directory<P: AsRef<Path>>(directory: P, tx: mpsc::UnboundedSender<ScanEvent>) {
    let directory = directory.as_ref().to_owned();
    let tx = tx.clone();
    tokio::spawn(async move {
        scan_directory(&directory, tx).await.unwrap();
    });
}

#[allow(unused)]
async fn watch_fs_loop<P: AsRef<Path>>(
    directory: P,
    tx: mpsc::UnboundedSender<ScanEvent>,
) -> Result<()> {
    todo!()
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
            ScanEvent::FileFound { path, .. } => {
                assert_eq!(path.parent().unwrap(), dir.path());
            }
            _ => {
                panic!("Unexpected event received");
            }
        }
    }
}
