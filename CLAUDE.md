# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

### Primary Development Workflow
```bash
# Development with live reload (concurrent CSS and JS watching)
npm run dev

# Production build (full optimization)
npm run build

# Preview production build locally
npm run preview

# Clean build artifacts
npm run clean
```

### Code Quality & Optimization
```bash
# Lint JavaScript files
npm run lint
npm run lint:fix

# Format all code (JS, TS, CSS, SCSS, MD)
npm run format

# Check bundle size against performance budget
npm run size
```

### CSS Build Process
```bash
# Manual CSS compilation (SCSS → CSS with lightningcss optimization)
./build-css.sh
```

### Deployment
```bash
# Full deployment build with size analysis and GitHub Pages preparation
./deploy.sh

# Manual GitHub Pages deployment after build
git subtree push --prefix _site origin gh-pages
```

## Architecture Overview

### Technology Stack
- **Static Site Generator**: Eleventy 3.x (ESM configuration)
- **Bundler**: Rollup with Terser (aggressive minification)
- **Styling**: SCSS → lightningcss optimization pipeline
- **JavaScript**: Vanilla ES6+ with class-based architecture
- **Templates**: Nunjucks (.njk) with Markdown content
- **PWA**: Service worker with Workbox integration

### Directory Structure
```
├── content/              # Markdown content (input to Eleventy)
│   ├── tutorials/        # Technical tutorials with frontmatter
│   └── *.md             # Page content (about, projects, resume, etc.)
├── data/                # Global data files (JSON/JS/MD)
├── src/
│   ├── _includes/       # Nunjucks templates and partials
│   ├── styles/          # SCSS source files
│   ├── js/              # JavaScript modules (bundled by Rollup)
│   └── static/          # Static assets (favicon, manifest, SW)
├── _site/               # Generated output (deployment target)
├── .eleventy.js         # Eleventy configuration (ESM)
├── rollup.config.js     # JavaScript bundling configuration
└── package.json         # Dependencies and build scripts
```

### Build Pipeline
1. **CSS Pipeline**: SCSS compilation → lightningcss optimization → output to `_site/css/`
2. **JavaScript Pipeline**: Rollup bundling → Terser minification → output to `_site/js/bundle.js`
3. **Static Site Generation**: Eleventy processes Markdown + Nunjucks → HTML output
4. **Asset Copying**: Static files copied to `_site/static/`
5. **Size Analysis**: Automated bundle size checking against 200 KiB budget

### JavaScript Architecture
**Class-based modular design** in `src/js/main.js`:
- **ThemeManager**: Multi-theme system with localStorage persistence
- **SPARouter**: Client-side navigation with graceful fallback
- **KeyboardShortcuts**: Terminal-style keyboard navigation (T, H, A, P, R, C)
- **PerformanceOptimizer**: Lazy loading, prefetching, critical page preloading
- **PortfolioApp**: Main application orchestrator

### Template System
- **Base Template**: `src/_includes/base.njk` with comprehensive SEO metadata
- **Content Processing**: Markdown → Nunjucks → HTML
- **Collections**: Custom tutorial collection with numerical sorting
- **Data Sources**: Global data from `data/` directory accessible in templates

### Performance Constraints
**Strict performance budget** enforced by build tools:
- **Total Transfer**: <200 KiB gzipped
- **CSS**: <1 KiB gzipped (target in bundlesize config)
- **JavaScript**: <4 KiB gzipped (target in bundlesize config)
- **Automated Validation**: `npm run size` and `./deploy.sh` report violations

### PWA Implementation
- **Service Worker**: Auto-generated caching with stale-while-revalidate
- **Manifest**: `/manifest.json` for installable web app
- **Offline Support**: Critical assets cached for offline browsing
- **Theme Integration**: PWA theme-color matches current theme

### Content Management
- **Tutorials**: Structured with `part` frontmatter for automatic ordering
- **SEO Optimization**: Comprehensive meta tags, JSON-LD structured data, Open Graph
- **Accessibility**: WCAG AA compliance, semantic HTML, keyboard navigation
- **Multi-theme Support**: 5 themes with CSS custom properties

### Development Patterns
- **Terminal Aesthetic**: Retro shell interface with modern UX polish
- **Mobile-first**: Responsive design optimized for terminal layouts
- **Progressive Enhancement**: Works without JavaScript, enhanced with SPA features
- **Performance First**: Every feature evaluated against 200 KiB budget constraint

## Key Configuration Files
- `.eleventy.js`: Eleventy configuration, collections, filters, and build paths
- `rollup.config.js`: JavaScript bundling with terser optimization
- `package.json`: Build scripts and performance budget definitions (bundlesize)
- `deploy.sh`: Production deployment script with automated size analysis