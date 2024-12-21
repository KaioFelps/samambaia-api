#!/usr/bin/env pwsh
cargo fmt --all -- --check && cargo clippy --all-features --all && cargo test