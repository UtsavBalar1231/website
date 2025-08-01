#!/bin/bash

# Ultra-optimized WASM build script for syntax highlighter
# Targets minimal bundle size and maximum performance

set -e

echo "🦀 Building ultra-lightweight syntax highlighter..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack not found. Installing..."
    cargo install wasm-pack
fi

# Clean previous builds
echo "🧹 Cleaning previous builds..."
rm -rf pkg/
rm -rf target/

# Build optimized WASM package
echo "⚡ Building optimized WASM package..."
wasm-pack build \
    --target web \
    --out-dir pkg \
    --release \
    --no-typescript \
    --no-pack

# Further optimize the WASM binary
echo "🔧 Optimizing WASM binary..."
if command -v wasm-opt &> /dev/null; then
    echo "Using wasm-opt for further optimization..."
    wasm-opt -Os pkg/syntax_highlighter_bg.wasm -o pkg/syntax_highlighter_bg.wasm
else
    echo "⚠️  wasm-opt not found. Consider installing binaryen for smaller binaries:"
    echo "   Ubuntu/Debian: sudo apt install binaryen"
    echo "   macOS: brew install binaryen"
    echo "   Or build from source: https://github.com/WebAssembly/binaryen"
fi

# Display build stats
echo "📊 Build statistics:"
if [ -f pkg/syntax_highlighter_bg.wasm ]; then
    WASM_SIZE=$(wc -c < pkg/syntax_highlighter_bg.wasm)
    WASM_SIZE_KB=$((WASM_SIZE / 1024))
    echo "   WASM binary: ${WASM_SIZE_KB}KB (${WASM_SIZE} bytes)"
fi

if [ -f pkg/syntax_highlighter.js ]; then
    JS_SIZE=$(wc -c < pkg/syntax_highlighter.js)
    JS_SIZE_KB=$((JS_SIZE / 1024))
    echo "   JS glue code: ${JS_SIZE_KB}KB (${JS_SIZE} bytes)"
fi

# Calculate total size
if [ -f pkg/syntax_highlighter_bg.wasm ] && [ -f pkg/syntax_highlighter.js ]; then
    TOTAL_SIZE=$((WASM_SIZE + JS_SIZE))
    TOTAL_SIZE_KB=$((TOTAL_SIZE / 1024))
    echo "   Total size: ${TOTAL_SIZE_KB}KB (${TOTAL_SIZE} bytes)"
    
    # Check against our target
    if [ $TOTAL_SIZE_KB -lt 2 ]; then
        echo "✅ Size target achieved! (< 2KB)"
    else
        echo "⚠️  Size target missed (${TOTAL_SIZE_KB}KB > 2KB)"
    fi
fi

echo "✨ Build complete! Files generated in pkg/"
echo "📁 pkg/syntax_highlighter.js - JS loader"
echo "📁 pkg/syntax_highlighter_bg.wasm - WASM binary"