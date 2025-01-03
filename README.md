# Nimble GUI

A cross-platform graphical user interface for the Nimble mod manager for Arma 3.

## Features

- Sync mods from a remote repository
- Generate SRF cache files
- Launch Arma 3 with configured mods
- User-friendly interface with file browsing
- Cross-platform support (Windows, Linux with Proton)

## Building from Source

1. Install Rust using [rustup](https://rustup.rs/)
2. Clone the repository:
```bash
git clone https://github.com/yourusername/nimble_gui.git
cd nimble_gui
```
3. Build the project:
```bash
cargo build --release
```

The compiled binary will be available in `target/release/nimble-gui`

## Dependencies

- eframe (egui) for the GUI framework
- nimble (included as a workspace member) for the core functionality
- rfd for native file dialogs

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## Related Projects

- [Nimble](https://github.com/vitorhnn/nimble) - The core mod manager library
