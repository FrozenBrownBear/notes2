#!/bin/sh
set -e

if [ "$1" = "--clean" ]; then
    cargo clean
fi

# Precompile common crates in release mode
cargo build --release -p core -p storage >/dev/null
