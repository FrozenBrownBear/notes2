#!/bin/sh
set -e

if [ "$1" = "--clean" ]; then
    cargo clean
fi

if ! command -v sccache >/dev/null 2>&1; then
    echo "sccache not available, disabling rustc wrapper" >&2
    RUSTC_WRAPPER=""
fi

# Pre-fetch dependencies
cargo fetch

# Precompile frequently used crates in release mode
cargo build --release \
    -p core \
    -p config \
    -p markdown_renderer \
    -p interactive_widgets \
    -p storage >/dev/null
