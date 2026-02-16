use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ss")]
#[command(about = "Sonic-Search: High-performance cross-platform CLI search tool", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Index a directory for searching
    Scan {
        /// Directory path to scan
        path: String,
    },
    /// Find files by name
    Find {
        /// Search query
        query: String,
    },
    /// Show index statistics
    Stats,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path } => {
            println!("🔍 Scanning directory: {}", path);
            scan_directory(&path)?;
        }
        Commands::Find { query } => {
            println!("🔎 Searching for: {}", query);
            find_files(&query)?;
        }
        Commands::Stats => {
            println!("📊 Index Statistics");
            show_stats()?;
        }
    }

    Ok(())
}

fn scan_directory(path: &str) -> Result<()> {
    use ignore::WalkBuilder;
    use std::sync::atomic::{AtomicUsize, Ordering};

    let file_count = AtomicUsize::new(0);
    let dir_count = AtomicUsize::new(0);

    println!("Starting parallel scan...");

    WalkBuilder::new(path)
        .hidden(true) // Respect hidden files by default
        .git_ignore(true) // Respect .gitignore
        .build_parallel()
        .run(|| {
            Box::new(|entry| {
                if let Ok(entry) = entry {
                    if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                        file_count.fetch_add(1, Ordering::Relaxed);
                    } else if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                        dir_count.fetch_add(1, Ordering::Relaxed);
                    }
                }
                ignore::WalkState::Continue
            })
        });

    let files = file_count.load(Ordering::Relaxed);
    let dirs = dir_count.load(Ordering::Relaxed);

    println!("✅ Scan complete!");
    println!("   Files found: {}", files);
    println!("   Directories: {}", dirs);

    Ok(())
}

fn find_files(query: &str) -> Result<()> {
    println!("⚠️  File search not yet implemented");
    println!("   Query: {}", query);
    println!("   This feature will be added in the next iteration");
    Ok(())
}

fn show_stats() -> Result<()> {
    println!("⚠️  Statistics not yet implemented");
    println!("   This feature will be added once indexing is complete");
    Ok(())
}
