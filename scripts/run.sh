#!/bin/bash

# Update the dependencies
cargo update

# Run the project
cargo run  --release --bin rem-cli test-github https://github.com/RuleBrittonica/rem-testfiles -v