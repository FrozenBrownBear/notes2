#!/bin/sh
set -e

cargo fmt --all
cargo clippy --workspace -- -D warnings
cargo build --workspace
cargo test --workspace
