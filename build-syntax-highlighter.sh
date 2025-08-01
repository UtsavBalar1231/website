#!/bin/bash

# Build script integration for syntax highlighter
# Runs as part of main build process

echo "Building syntax highlighter..."

# Check if syntax-highlighter directory exists
if [ ! -d "syntax-highlighter" ]; then
	echo "Syntax highlighter not found. Skipping."
	exit 0
fi


# Check if cargo is available
if ! command -v cargo &>/dev/null; then
	echo "Cargo not found. Attempting to use pre-built files..."

	# Try to copy from existing pkg directory
	if [ -f "syntax-highlighter/pkg/syntax_highlighter.js" ] && [ -f "syntax-highlighter/pkg/syntax_highlighter_bg.wasm" ]; then
		echo "Found existing WASM files in pkg directory"
		cp syntax-highlighter/pkg/syntax_highlighter.js _site/js/
		cp syntax-highlighter/pkg/syntax_highlighter_bg.wasm _site/js/
		echo "Using cached WASM files"
		exit 0
	fi

	echo "No pre-built WASM files found and Rust/Cargo not available"
	echo "Solution: Run GitHub Actions to pre-build WASM files"
	exit 1
fi

# Check if pre-built WASM files exist (for environments without Rust)
# if [ -f "_site/js/syntax_highlighter.js" ] && [ -f "_site/js/syntax_highlighter_bg.wasm" ]; then
# 	echo "Using pre-built WASM files (Rust not available)"
# 	echo "Syntax highlighter ready"
# 	exit 0
# fi

# Build the WASM module
cd syntax-highlighter
if ./build.sh; then
	echo "Syntax highlighter built successfully"

	# Copy files to _site
	cd ..
	cp syntax-highlighter/pkg/syntax_highlighter.js _site/js/
	cp syntax-highlighter/pkg/syntax_highlighter_bg.wasm _site/js/

	echo "WASM files copied to _site/js/"
else
	echo "Syntax highlighter build failed"
	exit 1
fi

echo "Syntax highlighter integration complete"
