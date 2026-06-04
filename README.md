# nanofetch

A lightweight, blazing fast, and **zero-dependency** system information fetching tool written in Rust for Linux. 

Unlike other tools that rely on heavy external crates, `nanofetch` reads system statistics directly from the Linux virtual filesystem (`/proc`), making it incredibly small, efficient, and educational!

## ✨ Features

- 🚀 **Zero Dependencies**: Pure Rust standard library (`std::fs`, `std::thread`, etc.).
- 🧠 **Memory Insights**: Reads and parses `/proc/meminfo` directly.
- 🐧 **Linux Native**: Tailored specifically for the Linux architecture.

## 🚀 Getting Started

### Prerequisites

Make sure you have Rust and Cargo installed on your Linux machine.
```bash
# To check if you have Rust installed
rustc --version
```

### Installation & Running
Clone this repository and run it using Cargo:
```bash
# Clone the repository
git clone https://github.com/andreananda1022/nanofetch.git

# Navigate into the directory
cd nanofetch

# Run the project
cargo run --release
```
