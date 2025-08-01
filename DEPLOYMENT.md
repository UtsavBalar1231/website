# Deployment Guide

This project uses GitHub Actions for automated deployment with WebAssembly (WASM) build support.

## Deployment Options

### 1. GitHub Pages (Recommended)

The main deployment uses GitHub Actions to build and deploy to GitHub Pages:

- **File**: `.github/workflows/deploy.yml`
- **Triggers**: Push to main branch, PRs, manual dispatch
- **Features**: 
  - Full Rust/WASM build environment
  - Automatic optimization with `wasm-opt`
  - Bundle size analysis
  - PR preview comments

**Setup:**
1. Enable GitHub Pages in your repository settings
2. Set source to "GitHub Actions"
3. Push to main branch to trigger deployment

### 2. Vercel Deployment

For Vercel deployment, the WASM files are pre-built by GitHub Actions:

- **File**: `.github/workflows/vercel-build.yml`
- **Triggers**: Changes to syntax-highlighter directory
- **Purpose**: Pre-builds WASM files and commits them

**Setup:**
1. Connect your repository to Vercel
2. The build script will use pre-built WASM files
3. If WASM files are missing, run the "Build WASM for Vercel" action

## Build Process

### Local Development
```bash
npm install
npm run dev
```

### Production Build
```bash
npm run build
```

### WASM-only Build
```bash
npm run build:wasm
```

## Key Features

### WebAssembly Syntax Highlighter
- Built with Rust for performance
- Optimized for minimal bundle size
- Fallback mechanism for environments without Rust

### Build Optimizations
- CSS minification and optimization
- JavaScript bundling with Rollup
- WASM binary optimization with `wasm-opt`
- Static asset optimization

### Performance Monitoring
- Bundle size reporting
- Build time tracking
- Performance budget checking

## Environment Variables

No environment variables required for basic deployment.

## Troubleshooting

### Vercel "cargo: command not found"
- The GitHub Action will pre-build WASM files
- Vercel deployment uses these pre-built files
- No Rust installation needed in Vercel environment

### WASM Build Failures
- Check Rust toolchain installation
- Ensure `wasm-pack` is available
- Verify `wasm32-unknown-unknown` target is installed

### Bundle Size Issues
- Check the build logs for size analysis
- Consider code splitting for large bundles
- Review WASM binary size optimization

## File Structure

```
.github/workflows/
├── deploy.yml          # Main deployment workflow
└── vercel-build.yml    # WASM pre-build for Vercel

_site/                  # Built output directory
├── js/
│   ├── bundle.js       # Main JavaScript bundle
│   ├── syntax_highlighter.js     # WASM loader
│   └── syntax_highlighter_bg.wasm # WASM binary
└── ...

syntax-highlighter/     # Rust WASM source
├── build.sh           # WASM build script
├── Cargo.toml         # Rust manifest
└── src/               # Rust source code
```
