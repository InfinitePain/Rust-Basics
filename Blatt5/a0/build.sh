#!/bin/bash

# Compile p2
cargo build --bin p2
if [ $? -ne 0 ]; then
    echo "Failed to compile p2"
    exit 1
fi

# Compile p1
cargo build --bin p1
if [ $? -ne 0 ]; then
    echo "Failed to compile p1"
    exit 1
fi

# Get the path to p2 executable
P2_PATH="./target/debug/p2"

# Run p1 with the path to p2
cargo run --bin p1 -- -p "$P2_PATH"