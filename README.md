# nanofetch

A lightweight, blazing fast, and **zero-dependency** system information fetching tool written in Rust for Linux. 

Unlike other tools that rely on heavy external crates, `nanofetch` reads system statistics directly from the Linux virtual filesystem (`/proc`), making it incredibly small, efficient, and educational!

## ✨ Features

- 🚀 **Zero Dependencies**: Pure Rust standard library (`std::fs`, `std::env`, `std::process`, etc.).
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

## 🐧 Installation & Running Globally (Linux Only)

If you have downloaded the pre-compiled binary file of **Nanofetch**, you can install it globally so that it can be run from anywhere in your terminal using just the `nanofetch` command.

Follow these simple steps:

### 1. Make the Binary Executable
Open your terminal in the directory where you downloaded the `nanofetch` binary, then run the following command to grant execution permissions:
```bash
chmod +x nanofetch
```

### 2. Move to a System PATH Directory
Move the binary to /usr/local/bin/ so the system can recognize it as a global command. This requires administrator (sudo) privileges:
```bash
sudo mv nanofetch /usr/local/bin/
```

### Run the Application
Open a new terminal window or tab, and simply type:
```bash
nanofetch
```
