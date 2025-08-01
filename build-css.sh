#!/bin/bash

# CSS Build Script for Terminal Portfolio
# Compiles SCSS to CSS with optimization for <1 KiB gzipped target

set -e

echo "Building CSS for terminal portfolio..."

# Create output directory
mkdir -p _site/css

# Compile SCSS to CSS with compression
sass src/styles/main.scss _site/css/main.css \
  --style=compressed \
  --no-source-map \
  --load-path=node_modules

# Use lightningcss for additional optimization
if command -v lightningcss >/dev/null 2>&1; then
  echo "Optimizing CSS with lightningcss..."
  lightningcss --minify _site/css/main.css -o _site/css/main.css
fi

# Check file size
CSS_SIZE=$(stat -f%z _site/css/main.css 2>/dev/null || stat -c%s _site/css/main.css 2>/dev/null)
CSS_GZIP_SIZE=$(gzip -c _site/css/main.css | wc -c)

echo "CSS build complete:"
echo "   Raw size: ${CSS_SIZE} bytes"
echo "   Gzipped:  ${CSS_GZIP_SIZE} bytes"

# Warn if over budget
if [ $CSS_GZIP_SIZE -gt 1024 ]; then
  echo "Warning: CSS size exceeds 1 KiB gzipped budget"
else
  echo "CSS size within budget"
fi

# Copy to src/css for development
cp _site/css/main.css src/css/main.css

echo "CSS build complete!"