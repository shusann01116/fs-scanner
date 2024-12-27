use std::path::Path;

use tokio::sync::mpsc;

use crate::{
    aggregator::{Aggregator, DirectoryUpdate},
    result::Result,
    scanner::Scanner,
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
    pub fn start(&self) -> Result<mpsc::UnboundedReceiver<DirectoryUpdate>> {
        let stream = self.scanner.start()?;
        Ok(self.aggregator.process_events(stream))
    }

    /// Returns the total size of all files under the specified directory
    pub async fn get_directory_size<P: AsRef<Path>>(&self, directory: P) -> u64 {
        self.aggregator.get_directory_size(directory).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_tools;

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
        while let Some(update) = updates.recv().await {
            println!("Update received: {:?}", update);
        }

        let total_size = monitor.get_directory_size(dir.path()).await;
        assert_eq!(total_size, actual_total_size);
    }
}
