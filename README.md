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
cargo run -p ui_iced -- --origin /path/to/notes
```

## Running the TUI

```bash
cargo run -p tui_editor -- --origin /path/to/notes
```

## Notes origin directory

Both interfaces require a folder where notes are stored. Provide it with
the `--origin` argument or set the `ELEPHANT_NOTES_ORIGIN` environment variable.
The folder will be created automatically if it does not exist.

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

To warm up the build cache locally and fetch all dependencies, run:

```bash
./tools/prepare_build.sh
```

Pass `--clean` to the script if you need a full rebuild. The script also falls back
to normal `cargo` compilation when `sccache` is not available.

On Windows you can install `sccache` with:

```bash
cargo install sccache # or scoop install sccache
```

After installing `sccache`, running `./tools/prepare_build.sh` should reduce
subsequent compilation times to just a few minutes.

## Pixel art TUI

The terminal interface renders a small "pixel art" sidebar so that the look and
feel matches the graphical application. Run it with:

```bash
cargo run -p tui_editor -- --origin /path/to/notes
```

## Pre-commit procedure

Before committing changes, run the helper script to format, lint, build and test
the workspace:

```bash
./tools/pre_commit.sh
```

The CI workflow also builds the GUI and TUI crates on Linux, macOS and Windows
to ensure cross-platform compatibility.

