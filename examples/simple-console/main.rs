use std::path::PathBuf;

use clap::Parser;
use disk_usage_monitor::{FileEvent, Monitor};
use futures_util::StreamExt;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory to analyze
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Watch for changes
    #[arg(short, long, default_value_t = true)]
    watch: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let mut monitor = Monitor::new().with_directory(&args.path);
    if args.watch {
        monitor = monitor.watch_changes();
    }

    let mut updates = monitor.start().expect("Failed to start monitoring");

    // Updated loop to handle all rElevant events
    while let Some(update) = updates.next().await {
        match &update {
            FileEvent::InitialScanComplete => {
                println!("Initial scan complete");
                if !args.watch {
                    break;
                }
            }
            FileEvent::FileFound { path, size } => {
                println!("File found: '{}': size = {} bytes", path.display(), size);
            }
            FileEvent::FileAdded { path, size } => {
                println!("File added: '{}': size = {} bytes", path.display(), size);
            }
            FileEvent::FileRemoved { path } => {
                println!("File removed: '{}'", path.display());
            }
            FileEvent::FileModified { path, size } => {
                println!(
                    "File modified: '{}': new size = {} bytes",
                    path.display(),
                    size
                );
            }
        }
    }

    let total_size = monitor.get_directory_size(&args.path).await;
    println!(
        "Total size of '{}': {} bytes ({:.2} GiB)",
        args.path.display(),
        total_size,
        total_size as f64 / 1024.0 / 1024.0 / 1024.0
    );
}
