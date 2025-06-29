# pykill

A cross-platform TUI tool for finding and deleting Python virtual environments.

## Features

*   **Interactive TUI:** A user-friendly terminal interface for navigating and managing Python virtual environments.
*   **Cross-Platform:** Works on Linux, Windows, and macOS.
*   **Virtual Environment Detection:** Scans directories to find common Python virtual environment folders (e.g., `venv`, `.venv`, `env`).
*   **Interactive Deletion:** Safely delete virtual environments with a confirmation step.
*   **Command-Line Mode:** Option to list virtual environments without launching the TUI.
*   **Fast Scanning:** Built in Rust for efficient directory traversal and analysis.

## Installation

### From Releases (Recommended)

You can download pre-compiled binaries for your operating system from the GitHub Releases page:

[https://github.com/YOUR_GITHUB_USER/YOUR_REPO_NAME/releases](https://github.com/YOUR_GITHUB_USER/YOUR_REPO_NAME/releases)

Binaries are typically provided for Linux, Windows, and macOS (x86_64 and aarch64). Download the appropriate archive for your system, extract it, and place the `pykill` executable in a directory included in your system's PATH.

### From Source

If you have Rust and Cargo installed, you can build `pykill` from source.

1.  **Install via `cargo install` (recommended for source installs):**
    ```bash
    cargo install --git https://github.com/YOUR_GITHUB_USER/YOUR_REPO_NAME.git
    ```
    This will build and install the `pykill` binary into your Cargo bin directory (e.g., `~/.cargo/bin/pykill`).

2.  **Manual Build:**
    Alternatively, you can clone the repository and build it manually:
    ```bash
    git clone https://github.com/YOUR_GITHUB_USER/YOUR_REPO_NAME.git
    cd YOUR_REPO_NAME 
    cargo build --release
    ```
    The binary will be located at `target/release/pykill`. You can then copy this to a location in your PATH.

## Usage

### TUI Mode (Default)

To start `pykill` in its interactive TUI mode, simply run the command:

```bash
pykill
```

This will scan the current directory for virtual environments.

You can also specify a path to scan:

```bash
pykill /path/to/your/projects
```

**Keybindings:**

*   **`↑` / `↓`**: Navigate up and down the list of detected virtual environments.
*   **`d`**: Mark the currently selected virtual environment for deletion. This will open a confirmation dialog.
*   **`y`**: (In confirmation dialog) Confirm the deletion of the virtual environment.
*   **`n`**: (In confirmation dialog) Cancel the deletion and close the dialog.
*   **`q`**: Quit the application. This works in both the main list view and the confirmation dialog.

### Command-Line Mode

If you prefer to get a simple list of virtual environments printed to your terminal without the TUI, use the `--no-tui` flag.

*   Scan the current directory:
    ```bash
    pykill --no-tui
    ```

*   Scan a specific directory:
    ```bash
    pykill /path/to/your/projects --no-tui
    ```

The output will list the path, size, and last modified date for each detected virtual environment.

## Building from Source

If you wish to contribute or build the latest version yourself:

1.  **Prerequisites:**
    *   Ensure you have Rust and Cargo installed. You can get them from [rustup.rs](https://rustup.rs/).

2.  **Clone the repository:**
    ```bash
    git clone https://github.com/YOUR_GITHUB_USER/YOUR_REPO_NAME.git
    ```

3.  **Navigate to the directory:**
    ```bash
    cd YOUR_REPO_NAME 
    ```
    (Note: If the repository is named `pykill`, you would `cd pykill`)

4.  **Build the release binary:**
    ```bash
    cargo build --release
    ```

5.  **Run the binary:**
    The executable will be located at `target/release/pykill`.

## Contributing

Contributions are welcome! If you have suggestions, feature requests, or bug reports, please open an issue or submit a pull request on the GitHub repository.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details (if one is present, otherwise assume MIT).
