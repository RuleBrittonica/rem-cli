#!/bin/bash

cargo clean

# Also clean the cache (will require re-downloading dependencies)
cargo cache -a

# Update the dependencies
cargo update

# Build the project
cargo lcheck --release --bin rem-cli
cargo build --release --bin rem-cli

# Run the project
cargo run --release --bin rem-cli test src_tests -v