This document is designed to be the "Master Instructions" for your AI agent. It establishes the technical constraints, the roadmap, and the cross-platform standards required to build **Sonic-Search**.

---

# Project Blueprint: Sonic-Search (Rust)

## 1. Vision Statement

**Sonic-Search** is a high-performance, cross-platform CLI tool designed to replace slow OS-native search. It leverages Rust’s concurrency and memory safety to provide instant filename, content, and semantic search results across Windows, macOS, and Linux.

---

## 2. Technical Core Constraints

To ensure the project remains scalable and professional, the Agent must adhere to these rules:

* **Language:** Rust (Stable)
* **Path Handling:** Use `std::path::PathBuf` exclusively. Use the `dunce` crate to normalize Windows UNC paths for cross-platform display.
* **Concurrency:** Use `rayon` for data-parallelism (scanning) and `tokio` for any asynchronous I/O or background indexing.
* **Error Handling:** Never `panic!`. Use `anyhow` for application-level errors and `thiserror` for library-level errors. Gracefully skip "Permission Denied" directories.
* **Data Integrity:** Use `serde` for all configuration and index metadata serialization.

---

## 3. Evolutionary Roadmap (Daily Progress Plan)

### **Phase 1: The Foundation (Filesystem)**

* **Goal:** Instant recursive filename search.
* **Key Tasks:** * Implement parallel directory walking using `ignore` or `jwalk`.
* Respect `.gitignore` and hidden file rules by default.
* Create a CLI interface using `clap` with commands: `scan`, `find`, and `stats`.



### **Phase 2: The Librarian (Inverted Indexing)**

* **Goal:** Search inside file contents (Text, PDF, Docx).
* **Key Tasks:**
* Integrate `tantivy` as the core search engine library.
* Implement a background "Watcher" (using `notify` crate) to update the index when files change.
* Add text extraction for common document formats using `extractous`.



### **Phase 3: The Brain (Semantic Search)**

* **Goal:** Search by "meaning" using local AI.
* **Key Tasks:**
* Implement local vector embedding generation using `ort` (ONNX Runtime).
* Store vectors in a local store to allow "Smart Search" (e.g., searching "banking" finds "invoice.pdf").
* Ensure the AI models are small enough to run on standard consumer CPUs without a GPU.



### **Phase 4: The Interface (TUI)**

* **Goal:** A professional terminal dashboard.
* **Key Tasks:**
* Build a full-screen interactive interface using `ratatui`.
* Implement a "Live Preview" pane to see file snippets while scrolling.
* Add keyboard shortcuts for opening files in the default OS handler.



---

## 4. GitHub Actions Configuration (CI/CD)

The agent must ensure the code passes this build matrix on every commit.

```yaml
name: Sonic-Search CI
on: [push]

jobs:
  test:
    name: Build & Test on ${{ matrix.os }}
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    steps:
      - uses: actions/checkout@v4
      - name: Rust Toolchain
        uses: dtolnay/rust-toolchain@stable
      - name: Cache Dependencies
        uses: Swatinem/rust-cache@v2
      - name: Run Tests
        run: cargo test --verbose
      - name: Check Formatting
        run: cargo fmt --all -- --check

```

---

## 5. Target User Experience (CLI Examples)

| Command | Action |
| --- | --- |
| `ss scan C:\Users` | Indexes the directory for the first time. |
| `ss find "budget"` | Instant filename search. |
| `ss grep "target_profit"` | Searches inside file contents (like ripgrep but indexed). |
| `ss smart "travel plans"` | Semantic search (finds hotel receipts, flight PDFs). |
| `ss ui` | Launches the interactive TUI dashboard. |

---

## 6. Guidelines for the AI Agent

1. **Commit Granularity:** One feature per day. Commits must include updated documentation and unit tests.
2. **Performance First:** Every feature should be benchmarked. If a change slows down the search, it must be optimized.
3. **Clean Code:** Follow `clippy` suggestions strictly. Use `cargo clippy -- -D warnings` in the CI.
