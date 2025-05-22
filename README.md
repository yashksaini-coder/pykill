# pykill

pykill is a Rust-based utility (currently in early development) designed to scan projects for Python virtual environments. The goal is to provide a fast, cross-platform tool with a terminal user interface (TUI) for managing and potentially cleaning up Python virtual environments within a given directory tree.

## Features (Planned)
- **Scan for Python Virtual Environments:** Detect virtual environments in a project using Python's `sys.prefix` or directory heuristics.
- **Terminal User Interface:** Built with [ratatui](https://crates.io/crates/ratatui) and [crossterm](https://crates.io/crates/crossterm) for a modern, interactive TUI experience.
- **Project Navigation:** Easily browse and select directories to scan.
- **Environment Management:** (Planned) List, inspect, and optionally remove unused or large virtual environments.

## Status
This project is in the early stages. Most modules are placeholders with TODOs for future implementation.

## Dependencies
- [ratatui](https://crates.io/crates/ratatui) - TUI rendering
- [crossterm](https://crates.io/crates/crossterm) - Terminal handling
- [walkdir](https://crates.io/crates/walkdir) - Recursive directory traversal
- [humansize](https://crates.io/crates/humansize) - Human-readable file sizes
- [chrono](https://crates.io/crates/chrono) - Date and time utilities

## Usage
1. **Build the project:**
   ```sh
   cargo build --release
   ```
2. **Run the project:**
   ```sh
   cargo run -- <project_path>
   ```
   (Note: Functionality is not yet implemented; running will only print "Hello, world!" for now.)

## Contributing
Contributions are welcome! Please open issues or pull requests to discuss features or report bugs.

## License
This project is licensed under the MIT License.
