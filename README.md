# Terminal Portfolio - Utsav Balar

A performance-optimized, terminal-inspired personal portfolio website built with modern web technologies while maintaining a strict 200 KiB total transfer size budget.

## Live Demo

Visit the live site: [utsavbalar.in](https://utsavbalar.in)

## Features

- **Terminal Aesthetic**: Retro shell interface with modern UX polish
- **Multi-Theme Support**: 5 carefully crafted themes (classic-dark, light, hacker-green, matrix, solarized-dark)
- **Performance Optimized**: <200 KiB total gzipped, <1s First Contentful Paint on 3G
- **Progressive Web App**: Offline support with service worker caching
- **SPA-like Navigation**: Smooth client-side routing with fallback to full page loads
- **Responsive Design**: Mobile-first approach with terminal-optimized layouts
- **Accessibility**: WCAG AA compliant with keyboard navigation support
- **SEO Optimized**: Structured data, OpenGraph tags, and optimized meta information

## Technology Stack

- **Static Site Generator**: Eleventy 3.x for zero-JS output
- **Bundler**: Rollup with Terser for minimal JavaScript bundle
- **Styling**: Hand-crafted SCSS compiled with Sass and optimized with lightningcss
- **Deployment**: GitHub Pages with automated CI/CD
- **PWA**: Workbox-generated service worker with stale-while-revalidate strategy

## Performance Budget

| Asset Type | Target (raw) | Target (gzipped) | Status |
|------------|--------------|------------------|--------|
| HTML (7 pages) | 40 KiB | 8 KiB | OK |
| CSS | 6 KiB | 1 KiB | OK |
| JavaScript | 12 KiB | 4 KiB | OK |
| ASCII banners | 50 KiB | 15 KiB | OK |
| PWA assets | 25 KiB | 8 KiB | OK |
| **Total** | **133 KiB** | **<200 KiB** | OK |

## Project Structure

```
├── content/                 # Markdown content files
│   ├── tutorials/          # Kernel development tutorials
│   ├── about.md            # About page content
│   ├── projects.md         # Project showcase
│   ├── resume.md           # Professional resume
│   ├── books.md            # Book excerpts and notes
│   ├── quotes.md           # Inspirational quotes
│   └── contact.md          # Contact information
├── data/                   # Source data files
├── src/
│   ├── _includes/          # Nunjucks templates
│   ├── styles/             # SCSS source files
│   ├── js/                 # JavaScript source
│   └── static/             # Static assets
├── _site/                  # Generated site output
├── .eleventy.js            # Eleventy configuration
├── rollup.config.js        # Rollup bundler config
├── package.json            # Dependencies and scripts
└── deploy.sh               # Deployment automation
```

## Quick Start

### Prerequisites
- Node.js 18+ 
- npm or pnpm
- Git

### Installation

```bash
# Clone the repository
git clone https://github.com/UtsavBalar1231/NewUtsavBalar.git
cd NewUtsavBalar

# Install dependencies
npm install

# Start development server
npm run dev
```

The site will be available at `http://localhost:8080` with live reload.

### Development Commands

```bash
# Development with live reload
npm run dev

# Production build
npm run build

# Preview production build
npm run preview

# Lint JavaScript
npm run lint

# Format code
npm run format

# Check bundle sizes
npm run size

# Full deployment build
./deploy.sh
```

## Themes

The portfolio includes 5 carefully designed themes:

1. **Classic Dark** (default): Green-on-black terminal aesthetic
2. **Light**: Professional light mode for readability
3. **Hacker Green**: Classic bright green Matrix-style terminal
4. **Matrix**: Stylized Matrix movie-inspired theme
5. **Solarized Dark**: Popular developer-friendly color scheme

Themes can be switched using the floating theme selector or keyboard shortcut `T`.

## Keyboard Shortcuts

- `T` - Cycle through themes
- `H` - Navigate to Home
- `A` - Navigate to About
- `P` - Navigate to Projects
- `R` - Navigate to Resume
- `C` - Navigate to Contact

## Progressive Web App

The site works offline and can be installed as a PWA:

- **Offline Support**: Service worker caches all critical assets
- **Installation**: Add to home screen on mobile/desktop
- **Background Sync**: Form submissions sync when back online
- **Push Notifications**: Ready for future engagement features

## Customization

### Adding New Content

1. **Pages**: Create new `.md` files in `content/`
2. **Navigation**: Update `src/_includes/nav.njk`
3. **Tutorials**: Add to `content/tutorials/` with proper frontmatter
4. **Projects**: Update `content/projects.md`

### Styling Changes

1. Edit `src/styles/main.scss`
2. Run `./build-css.sh` to compile
3. Check size budget with `npm run size`

### Theme Customization

Themes are defined using CSS custom properties in `src/styles/main.scss`:

```scss
[data-theme="custom"] {
  --bg-primary: #your-bg-color;
  --text-primary: #your-text-color;
  --link-color: #your-link-color;
  // ... other properties
}
```

## Deployment

### GitHub Pages (Recommended)

```bash
# Build and deploy
./deploy.sh

# Deploy to gh-pages branch
git subtree push --prefix _site origin gh-pages
```

### Netlify

1. Connect your GitHub repository
2. Set build command: `npm run build`
3. Set publish directory: `_site`

### Other Hosting

The `_site` directory contains all static files and can be deployed to any static hosting service.

## Performance Optimization

### Implemented Optimizations

- **Critical CSS inlining**: Above-the-fold styles inlined
- **JavaScript minification**: Terser with aggressive settings
- **Image optimization**: SVG icons and optimized PNGs
- **Resource hints**: Preload critical assets
- **Service worker caching**: Stale-while-revalidate strategy
- **Bundle analysis**: Automated size checking

### Monitoring

Use the built-in bundle analysis:

```bash
./deploy.sh  # Shows detailed size report
npm run size # Quick size check
```

## Testing

### Manual Testing Checklist

- [ ] All pages load correctly
- [ ] Theme switching works
- [ ] Navigation functions (both click and keyboard)
- [ ] SPA routing works with JavaScript disabled
- [ ] PWA installation works
- [ ] Offline functionality works
- [ ] Mobile responsive design
- [ ] Screen reader accessibility

### Performance Testing

```bash
# Lighthouse CI (if configured)
npm run lighthouse

# Bundle size validation
npm run size
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Code Style

- ESLint configuration in `.eslintrc.js`
- Prettier configuration in `.prettierrc`
- Run `npm run format` before committing

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## About the Author

**Utsav Balar** - Embedded Linux & BSP Engineer

- GitHub: [@UtsavBalar1231](https://github.com/UtsavBalar1231)
- LinkedIn: [utsavbalar](https://linkedin.com/in/utsavbalar)
- Email: utsavbalar1231@gmail.com

Specializing in Linux kernel development, device drivers, and custom ROM development for embedded systems.

## Acknowledgments

- Terminal design inspiration from classic Unix systems
- Performance optimization techniques from web.dev
- Accessibility guidelines from WCAG 2.1
- PWA best practices from Google's PWA documentation

---

*Built with love and lots of coffee by Utsav Balar*
