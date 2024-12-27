use std::path::PathBuf;

use clap::Parser;
use disk_usage_monitor::DiskUsageMonitor;

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

    // 初期サイズを表示
    let initial_size = monitor.get_directory_size(&args.path).await;
    println!(
        "Initial total size of '{}': {} bytes",
        args.path.display(),
        initial_size
    );

    if args.watch {
        println!("Watching for changes...");
        let mut updates = monitor.start().expect("Failed to start monitoring");

        while let Some(update) = updates.recv().await {
            let update = update;
            println!(
                "Change detected at '{}': new total size = {} bytes",
                update.path.display(),
                update.total_size
            );
        }
    }
}
