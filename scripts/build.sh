#!/bin/bash

# Update the dependencies
cargo update

# Run the project
cargo lcheck --bin rem-cli
cargo build --bin rem-cli