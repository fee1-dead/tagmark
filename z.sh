#!/usr/bin/env bash
set -e
cargo run --release -p build_system -- $@
