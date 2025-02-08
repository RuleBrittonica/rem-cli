#!/bin/bash

# Update the dependencies
cargo update

# Run the project
cargo run  --release --bin rem-cli test src_tests -v