#!/bin/bash

# Create the releases directory if it doesn't exist
mkdir -p releases

# Build for aarch64-apple-darwin
cargo build --release --target=aarch64-apple-darwin
tar -czf releases/rsii-macos-arm64.tar.gz -C target/aarch64-apple-darwin/release rsii

# Build for x86_64-apple-darwin
cargo build --release --target=x86_64-apple-darwin
tar -czf releases/rsii-macos-x86_64.tar.gz -C target/x86_64-apple-darwin/release rsii

# Generate and print checksums
echo "Checksums:"
shasum -a 256 releases/rsii-macos-arm64.tar.gz
shasum -a 256 releases/rsii-macos-x86_64.tar.gz
