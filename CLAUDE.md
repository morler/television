# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Television is a cross-platform, fast and extensible general purpose fuzzy finder TUI built in Rust. It integrates with shells and lets users quickly search through any kind of data source using a fuzzy matching algorithm. The project is inspired by neovim's telescope plugin.

## Common Development Commands

All development commands are managed through `just` (justfile):

- `just run` - Run the program in debug mode with logs enabled
- `just setup` - Setup project environment for local development (installs git hooks, builds project)
- `just build [profile]` - Build with specified profile (dev by default), use `just br` for release
- `just test` - Run the full test suite with output
- `just check` - Check project for errors and warnings
- `just fix` - Fix linting and formatting errors (runs cargo fix, fmt, clippy)
- `just format` - Format code using cargo fmt
- `just lint` - Lint code using cargo clippy with warnings as errors
- `just clean` - Clean up build artifacts and git hooks
- `just update-channels` - Update community channel configurations

### Testing specific functionality:
- `cargo test --test [test_name]` - Run specific test file
- `cargo test [function_name]` - Run specific test function
- `cargo run -- [channel_name]` - Run with specific channel (e.g. `cargo run -- files`)

### Development workflow:
1. `just setup` - Initial setup
2. Make changes
3. `just fix` - Auto-fix issues before commit
4. `just test` - Verify functionality
5. `just check` - Final validation before commit

## Architecture Overview

Television uses an async actor model with separate event loops:

### Core Architecture Components

1. **Application Orchestrator** (`app.rs`): Main coordinator managing lifecycle, routing messages between loops via async channels
2. **Television Core** (`television.rs`): Main state manager tracking modes, search patterns, selections, and preview state  
3. **Channel System** (`channels/`): Extensible data sources defined in TOML configs, with async command execution and fuzzy matching
4. **Event System** (`event.rs`, `action.rs`): Converts raw input events to typed actions flowing through the system
5. **Render System** (`draw.rs`, render loop): 60 FPS capped rendering with synchronized terminal updates
6. **Preview System** (`previewer/`): Async preview generation with caching and debouncing
7. **Configuration** (`config/`): Three-layer config system (defaults < user config < CLI args)

### Key Event Loops
- **Event Loop**: Handles keyboard/mouse input, converts to actions
- **Render Loop**: Updates terminal UI at 60 FPS without blocking
- **Watch Timer**: Automatically reloads channels at configured intervals

### Communication Pattern
Events → Actions → State Changes → Render (unidirectional data flow)
All components communicate via async channels to maintain responsiveness.

## Code Organization

### Main source directories:
- `television/` - Core library modules
  - `channels/` - Channel system and data sources
  - `config/` - Configuration management
  - `matcher/` - Fuzzy matching logic
  - `previewer/` - Preview system
  - `screen/` - UI components and rendering
  - `utils/` - Shared utilities
- `tests/` - Integration and unit tests
- `cable/` - Channel configuration files (unix/windows)
- `themes/` - UI theme definitions

### Key files:
- `television/main.rs` - Binary entry point
- `television/lib.rs` - Library root
- `television/app.rs` - Application orchestrator
- `television/television.rs` - Core state management
- `justfile` - Development commands and build scripts
- `cable/unix/` - Unix/Linux channel configurations
- `cable/windows/` - Windows-specific channel configurations
- `themes/` - UI theme definitions
- `tests/` - Integration and unit tests

## Channel Development

Channels are the extensible data sources. To create a new channel:

1. Create a TOML file in `cable/unix/` or `cable/windows/`
2. Define metadata, source command, preview command, UI settings, and keybindings
3. Example structure:
   ```toml
   [metadata]
   name = "my-channel"
   description = "Description"
   
   [source]
   command = "command-that-outputs-data"
   
   [preview]
   command = "preview-command '{}'"
   
   [ui]
   preview_panel = { size = 70 }
   
   [keybindings]
   shortcut = "f1"
   ```

## Dependencies and Tech Stack

- **Async Runtime**: tokio (full features)
- **TUI Framework**: ratatui with serde support
- **Fuzzy Matching**: nucleo (from helix editor)
- **CLI Parsing**: clap with derive features
- **Configuration**: toml, serde
- **Cross-platform**: Platform-specific crossterm features
- **Windows Support**: winapi-util, clipboard-win for Windows-specific functionality
- **Build System**: clap_mangen for man page generation
- **Current Version**: 0.13.3 (Rust Edition 2024)

## Testing Guidelines

The project has comprehensive integration tests in `tests/` covering:
- Channel functionality
- CLI argument parsing
- Configuration merging
- UI behavior
- Preview system
- Remote control
- Cross-platform path handling
- Command execution modes
- Async event handling

Run tests with output using `just test` to see detailed results.

### Test Coverage:
- Unit tests for core modules
- Integration tests for full application workflow
- Cross-platform compatibility tests
- Performance benchmarks with criterion
- Windows-specific path handling tests