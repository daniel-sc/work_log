# AGENTS.md

## Overview

Simple Rust CLI that logs the active window (app name + title) and cursor position to CSV when they change. Cross-platform: Linux, macOS, Windows.

## Commands

```bash
cargo build          # Build
cargo test           # Run tests
cargo run -- --help  # Show CLI help
```

## Linux Build Requirement

Requires development libraries on Linux:
```bash
sudo apt-get install -y libx11-dev libxi-dev libxtst-dev libevdev-dev libdbus-1-dev pkg-config
```

## Structure

- `src/main.rs` - CLI entry point, argument parsing, main loop
- `src/get_state.rs` - Reads active window via `active-win-pos-rs` and cursor via `rdev`
- `src/write_csv.rs` - Appends state to CSV file

## CI

- `rust.yml` - Builds and tests on Linux, macOS, Windows on push/PR to `master`
- `release.yml` - Creates GitHub release with binaries when tags matching `v[0-9]+.*` are pushed
