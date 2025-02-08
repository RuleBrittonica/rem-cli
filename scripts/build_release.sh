#!/bin/bash

# Update the dependencies
cargo update

# Run the project
cargo lcheck --release --bin rem-cli
cargo build --release --bin rem-cli