# How to Install Sentinel

This guide explains how to install Sentinel into a new or existing Rust project.

## Option 1 — Use the Sentinel Installer (Recommended)

1. Download `Sentinelinstaller.rs` from the repository.
2. Place it inside your project folder.
3. Run:

   ```bash
   rustc Sentinelinstaller.rs
   ./Sentinelinstaller
   ```
The installer will:

  - Download the official Sentinel `Cargo.toml`
  - Download the entire `src/` directory from the Sentinel repository
  - Install everything into your project folder
After installation:
```bash
cargo build
cargo run
```
---
