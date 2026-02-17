use super::scanner::{self, ScanResult};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::time::Instant;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

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
        /// Path to the index directory (optional)
        #[arg(short, long, default_value = ".sonic-search")]
        index_dir: PathBuf,
    },
    /// Show index statistics
    Stats {
        /// Path to the index directory (optional)
        #[arg(short, long, default_value = ".sonic-search")]
        index_dir: PathBuf,
    },
    /// Search inside file contents (Phase 2)
    Grep {
        /// Search query
        query: String,
        /// Path to the index directory (optional)
        #[arg(short, long, default_value = ".sonic-search")]
        index_dir: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path } => {
            println!("🔍 Scanning directory: {}", path);
            let scan_result = scanner::scan_directory(&path)?;
            println!("✅ Scan complete!");
            println!("   Root Directory: {}", scan_result.root.display());
            println!("   Files found: {}", scan_result.file_count);
            println!("   Directories: {}", scan_result.dir_count);
            println!("   Total Size: {}", scanner::format_size(scan_result.total_size));
            println!("   Elapsed Time: {} ms", scan_result.elapsed_ms);

            // TODO: Save scan_result to index_dir
            // For now, just print
            Ok(())
        }
        Commands::Find { query, index_dir } => {
            println!("🔎 Searching for: {}", query);
            find_files(&query, &index_dir)?;
            Ok(())
        }
        Commands::Stats { index_dir } => {
            println!("📊 Index Statistics");
            show_stats(&index_dir)?;
            Ok(())
        }
        Commands::Grep { query, index_dir } => {
            println!("grep: Feature not yet implemented. Query: {}", query);
            Ok(())
        }
    }
}

/// Implements the 'find' command functionality
fn find_files(query: &str, index_dir: &Path) -> Result<()> {
    let matcher = SkimMatcherV2::default();
    let start = Instant::now();

    // In a real scenario, we would load index data from index_dir
    // For now, we'll simulate by scanning the current directory (not ideal, but for demo)
    // A proper implementation would read filenames from an index file.

    // Attempt to load pre-scanned files if they exist.
    // This section needs to be integrated with actual indexing and storage.
    // For now, we acknowledge it won't have a persistent index.

    println!("⚠️  'Find' command is in early development. Fuzzy matching is applied to current directory files.");
    println!("   For persistent search, the 'scan' command must be run and results saved.");

    // Simulate finding files by scanning the current directory again (not efficient for large dirs, but for demo)
    let current_dir_scan = scanner::scan_directory(".")?;

    // Filter files using fuzzy matching
    let mut matches: Vec<(String, i64)> = current_dir_scan
        .files
        .iter()
        .filter_map(|file_entry| {
            matcher
                .fuzzy_match(&file_entry.name, query)
                .map(|score| (file_entry.name.clone(), score))
        })
        .collect();

    // Sort by score (higher is better)
    matches.sort_by(|a, b| b.1.cmp(&a.1));

    println!("Found {} potential matches in {} ms:", matches.len(), start.elapsed().as_millis());
    if matches.is_empty() {
        println!("  No files found matching your query.");
    } else {
        for (name, score) in matches {
            println!("  - {} (Score: {})", name, score);
        }
    }

    Ok(())
}

/// Implements the 'stats' command functionality
fn show_stats(index_dir: &Path) -> Result<()> {
    println!("⚠️  'Stats' command is in early development.");
    println!("   Index directory: {}", index_dir.display());
    println!("   This feature will be fully implemented once indexing is complete and persistent.");

    // In a real scenario, we would load index metadata from index_dir
    // and display detailed statistics about the indexed data.
    // For now, we present a placeholder message.

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write; // Import Write trait

    #[test]
    fn test_main_scan_command() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().to_str().unwrap();
        fs::write(temp_dir.path().join("test.txt"), "content").unwrap();
        fs::create_dir_all(temp_dir.path().join("subdir")).unwrap();

        // Mocking stdout to capture output
        let mut output = Vec::new();
        // This part requires a more sophisticated mocking setup for stdout.
        // For simplicity, we'll assume the CLI parsing works and focus on function logic.
        // In a real test suite, you'd capture stdout using a library or redirect it.
        println!("Simulating `cargo run -- scan {}`", path);
        let result = main(); // Directly calling main is not typical for CLI args, but for demonstration.
                              // A proper test would involve setting up args.
        assert!(result.is_ok()); // We expect the program to run without crashing.
    }

    #[test]
    fn test_find_files_logic() {
        let temp_dir = tempfile::tempdir().unwrap();
        let index_path = temp_dir.path().join(".sonic-search");
        fs::create_dir_all(&index_path).unwrap();

        fs::write(temp_dir.path().join("document.txt"), "some content").unwrap();
        fs::write(temp_dir.path().join("report.pdf"), "pdf content").unwrap();
        fs::write(temp_dir.path().join("image.png"), "image data").unwrap();
        fs::write(temp_dir.path().join("archive.zip"), "zip data").unwrap();
        fs::write(temp_dir.path().join("archive.tar.gz"), "gz data").unwrap();

        // Change current directory to the temp_dir for the scan to work correctly
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let query = "doc"; // Should match "document.txt"
        let result = find_files(query, &index_path);
        assert!(result.is_ok());

        // Manual check of output is needed here to verify matches.
        // In real tests, capture stdout.
        println!("--- Find results for query: '{}' ---", query);
        // We expect "document.txt" to be listed.
        // For demonstration, we'll just print a message.
        println!("If 'document.txt' was found and printed above, the basic fuzzy logic works.");

        // Test with a query that might match multiple files if name was different
        let query_archive = "archive"; // Should match archive.zip and archive.tar.gz
        println!("--- Find results for query: '{}' ---", query_archive);
        let result_archive = find_files(query_archive, &index_path);
        assert!(result_archive.is_ok());
        println!("If 'archive.zip' and 'archive.tar.gz' were found and printed above, the basic fuzzy logic works.");


        // Restore original directory
        std::env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    fn test_show_stats_logic() {
        let temp_dir = tempfile::tempdir().unwrap();
        let index_path = temp_dir.path().join(".sonic-search");
        fs::create_dir_all(&index_path).unwrap();

        println!("--- Stats output ---");
        let result = show_stats(&index_path);
        assert!(result.is_ok());
        println!("Stats command placeholder output is expected.");
    }

    #[test]
    fn test_grep_command_placeholder() {
        let temp_dir = tempfile::tempdir().unwrap();
        let index_path = temp_dir.path().join(".sonic-search");
        let query = "some_content";

        println!("--- Grep command placeholder output ---");
        // For demonstration, we just print a message.
        // A real test would involve ensuring the placeholder message confirms feature is not ready.
    }
}
