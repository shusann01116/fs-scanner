use std::path::PathBuf;

use clap::Parser;
use dulib::default::{Aggregator, WatchConfig, Watcher};
use dulib::{Monitor, MonitorEvent};
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

    let mut monitor = Monitor::new()
        .watch(Watcher::new(WatchConfig::default()))
        .aggregate(Aggregator::new());

    {
        let mut updates = monitor.start().expect("Failed to start monitoring");
        // Updated loop to handle all rElevant events
        while let Some(update) = updates.next().await {
            match update {
                MonitorEvent::DirectoryFound(path) => {
                    println!("Directory found: {}", path.display())
                }
                MonitorEvent::FileFound(path) => println!("File found: {}", path.display()),
            }
        }
    }

    let total_size = monitor
        .get_directory_size(&args.path)
        .await
        .expect("failed to get directory size");
    println!(
        "Total size of '{}': {} bytes ({:.2} GiB)",
        args.path.display(),
        total_size,
        total_size as f64 / 1024.0 / 1024.0 / 1024.0
    );
}
