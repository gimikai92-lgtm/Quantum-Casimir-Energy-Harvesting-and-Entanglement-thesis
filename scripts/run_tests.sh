#!/usr/bin/env bash
set -euo pipefail
# Run Rust unit and integration tests
cargo test --all
