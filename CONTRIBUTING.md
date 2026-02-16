# Contributing to Sonic-Search

Thank you for your interest in contributing to Sonic-Search! This document provides guidelines and information for contributors.

## 🚀 Getting Started

### Prerequisites

- **Rust:** Install via [rustup](https://rustup.rs/)
- **Git:** For version control
- **GitHub CLI (optional):** `gh` for managing issues and PRs

### Setting Up Your Development Environment

```bash
# Clone the repository
git clone https://github.com/YOUR_USERNAME/sonic-search.git
cd sonic-search

# Build the project
cargo build

# Run tests
cargo test

# Run the application
cargo run -- scan ~/Documents
```

## 📋 Development Workflow

### 1. Find or Create an Issue

- Check [existing issues](https://github.com/YOUR_USERNAME/sonic-search/issues)
- Comment on issues you'd like to work on
- Create new issues for bugs or feature requests

### 2. Create a Feature Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/bug-description
```

### 3. Make Your Changes

Follow the technical constraints outlined in [docs/planning/raw.md](docs/planning/raw.md):

- **Use `PathBuf` for all path operations**
- **Never `panic!`** - use `anyhow` or `thiserror`
- **Use `rayon` for parallelism** and `tokio` for async I/O
- **Follow Rust idioms** and best practices

### 4. Test Your Changes

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Check formatting
cargo fmt --all -- --check

# Run linter
cargo clippy -- -D warnings
```

### 5. Commit Your Changes

Write clear, descriptive commit messages:

```bash
git commit -m "feat: add filename search caching"
git commit -m "fix: handle permission denied errors gracefully"
git commit -m "docs: update README with new commands"
```

#### Commit Message Format

- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation changes
- `test:` Adding or updating tests
- `refactor:` Code refactoring
- `perf:` Performance improvements
- `chore:` Maintenance tasks

### 6. Push and Create a Pull Request

```bash
git push origin feature/your-feature-name
```

Then create a PR on GitHub with:
- Clear description of changes
- Link to related issues
- Screenshots/examples if applicable

## 🎯 Development Phases

Sonic-Search follows an evolutionary roadmap:

### Phase 1: The Foundation *(Current)*
- ✅ Parallel directory walking
- ✅ Respect `.gitignore` and hidden files
- 🚧 Instant filename search
- 🚧 CLI interface (`scan`, `find`, `stats`)

### Phase 2: The Librarian *(Planned)*
- Content indexing with Tantivy
- File watching with `notify`
- Text extraction from documents

### Phase 3: The Brain *(Planned)*
- Local vector embeddings
- Semantic search capabilities
- ONNX Runtime integration

### Phase 4: The Interface *(Planned)*
- Interactive TUI with `ratatui`
- Live preview pane
- Keyboard shortcuts

## 🧪 Testing Guidelines

### Unit Tests

Write unit tests for individual functions:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_empty_directory() {
        // Test implementation
    }
}
```

### Integration Tests

Place integration tests in `tests/` directory:

```rust
// tests/integration_test.rs
use sonic_search::*;

#[test]
fn test_full_scan_workflow() {
    // Test implementation
}
```

### Performance Benchmarks

Use `criterion` for benchmarks (add to `Cargo.toml` when needed):

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_scan(c: &mut Criterion) {
    c.bench_function("scan 1000 files", |b| {
        b.iter(|| {
            // Benchmark code
        });
    });
}
```

## 📐 Code Style

### Formatting

```bash
# Auto-format code
cargo fmt

# Check formatting without changing files
cargo fmt --all -- --check
```

### Linting

```bash
# Run Clippy with strict warnings
cargo clippy -- -D warnings

# Fix auto-fixable issues
cargo clippy --fix
```

### Documentation

- Add doc comments for public APIs:

```rust
/// Scans a directory and returns file count
///
/// # Arguments
/// * `path` - The directory path to scan
///
/// # Returns
/// * `Result<usize>` - Number of files found or error
pub fn scan_directory(path: &str) -> Result<usize> {
    // Implementation
}
```

## 🐛 Reporting Bugs

When reporting bugs, include:

1. **Description:** Clear description of the issue
2. **Steps to Reproduce:** Minimal steps to reproduce
3. **Expected Behavior:** What should happen
4. **Actual Behavior:** What actually happens
5. **Environment:**
   - OS (Windows/macOS/Linux)
   - Rust version (`rustc --version`)
   - Sonic-Search version

## 💡 Feature Requests

When proposing features:

1. Check if it aligns with project goals
2. Describe the use case
3. Provide examples of expected behavior
4. Consider performance implications

## 🔍 Code Review Process

All submissions require review. We use GitHub pull requests:

1. **Automated Checks:** CI must pass (builds, tests, clippy, fmt)
2. **Code Review:** At least one maintainer approval
3. **Documentation:** Update README/docs if needed
4. **Tests:** Add tests for new functionality

## 📚 Additional Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/)
- [Project Blueprint](docs/planning/raw.md)

## 🤝 Community

- **Issues:** Bug reports and feature requests
- **Discussions:** Q&A and general discussions
- **Discord/Slack:** *(Add if available)*

## 📜 License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

**Thank you for contributing to Sonic-Search!** 🎉
