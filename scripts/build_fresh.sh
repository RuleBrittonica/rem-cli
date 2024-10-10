#!/bin/bash

cargo clean

# Also clean the cache (will require re-downloading dependencies)
cargo cache -a

# Update the dependencies
cargo update

# Build the project
cargo lcheck && cargo build --bin rem-cli