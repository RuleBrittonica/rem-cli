#!/bin/bash

cargo clean

# Build the project
cargo build --release --bin rem-cli

# Run the project
cargo run --release --bin rem-cli test -v 