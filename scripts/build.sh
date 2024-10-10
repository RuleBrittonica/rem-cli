#!/bin/bash

# Update the dependencies
cargo update

# Run the project
cargo lcheck && cargo build --release --bin rem-cli