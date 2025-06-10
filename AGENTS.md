# Codex agent instructions

## Pre-compile helper

To speed up builds, run `./tools/prepare_build.sh`. This script precompiles
selected crates in release mode. If you need a full rebuild, pass `--clean`
to remove the target directory first.

## Required checks

Before committing, run:

```bash
./tools/prepare_build.sh
cargo test --workspace --quiet
```

All tests must pass.
