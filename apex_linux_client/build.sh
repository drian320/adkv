#!/bin/bash

echo "Building Apex Linux GUI Client..."
cargo build --release

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Build successful!"
    echo "Run with: cargo run --release"
    echo "Or directly: ./target/release/apex_linux_client"
else
    echo "❌ Build failed!"
    exit 1
fi

