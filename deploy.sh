#!/bin/bash

# Deployment script for Terminal Portfolio
# Builds, optimizes, and prepares for GitHub Pages deployment

set -e

echo "Starting deployment build..."

# Clean previous build
echo "Cleaning previous build..."
rm -rf _site
mkdir -p _site

# Install dependencies if needed
if [ ! -d "node_modules" ]; then
  echo "Installing dependencies..."
  npm install
fi

# Build CSS
echo "Building CSS..."
./build-css.sh

# Build JavaScript
echo "Building JavaScript..."
npx rollup -c

# Build static site
echo "Building static site..."
npx eleventy

# Copy static assets
echo "Copying static assets..."
cp -r src/static/* _site/

# Create .nojekyll for GitHub Pages
touch _site/.nojekyll

# Generate sitemap
echo "Generating sitemap..."
cat > _site/sitemap.xml << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
    <loc>https://utsavbalar.in/</loc>
    <lastmod>2025-08-01</lastmod>
    <changefreq>weekly</changefreq>
    <priority>1.0</priority>
  </url>
  <url>
    <loc>https://utsavbalar.in/about/</loc>
    <lastmod>2025-08-01</lastmod>
    <changefreq>monthly</changefreq>
    <priority>0.8</priority>
  </url>
  <url>
    <loc>https://utsavbalar.in/projects/</loc>
    <lastmod>2025-08-01</lastmod>
    <changefreq>weekly</changefreq>
    <priority>0.9</priority>
  </url>
  <url>
    <loc>https://utsavbalar.in/resume/</loc>
    <lastmod>2025-08-01</lastmod>
    <changefreq>monthly</changefreq>
    <priority>0.7</priority>
  </url>
  <url>
    <loc>https://utsavbalar.in/tutorials/</loc>
    <lastmod>2025-08-01</lastmod>
    <changefreq>weekly</changefreq>
    <priority>0.8</priority>
  </url>
  <url>
    <loc>https://utsavbalar.in/books/</loc>
    <lastmod>2025-08-01</lastmod>
    <changefreq>monthly</changefreq>
    <priority>0.6</priority>
  </url>
  <url>
    <loc>https://utsavbalar.in/quotes/</loc>
    <lastmod>2025-08-01</lastmod>
    <changefreq>monthly</changefreq>
    <priority>0.5</priority>
  </url>
  <url>
    <loc>https://utsavbalar.in/contact/</loc>
    <lastmod>2025-08-01</lastmod>
    <changefreq>monthly</changefreq>
    <priority>0.7</priority>
  </url>
</urlset>
EOF

# Bundle size analysis
echo "Analyzing bundle sizes..."

CSS_SIZE=$(stat -f%z _site/css/main.css 2>/dev/null || stat -c%s _site/css/main.css 2>/dev/null)
CSS_GZIP=$(gzip -c _site/css/main.css | wc -c)

JS_SIZE=$(stat -f%z _site/js/bundle.js 2>/dev/null || stat -c%s _site/js/bundle.js 2>/dev/null)
JS_GZIP=$(gzip -c _site/js/bundle.js | wc -c)

# Calculate total HTML size
HTML_SIZE=$(find _site -name "*.html" -exec cat {} \; | wc -c)
HTML_GZIP=$(find _site -name "*.html" -exec cat {} \; | gzip -c | wc -c)

# Calculate total size
TOTAL_RAW=$((CSS_SIZE + JS_SIZE + HTML_SIZE))
TOTAL_GZIP=$((CSS_GZIP + JS_GZIP + HTML_GZIP))

echo ""
echo "Bundle Size Report:"
echo "================================"
echo "CSS:     ${CSS_SIZE} bytes (${CSS_GZIP} gzipped)"
echo "JS:      ${JS_SIZE} bytes (${JS_GZIP} gzipped)"
echo "HTML:    ${HTML_SIZE} bytes (${HTML_GZIP} gzipped)"
echo "--------------------------------"
echo "Total:   ${TOTAL_RAW} bytes (${TOTAL_GZIP} gzipped)"
echo ""

# Check budget (200 KiB = 204800 bytes)
if [ $TOTAL_GZIP -gt 204800 ]; then
  echo "Warning: Total size exceeds 200 KiB budget!"
  echo "   Budget:   204800 bytes"
  echo "   Actual:   ${TOTAL_GZIP} bytes" 
  echo "   Overage:  $((TOTAL_GZIP - 204800)) bytes"
else
  echo "Total size within 200 KiB budget"
  echo "   Remaining: $((204800 - TOTAL_GZIP)) bytes"
fi

echo ""
echo "Build complete! Site ready for deployment in _site/"
echo ""
echo "To deploy to GitHub Pages:"
echo "  git add _site"
echo "  git commit -m 'Deploy: $(date)'"
echo "  git subtree push --prefix _site origin gh-pages"
