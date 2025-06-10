# Elephant Notes

Elephant Notes is a simple note‐taking application written in Rust.  The project
provides both a graphical user interface (GUI) and a terminal user interface
(TUI).

## Workspace layout

The repository is structured as a Cargo workspace with multiple crates:

 - `ui_iced/` &ndash; Rust crate for the desktop GUI
 - `tui_editor/` &ndash; Rust crate for the terminal UI
 - `core/` &ndash; code shared between frontends
- `assets/` &ndash; application icons and other resources

## Building the project

Ensure Rust and Cargo are installed. Then build all crates with:

```bash
cargo build --workspace
```

## Running the GUI

```bash
cargo run -p ui_iced
```

## Running the TUI

```bash
cargo run -p tui_editor
```

## Unified launcher

The `launcher` binary lets you select the interface at runtime:

```bash
cargo run -p launcher -- --mode gui
cargo run -p launcher -- --mode tui
```

## Faster builds

The workspace includes a `.cargo/config.toml` that enables the
[`sccache`](https://github.com/mozilla/sccache) compiler cache and
incremental release mode. Installing `sccache` can greatly reduce build
times on repeated compilations.

