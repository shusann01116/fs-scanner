use std::path::Path;

use crate::{
    aggregator::Aggregator, result::Result, scanner::Scanner, streams::ScannerEventStream,
};

/// Monitors disk usage of a directory and its subdirectories
#[derive(Default, Debug, Clone)]
pub struct DiskUsageMonitor {
    scanner: Scanner,
    aggregator: Aggregator,
}

impl DiskUsageMonitor {
    pub fn new() -> Self {
        Self {
            scanner: Scanner::new(),
            aggregator: Aggregator::new(),
        }
    }

    /// Sets the directory to monitor
    pub fn with_directory<P: AsRef<Path>>(mut self, directory: P) -> Self {
        self.scanner = self.scanner.with_directory(directory);
        self
    }

    /// Enables watching for filesystem changes
    pub fn watch_changes(mut self) -> Self {
        self.scanner = self.scanner.watch_changes();
        self
    }

    /// Starts monitoring the directory and returns a stream of updates
    pub fn start(&self) -> Result<ScannerEventStream> {
        let stream = self.scanner.start()?;
        Ok(self.aggregator.process_events(stream))
    }

    /// Returns the total size of all files under the specified directory
    /// This method does not actually scan the directory, it just returns the size of the files that have been found by the scanner.
    pub async fn get_directory_size<P: AsRef<Path>>(&self, directory: P) -> u64 {
        self.aggregator.get_directory_size(directory).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{test_tools, FileEvent};
    use futures_util::StreamExt;

    #[tokio::test]
    async fn test_disk_usage_monitoring() {
        let dir = test_tools::setup_temp_dir();
        let file1_path = test_tools::create_random_file(dir.path());
        let file2_path = test_tools::create_random_file(dir.path());
        let actual_total_size =
            file1_path.metadata().unwrap().len() + file2_path.metadata().unwrap().len();

        let monitor = DiskUsageMonitor::new().with_directory(&dir.path());
        let mut updates = monitor.start().expect("Failed to start monitoring");

        // Wait for some updates
        let event_1 = updates.next().await.unwrap();
        let event_2 = updates.next().await.unwrap();
        assert_eq!(
            event_1,
            FileEvent::FileFound {
                path: file1_path.clone().into(),
                size: file1_path.metadata().unwrap().len()
            }
        );
        assert_eq!(
            event_2,
            FileEvent::FileFound {
                path: file2_path.clone().into(),
                size: file2_path.metadata().unwrap().len()
            }
        );

        updates.next().await;

        let total_size = monitor.get_directory_size(dir.path()).await;
        assert_eq!(total_size, actual_total_size);
    }
}
