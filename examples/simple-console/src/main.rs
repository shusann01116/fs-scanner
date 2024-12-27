use std::path::PathBuf;

use clap::Parser;
use disk_usage_monitor::{DiskUsageMonitor, FileEvent};
use futures_util::StreamExt;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory to analyze
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Watch for changes
    #[arg(short, long)]
    watch: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let mut monitor = DiskUsageMonitor::new().with_directory(&args.path);
    if args.watch {
        monitor = monitor.watch_changes();
    }

    let mut updates = monitor.start().expect("Failed to start monitoring");

    while let Some(update) = updates.next().await {
        match &update {
            FileEvent::InitialScanComplete => {
                println!("Initial scan complete");
                if !args.watch {
                    break;
                }
            }
            FileEvent::FileFound { path, size } => {
                println!(
                    "Change detected at '{}': new total size = {} bytes",
                    path.display(),
                    size
                );
            }
            _ => {}
        }
    }

    let total_size = monitor.get_directory_size(&args.path).await;
    println!(
        "Total size of '{}': {} bytes",
        args.path.display(),
        total_size
    );
}
