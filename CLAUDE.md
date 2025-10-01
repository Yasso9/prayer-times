# Project Overview

`prayer-times` is a Rust CLI application that provides Islamic prayer times notifications based on geographical location. It calculates prayer times using the algorithm from praytimes.org and can run as a daemon to send desktop notifications.

# Architecture

## Core Components

**Main Entry Point** (`src/main.rs`):
- Parses CLI arguments using `clap`
- Dispatches to appropriate command handlers
- Manages application lifecycle

**Command System** (`src/arguments.rs`):
- Defines CLI interface with clap derive macros
- Supports subcommands: daemon, current, next, list-prayers, list-methods, etc.
- Handles coordinate inputs, calculation methods, and notification settings

**Prayer Calculation Engine**:
- `src/calculations.rs` - Core prayer time calculations
- `src/method.rs` - Different calculation methods (MuslimWorldLeague, Egyptian, etc.)
- `src/madhab.rs` - Madhab-specific calculations (Shafi, Hanafi)
- `src/prayers.rs` - Prayer time management and utilities

**Location & Configuration**:
- `src/location.rs` - Geolocation from IP if coordinates not provided
- `src/config.rs` - Configuration file handling (TOML format)
- Uses `confy` crate for config file management

**Daemon & Notifications**:
- `src/daemon.rs` - Background process that monitors prayer times
- `src/notification.rs` - Desktop notifications using `notify-rust`
- `src/event.rs` - Event handling for prayer timing
- Daemon checks every configurable interval (default 20 seconds)

## Key Dependencies
- `chrono` - Date/time handling
- `clap` - CLI argument parsing
- `notify-rust` - Desktop notifications
- `geolocation` - IP-based location detection
- `confy` - Configuration file management
- `serde` - Serialization for config

# Configuration

The application uses a TOML configuration file located at `$XDG_CONFIG_HOME/prayer-times/config.toml`. Key configuration sections:
- `[prayer]` - Calculation method, madhab, time adjustments
- `[notification]` - Notification settings, urgency, intervals

CLI arguments always take precedence over configuration file settings.

# Package Distribution

The project includes:
- `PKGBUILD` - Arch Linux package configuration
- Shell completion generation via `generate-shell` command
- Icon assets in `assets/` directory