#!/bin/bash

# Modurust DAW Release Build Script
# This script builds optimized release binaries for all platforms

set -e

echo "🎵 Building Modurust DAW Release"

# Clean previous builds
echo "🧹 Cleaning previous builds..."
cargo clean

# Build release version
echo "🔨 Building release version..."
cargo build --release

# Run tests
echo "🧪 Running tests..."
cargo test --release

# Create distribution directory
echo "📦 Creating distribution..."
mkdir -p dist

# Copy binary
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    cp target/release/modurust_daw dist/modurust-daw-linux
elif [[ "$OSTYPE" == "darwin"* ]]; then
    cp target/release/modurust_daw dist/modurust-daw-macos
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "win32" ]]; then
    cp target/release/modurust_daw.exe dist/modurust-daw-windows.exe
fi

echo "✅ Release build completed!"
echo "📁 Binaries available in ./dist/"