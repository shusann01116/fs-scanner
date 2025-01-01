use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use futures_util::StreamExt;
use tokio::sync::{mpsc, RwLock};

use crate::scanner::{FileEvent, ScannerEventStream};

#[derive(Default, Debug, Clone)]
pub struct Aggregator {
    file_sizes: Arc<RwLock<HashMap<PathBuf, u64>>>,
}

impl Aggregator {
    pub fn new() -> Self {
        Self {
            file_sizes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Starts processing the scanner events and returns a stream of updates
    pub fn process_events(&self, mut scanner_stream: ScannerEventStream) -> ScannerEventStream {
        let (tx, rx) = mpsc::unbounded_channel();
        let file_sizes = self.file_sizes.clone();

        tokio::spawn(async move {
            while let Some(event) = scanner_stream.next().await {
                let mut sizes = file_sizes.write().await;
                match &event {
                    FileEvent::FileFound { path, size } => {
                        sizes.insert(path.clone(), *size);
                        tx.send(event).unwrap();
                    }
                    FileEvent::FileAdded { path, size } => {
                        sizes.insert(path.clone(), *size);
                        tx.send(event).unwrap();
                    }
                    FileEvent::FileRemoved { path } => {
                        sizes.remove(path);
                        tx.send(event).unwrap();
                    }
                    FileEvent::FileModified { path, size } => {
                        sizes.insert(path.clone(), *size);
                        tx.send(event).unwrap();
                    }
                    FileEvent::InitialScanComplete => {
                        // 初期スキャン完了時の処理が必要な場合はここに追加
                        tx.send(event).unwrap();
                    }
                }
            }
        });

        ScannerEventStream::new(rx)
    }

    /// Returns the total size of all files under the specified directory
    pub async fn get_directory_size<P: AsRef<Path>>(&self, directory: P) -> u64 {
        let sizes = self.file_sizes.read().await;
        Self::calculate_dir_size(&sizes, directory.as_ref())
    }

    fn calculate_dir_size(sizes: &HashMap<PathBuf, u64>, directory: &Path) -> u64 {
        sizes
            .iter()
            .filter(|(path, _)| path.starts_with(directory))
            .map(|(_, size)| size)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{scanner::Scanner, test_tools};

    #[tokio::test]
    async fn test_directory_size_calculation() {
        let dir = test_tools::setup_temp_dir();
        let file1_path = test_tools::create_random_file(dir.path());
        let file2_path = test_tools::create_random_file(dir.path());
        let actual_total_size =
            file1_path.metadata().unwrap().len() + file2_path.metadata().unwrap().len();

        let scanner = Scanner::new().with_directory(&dir.path());
        let stream = scanner.start().expect("Failed to start scanner");

        let aggregator = Aggregator::new();
        let mut updates = aggregator.process_events(stream);

        // Wait for some updates
        while let Some(update) = updates.next().await {
            println!("Update received: {:?}", update);
        }

        let total_size = aggregator.get_directory_size(dir.path()).await;
        assert_eq!(total_size, actual_total_size);
    }
}
