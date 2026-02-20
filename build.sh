#!/bin/bash
# Build script for dateno-duckdb extension
# Builds for multiple platforms

set -e

echo "Building dateno-duckdb extension..."

# Build for current platform
echo "Building for current platform..."
cargo build --release

# Cross-compilation targets (requires cross tool)
if command -v cross &> /dev/null; then
    echo "Building for Linux (musl)..."
    cross build --release --target x86_64-unknown-linux-musl
    
    echo "Building for Linux ARM64..."
    cross build --release --target aarch64-unknown-linux-gnu
else
    echo "cross tool not found. Install with: cargo install cross"
    echo "Skipping cross-compilation..."
fi

echo "Build complete!"
echo ""
echo "Extension libraries:"
find target -name "*.so" -o -name "*.dylib" -o -name "*.dll" | grep release || echo "No libraries found"
