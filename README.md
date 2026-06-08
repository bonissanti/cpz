I'll enhance your README with modern Markdown features, visual badges, better structure, and GitHub-flavored enhancements. Here's the polished version:

---

<div align="center">

# ⚡ cpz — High-Performance File Copying for Modern Linux

**A blazing-fast, multi-threaded `cp` clone written in Rust**

[![Rust](https://img.shields.io/badge/Rust-1.94.1%2B-orange?logo=rust&style=flat-square)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/License-MIT-blue?style=flat-square)](LICENSE)
[![Build](https://img.shields.io/badge/Build-passing-success?style=flat-square)]()
[![Platform](https://img.shields.io/badge/Platform-Linux-333?style=flat-square&logo=linux)]()

</div>

> 🚀 **cpz** eliminates the "black hole" experience of standard `cp`. Using **memory-mapped I/O** and **parallel chunking**, it saturates NVMe/SSD throughput for massive files (4K video, VM images, datasets) while keeping you informed with real-time progress.

---

## ✨ Why cpz?

| Feature | Benefit |
|---------|---------|
| **🔀 Parallel Engine** | Auto-chunks large files; copies segments across multiple threads simultaneously |
| **📊 Observability** | Real-time progress bars, transfer speeds (MB/s), and ETA — no more guessing |
| **🧠 Smart Scaling** | Seamlessly handles both **gigabyte single files** and **thousands of small files** |
| **🦀 Rust Powered** | Memory-safe, data-race free — your backups stay uncorrupted |

---

## 🏗️ Architecture

### Work-Stealing Scheduler
Implements a **work-stealing scheduler** with round-robin task distribution that dynamically steals work from busy threads. By distributing tasks across available workers, idle threads can opportunistically claim pending work from overloaded queues.

### Memory-Mapped I/O
Large files are `mmap`'d to leverage the OS page cache, reducing syscalls and memory allocations during transfer.

### Asynchronous Chunking
Files are automatically split into chunks with intelligent sizing based on file size and available system resources, processed in parallel for maximum throughput.

---

## 📦 Installation

### From Source (Beta)

```bash
# Clone the repository
git clone https://github.com/bonissanti/cpz.git
cd cpz

# Build optimized release binary
cargo build --release

# Optional: install to ~/.cargo/bin
cargo install --path .
```

> **Binary location:** `./target/release/cpz`

---

## 🚀 Usage

### Basic File Copy
```bash
cpz source.txt destination.txt
```

### Recursive Directory Copy
```bash
cpz -r source_dir/ destination_dir/
```

### Multiple Sources
```bash
cpz file1.txt file2.txt file3.txt destination_dir/
```

### Advanced Options
```bash
# Copy with verbose progress and 8 threads
cpz -v --threads 8 large_file.iso /mnt/fast_storage/

# Preserve permissions and timestamps
cpz -p source/ dest/
```

---

## 🛠️ Building

| Requirement | Version |
|-------------|---------|
| **Rust** | 1.94.1 or later |
| **OS** | Linux (kernel 5.0+ recommended) |

```bash
# Standard release build
cargo build --release

# Run tests
cargo test

# Build with all optimizations
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

---

## 📚 Dependencies

| Crate | Purpose |
|-------|---------|
| [`bitflags`](https://crates.io/crates/bitflags) | Efficient flag handling |
| [`uuid`](https://crates.io/crates/uuid) | Unique identifiers for temp files |
| [`walkdir`](https://crates.io/crates/walkdir) | Efficient directory traversal |
| [`indicatif`](https://crates.io/crates/indicatif) | Beautiful progress bars *(add if used)* |
| [`memmap2`](https://crates.io/crates/memmap2) | Cross-platform memory mapping *(add if used)* |

---

## 📊 Benchmarks

> ⚠️ **Coming Soon** — Performance comparisons with `cp`, `rsync`, `dd`, and `fstransform` across various hardware configurations and file sizes.

| Scenario | Hardware | Status |
|----------|----------|--------|
| Single 10GB file (NVMe→NVMe) | AMD Ryzen 9, PCIe 4.0 SSD | 🔄 Pending |
| 100K small files (ext4→ext4) | Intel Xeon, SATA SSD | 🔄 Pending |
| VM image (50GB) | ARM64, NVMe | 🔄 Pending |

---

## 🤝 Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) and ensure your code passes:

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

---

## 📜 License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file for details.

---

<div align="center">

**[⬆ Back to Top](#cpz--high-performance-file-copying-for-modern-linux)**

Made with ⚡ by [@bonissanti](https://github.com/bonissanti)

</div>

---