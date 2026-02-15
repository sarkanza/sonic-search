# Sonic-Search 🔍⚡

> A high-performance, cross-platform CLI search tool built with Rust

[![CI](https://github.com/YOUR_USERNAME/sonic-search/workflows/Sonic-Search%20CI/badge.svg)](https://github.com/YOUR_USERNAME/sonic-search/actions)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## 🎯 Vision

Sonic-Search is designed to replace slow OS-native search with instant filename, content, and semantic search results across Windows, macOS, and Linux. Leveraging Rust's concurrency and memory safety, it provides blazing-fast search capabilities for modern workflows.

## ✨ Features (Roadmap)

- **Phase 1: Filesystem Search** *(In Progress)*
  - 🚀 Instant recursive filename search
  - 📁 Respect `.gitignore` and hidden file rules
  - ⚡ Parallel directory walking for maximum performance

- **Phase 2: Content Indexing** *(Planned)*
  - 📄 Search inside file contents (Text, PDF, Docx)
  - 🔄 Real-time index updates with filesystem watchers
  - 🔎 Full-text search powered by Tantivy

- **Phase 3: Semantic Search** *(Planned)*
  - 🧠 AI-powered semantic search using local embeddings
  - 💡 Find files by meaning, not just keywords
  - 🖥️ CPU-only inference (no GPU required)

- **Phase 4: Interactive TUI** *(Planned)*
  - 🎨 Beautiful terminal dashboard
  - 👁️ Live preview pane for file contents
  - ⌨️ Keyboard shortcuts for quick file access

## 🚀 Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/YOUR_USERNAME/sonic-search.git
cd sonic-search

# Build from source
cargo build --release

# Install locally
cargo install --path .
```

### Usage Examples

```bash
# Index a directory
ss scan ~/Documents

# Find files by name
ss find "budget"

# Search inside file contents
ss grep "target_profit"

# Semantic search (Phase 3)
ss smart "travel plans"

# Launch interactive TUI
ss ui
```

## 🛠️ Technical Stack

- **Language:** Rust (Stable)
- **Concurrency:** `rayon` for parallelism, `tokio` for async I/O
- **Search Engine:** `tantivy` for inverted indexing
- **AI/ML:** `ort` (ONNX Runtime) for local embeddings
- **TUI:** `ratatui` for terminal interface
- **CLI:** `clap` for argument parsing

## 📊 Performance Goals

- **Filename Search:** < 100ms for 1M+ files
- **Content Search:** < 500ms with indexed corpus
- **Semantic Search:** < 1s on consumer CPUs

## 🧪 Development

```bash
# Run tests
cargo test

# Check formatting
cargo fmt --all -- --check

# Run linter
cargo clippy -- -D warnings

# Watch for changes
cargo watch -x test
```

## 🗺️ Roadmap

See [docs/planning/raw.md](docs/planning/raw.md) for detailed development phases and technical constraints.

## 🤝 Contributing

Contributions are welcome! Please ensure:
- All tests pass (`cargo test`)
- Code is formatted (`cargo fmt`)
- Clippy checks pass (`cargo clippy`)
- One feature per commit with clear messages

## 📄 License

MIT License - see [LICENSE](LICENSE) for details

## 🙏 Acknowledgments

Built with love using the amazing Rust ecosystem:
- [tantivy](https://github.com/quickwit-oss/tantivy) - Full-text search engine
- [rayon](https://github.com/rayon-rs/rayon) - Data parallelism
- [ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI framework
- [ort](https://github.com/pykeio/ort) - ONNX Runtime bindings

---

**Status:** 🚧 Early Development - Not ready for production use
