#![deny(unsafe_code)]

mod aggregator;
mod error;
mod monitor;
mod result;
mod scanner;

#[cfg(test)]
mod test_tools;

pub use error::ScannerError;
pub use monitor::Monitor;
// TODO: Replace with DiskUsageMonitor's Event (not implemented yet)
pub use scanner::FileEvent;
