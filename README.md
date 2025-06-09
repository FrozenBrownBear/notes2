# Elephant Notes

Elephant Notes is a simple note‐taking application written in Rust.  The project
provides both a graphical user interface (GUI) and a terminal user interface
(TUI).

## Workspace layout

The repository is structured as a Cargo workspace with multiple crates:

- `gui/` &ndash; Rust crate for the desktop GUI
- `tui/` &ndash; Rust crate for the terminal UI
- `common/` &ndash; code shared between frontends
- `assets/` &ndash; application icons and other resources

## Building the project

Ensure Rust and Cargo are installed. Then build all crates with:

```bash
cargo build --workspace
```

## Running the GUI

```bash
cargo run -p elephant-gui
```

## Running the TUI

```bash
cargo run -p elephant-tui
```

