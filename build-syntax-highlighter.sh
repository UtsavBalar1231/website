#!/bin/bash

# Build script integration for syntax highlighter
# Runs as part of main build process

echo "🔧 Building syntax highlighter..."

# Check if syntax-highlighter directory exists
if [ ! -d "syntax-highlighter" ]; then
    echo "❌ Syntax highlighter not found. Skipping."
    exit 0
fi

# Build the WASM module
cd syntax-highlighter
if ./build.sh; then
    echo "✅ Syntax highlighter built successfully"
    
    # Copy files to _site
    cd ..
    cp syntax-highlighter/pkg/syntax_highlighter.js _site/js/
    cp syntax-highlighter/pkg/syntax_highlighter_bg.wasm _site/js/
    
    echo "📁 WASM files copied to _site/js/"
else
    echo "❌ Syntax highlighter build failed"
    exit 1
fi

echo "🎯 Syntax highlighter integration complete"