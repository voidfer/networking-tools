# rust-port-scanner

A network scanning tool written in Rust, inspired by Nmap.

This project combines:

* Concurrent TCP and UDP port scanning
* Basic ICMP host discovery
* Asynchronous execution using Tokio
* Clean modular architecture for learning systems programming

---

## 🎯 Goals

This project is designed as a **learning tool** to demonstrate:

* How to build an asynchronous port scanner in Rust
* How to handle multiple scan types (TCP, UDP, ICMP)
* How to design a modular Rust project (CLI → config → engine → workers)
* How concurrency works using `tokio` and `Semaphore`

---

## ⚡ Features

### Port Scanning

* Scan a target (IP or domain)
* Supported scan types:

  * `tcp` — TCP connect scan
  * `udp` — basic UDP probe scan
  * `icmp` — host reachability (ping-based)
* Configurable:

  * Port range (`-p`, `--ports`)
  * Concurrency (`-c`, `--concurrency`)

---

## 📦 Project Structure

```
src/
├── main.rs          # Entry point
├── cli.rs           # CLI parsing (clap)
├── config.rs        # Configuration parsing
├── result.rs        # Scan result types
└── scanner/
    ├── mod.rs
    ├── engine.rs    # Task orchestration
    ├── tcp.rs       # TCP scan logic
    ├── udp.rs       # UDP scan logic
    └── icmp.rs      # ICMP host check
```

---

## 🔧 Prerequisites

* Rust (latest stable)
* Cargo

Install Rust:

```
curl https://sh.rustup.rs -sSf | sh
```

---

## 🛠️ Build

Debug build:

```
cargo build
```

Release build (recommended):

```
cargo build --release
```

Binary:

```
target/release/port_scanner
```

---

## 🚀 Usage

### Basic scan

```
cargo run -- 127.0.0.1
```

---

### Scan specific ports

```
cargo run -- 127.0.0.1 -p 20-100
```

---

### UDP scan

```
cargo run -- 127.0.0.1 -p 1-100 -t udp
```

---

### ICMP (host check)

```
cargo run -- 127.0.0.1 -t icmp
```

---

### Increase concurrency

```
cargo run -- 127.0.0.1 -c 200
```

---

## 🧠 How It Works

### 1. CLI → Config

User input is parsed using `clap` and converted into a structured `Config`.

---

### 2. Engine

The scan engine:

* Iterates over ports
* Uses a `Semaphore` to limit concurrency
* Spawns async tasks (`tokio::spawn`)

---

### 3. TCP Scan

* Attempts a connection using `TcpStream::connect`
* Results:

  * Success → `Open`
  * Error → `Closed`
  * Timeout → `Filtered`

---

### 4. UDP Scan

* Sends a datagram
* Waits for response
* Results:

  * Response → `Open`
  * No response → `Filtered`

⚠️ UDP is inherently unreliable:

* No response ≠ closed
* Often interpreted as *open|filtered*

---

### 5. ICMP Scan

* Uses system `ping`
* Determines if host is reachable

---

## ⚠️ Limitations

* No SYN scan (requires raw sockets)
* UDP detection is simplified
* ICMP uses system command (not raw packets)
* No service detection or fingerprinting

---

## 🔐 Responsible Use

This tool is intended for:

* Learning
* Lab environments
* Authorized network auditing

❗ Do NOT scan systems without permission.

---

## 🚀 Future Improvements

* Raw socket implementation (true ICMP + UDP detection)
* SYN scan support
* Banner grabbing
* JSON output
* Traceroute (pure Rust implementation)
* Service fingerprinting

---

## 👤 Author

Built as a systems programming and networking learning project in Rust.

