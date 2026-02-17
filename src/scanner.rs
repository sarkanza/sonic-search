use anyhow::Result;
use ignore::WalkBuilder;
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

/// Result of a directory scan operation
#[derive(Debug, Clone)]
pub struct ScanResult {
    pub root: PathBuf,
    pub file_count: usize,
    pub dir_count: usize,
    pub total_size: u64,
    pub elapsed_ms: u128,
    pub files: Vec<FileEntry>,
}

/// A single file entry discovered during scanning
#[derive(Debug, Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    pub name: String,
    pub size: u64,
    pub is_dir: bool,
}

/// Scan a directory and collect all file entries
pub fn scan_directory(path: &str) -> Result<ScanResult> {
    let root = Path::new(path).to_path_buf();
    if !root.exists() {
        anyhow::bail!("Path does not exist: {}", path);
    }
    if !root.is_dir() {
        anyhow::bail!("Path is not a directory: {}", path);
    }

    let start = Instant::now();
    let file_count = AtomicUsize::new(0);
    let dir_count = AtomicUsize::new(0);
    let total_size = std::sync::atomic::AtomicU64::new(0);
    let files: Mutex<Vec<FileEntry>> = Mutex::new(Vec::new());

    WalkBuilder::new(path)
        .hidden(true)
        .git_ignore(true)
        .build_parallel()
        .run(|| {
            let file_count = &file_count;
            let dir_count = &dir_count;
            let total_size = &total_size;
            let files = &files;
            Box::new(move |entry| {
                if let Ok(entry) = entry {
                    let is_file = entry.file_type().map(|ft| ft.is_file()).unwrap_or(false);
                    let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);

                    if is_file {
                        file_count.fetch_add(1, Ordering::Relaxed);
                        let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
                        total_size.fetch_add(size, Ordering::Relaxed);

                        let name = entry
                            .file_name()
                            .to_string_lossy()
                            .to_string();

                        let file_entry = FileEntry {
                            path: entry.path().to_path_buf(),
                            name,
                            size,
                            is_dir: false,
                        };

                        if let Ok(mut guard) = files.lock() {
                            guard.push(file_entry);
                        }
                    } else if is_dir {
                        dir_count.fetch_add(1, Ordering::Relaxed);
                    }
                }
                ignore::WalkState::Continue
            })
        });

    let elapsed = start.elapsed().as_millis();

    Ok(ScanResult {
        root,
        file_count: file_count.load(Ordering::Relaxed),
        dir_count: dir_count.load(Ordering::Relaxed),
        total_size: total_size.load(Ordering::Relaxed),
        elapsed_ms: elapsed,
        files: files.into_inner().unwrap_or_default(),
    })
}

/// Format bytes into human-readable size
pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;
    const GB: u64 = 1024 * MB;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_scan_nonexistent_path() {
        let result = scan_directory("/nonexistent/path/12345");
        assert!(result.is_err());
    }

    #[test]
    fn test_scan_current_dir() {
        let result = scan_directory(".");
        assert!(result.is_ok());
        let scan = result.unwrap();
        assert!(scan.file_count > 0, "Should find at least one file");
    }

    #[test]
    fn test_scan_temp_dir() {
        let dir = std::env::temp_dir().join("sonic_search_test_scan");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("test.txt"), "hello").unwrap();
        fs::write(dir.join("test2.rs"), "fn main() {}").unwrap();
        fs::create_dir_all(dir.join("subdir")).unwrap();
        fs::write(dir.join("subdir/nested.txt"), "nested").unwrap();

        let result = scan_directory(dir.to_str().unwrap());
        assert!(result.is_ok());
        let scan = result.unwrap();
        assert_eq!(scan.file_count, 3);
        assert!(scan.dir_count >= 2); // root dir + subdir
        assert!(scan.total_size > 0);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(0), "0 B");
        assert_eq!(format_size(512), "512 B");
        assert_eq!(format_size(1024), "1.00 KB");
        assert_eq!(format_size(1536), "1.50 KB");
        assert_eq!(format_size(1048576), "1.00 MB");
        assert_eq!(format_size(1073741824), "1.00 GB");
    }

    #[test]
    fn test_scan_file_entries_populated() {
        let dir = std::env::temp_dir().join("sonic_search_test_entries");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("alpha.txt"), "aaa").unwrap();
        fs::write(dir.join("beta.rs"), "bbb").unwrap();

        let result = scan_directory(dir.to_str().unwrap()).unwrap();
        assert_eq!(result.files.len(), 2);

        let names: Vec<&str> = result.files.iter().map(|f| f.name.as_str()).collect();
        assert!(names.contains(&"alpha.txt"));
        assert!(names.contains(&"beta.rs"));

        let _ = fs::remove_dir_all(&dir);
    }
}
