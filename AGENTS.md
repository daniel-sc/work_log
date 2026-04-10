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
sudo apt-get install -y libx11-dev libxi-dev libxtst-dev libevdev-dev libdbus-1-dev pkg-config \
  libinput-dev libxkbcommon-dev libudev-dev \
  libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxcb-shm0-dev
```

## Linux Runtime Requirements

- **Mouse tracking (Wayland):** User must be in the `input` group: `sudo usermod -aG input $USER` (log out/in after)
- **Active window (GNOME Wayland):** Requires the x-win GNOME Shell extension. The app auto-installs it on first run; user must then log out/in and enable it with `gnome-extensions enable x-win@miniben90.org`

## Structure

- `src/main.rs` - CLI entry point, argument parsing, main loop
- `src/get_state.rs` - Reads active window via `x-win` and cursor via `rdev`
- `src/write_csv.rs` - Appends state to CSV file

## CI

- `rust.yml` - Builds and tests on Linux, macOS, Windows on push/PR to `master`
- `release.yml` - Creates GitHub release with binaries when tags matching `v[0-9]+.*` are pushed
