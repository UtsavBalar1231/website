// Terminal Portfolio - JavaScript Bundle
// Target: <4 KiB gzipped

// Import syntax highlighter
import './syntax-highlighter.js';

(function () {
  'use strict';

  // Theme Management
  class ThemeManager {
    constructor() {
      this.themes = ['classic-dark', 'light', 'hacker-green', 'matrix', 'solarized-dark'];
      this.currentTheme = this.getStoredTheme() || this.getSystemTheme();
      this.init();
    }

    init() {
      this.setTheme(this.currentTheme);
      this.bindThemeButtons();
      this.detectSystemThemeChange();
    }

    getStoredTheme() {
      return localStorage.getItem('portfolio-theme');
    }

    getSystemTheme() {
      return window.matchMedia('(prefers-color-scheme: light)').matches ? 'light' : 'classic-dark';
    }

    setTheme(theme) {
      if (!this.themes.includes(theme)) theme = 'classic-dark';

      document.documentElement.setAttribute('data-theme', theme);
      localStorage.setItem('portfolio-theme', theme);
      this.currentTheme = theme;
      this.updateActiveButton(theme);
    }

    updateActiveButton(theme) {
      document.querySelectorAll('.theme-btn').forEach(btn => {
        btn.classList.toggle('active', btn.dataset.theme === theme);
      });
    }

    bindThemeButtons() {
      document.querySelectorAll('.theme-btn').forEach(btn => {
        btn.addEventListener('click', () => {
          this.setTheme(btn.dataset.theme);
        });
      });
    }

    detectSystemThemeChange() {
      window.matchMedia('(prefers-color-scheme: light)').addEventListener('change', e => {
        if (!this.getStoredTheme()) {
          this.setTheme(e.matches ? 'light' : 'classic-dark');
        }
      });
    }
  }

  // SPA Router for smooth transitions
  class SPARouter {
    constructor() {
      this.isLoading = false;
      this.init();
    }

    init() {
      this.bindNavigation();
      this.handlePopState();
    }

    bindNavigation() {
      document.addEventListener('click', e => {
        const link = e.target.closest('a');
        if (!link || !this.isInternalLink(link) || e.ctrlKey || e.metaKey) return;

        e.preventDefault();
        this.navigate(link.href);
      });
    }

    isInternalLink(link) {
      return (
        link.hostname === window.location.hostname &&
        !link.hasAttribute('target') &&
        !link.href.includes('#')
      );
    }

    async navigate(url) {
      if (this.isLoading || url === window.location.href) return;

      this.isLoading = true;
      this.showLoadingState();

      try {
        const response = await fetch(url);
        if (!response.ok) throw new Error('Page not found');

        const html = await response.text();
        const newDoc = new DOMParser().parseFromString(html, 'text/html');

        // Update content
        const newContent = newDoc.querySelector('.terminal-content');
        const currentContent = document.querySelector('.terminal-content');

        if (newContent && currentContent) {
          await this.fadeOut(currentContent);
          currentContent.innerHTML = newContent.innerHTML;
          await this.fadeIn(currentContent);

          // Update page metadata
          document.title = newDoc.title;
          const metaDesc = newDoc.querySelector('meta[name="description"]');
          if (metaDesc) {
            const currentMeta = document.querySelector('meta[name="description"]');
            if (currentMeta) currentMeta.content = metaDesc.content;
          }

          // Update history
          window.history.pushState({}, '', url);

          // Update active navigation
          this.updateActiveNavigation(url);

          // Scroll to top
          window.scrollTo({ top: 0, behavior: 'smooth' });
        }
      } catch (error) {
        console.warn('SPA navigation failed, falling back to full page load:', error);
        window.location.href = url;
      } finally {
        this.isLoading = false;
        this.hideLoadingState();
      }
    }

    fadeOut(element) {
      return new Promise(resolve => {
        element.style.transition = 'opacity 0.2s ease-out';
        element.style.opacity = '0';
        setTimeout(resolve, 200);
      });
    }

    fadeIn(element) {
      return new Promise(resolve => {
        element.style.opacity = '1';
        setTimeout(() => {
          element.style.transition = '';
          resolve();
        }, 200);
      });
    }

    updateActiveNavigation(url) {
      const path = new URL(url).pathname;
      document.querySelectorAll('.nav-link').forEach(link => {
        const linkPath = new URL(link.href).pathname;
        link.classList.toggle('nav-active', linkPath === path);
      });
    }

    showLoadingState() {
      const cursor = document.querySelector('.cursor');
      if (cursor) cursor.style.animation = 'blink 0.5s infinite';
    }

    hideLoadingState() {
      const cursor = document.querySelector('.cursor');
      if (cursor) cursor.style.animation = 'blink 1s infinite';
    }

    handlePopState() {
      window.addEventListener('popstate', e => {
        this.navigate(window.location.href);
      });
    }
  }

  // Keyboard Shortcuts
  class KeyboardShortcuts {
    constructor() {
      this.shortcuts = {
        KeyT: () => this.cycleTheme(),
        KeyH: () => this.navigateHome(),
        KeyA: () => this.navigateAbout(),
        KeyP: () => this.navigateProjects(),
        KeyR: () => this.navigateResume(),
        KeyC: () => this.navigateContact()
      };
      this.init();
    }

    init() {
      document.addEventListener('keydown', e => {
        if (e.ctrlKey || e.metaKey || e.altKey) return;
        if (e.target.tagName === 'INPUT' || e.target.tagName === 'TEXTAREA') return;

        const handler = this.shortcuts[e.code];
        if (handler) {
          e.preventDefault();
          handler();
        }
      });
    }

    cycleTheme() {
      const themeManager = window.portfolioApp?.themeManager;
      if (!themeManager) return;

      const themes = themeManager.themes;
      const currentIndex = themes.indexOf(themeManager.currentTheme);
      const nextIndex = (currentIndex + 1) % themes.length;
      themeManager.setTheme(themes[nextIndex]);
    }

    navigateHome() {
      this.navigate('/');
    }
    navigateAbout() {
      this.navigate('/about/');
    }
    navigateProjects() {
      this.navigate('/projects/');
    }
    navigateResume() {
      this.navigate('/resume/');
    }
    navigateContact() {
      this.navigate('/contact/');
    }

    navigate(path) {
      const router = window.portfolioApp?.router;
      if (router) {
        router.navigate(window.location.origin + path);
      } else {
        window.location.href = path;
      }
    }
  }

  // Performance Utilities
  class PerformanceOptimizer {
    constructor() {
      this.init();
    }

    init() {
      this.preloadCriticalPages();
      this.lazyLoadImages();
      this.prefetchOnHover();
    }

    preloadCriticalPages() {
      const criticalPages = ['/about/', '/projects/', '/resume/'];

      if ('requestIdleCallback' in window) {
        requestIdleCallback(() => {
          criticalPages.forEach(page => this.preloadPage(page));
        });
      }
    }

    preloadPage(url) {
      const link = document.createElement('link');
      link.rel = 'prefetch';
      link.href = url;
      document.head.appendChild(link);
    }

    lazyLoadImages() {
      if ('IntersectionObserver' in window) {
        const imageObserver = new IntersectionObserver(entries => {
          entries.forEach(entry => {
            if (entry.isIntersecting) {
              const img = entry.target;
              if (img.dataset.src) {
                img.src = img.dataset.src;
                img.removeAttribute('data-src');
                imageObserver.unobserve(img);
              }
            }
          });
        });

        document.querySelectorAll('img[data-src]').forEach(img => {
          imageObserver.observe(img);
        });
      }
    }

    prefetchOnHover() {
      const prefetchedUrls = new Set();

      document.addEventListener('mouseover', e => {
        const link = e.target.closest('a');
        if (!link || !this.isInternalLink(link) || prefetchedUrls.has(link.href)) return;

        prefetchedUrls.add(link.href);
        this.preloadPage(link.href);
      });
    }

    isInternalLink(link) {
      return link.hostname === window.location.hostname && !link.href.includes('#');
    }
  }

  // Typing Animation Manager
  class TypingAnimationManager {
    constructor() {
      this.init();
    }

    init() {
      this.addTypingEffects();
      this.addScrollAnimations();
    }

    addTypingEffects() {
      const terminalLines = document.querySelectorAll('.terminal-line .command');
      terminalLines.forEach((element, index) => {
        this.typeWriter(element, index * 100);
      });
    }

    typeWriter(element, delay = 0) {
      if (!element || element.classList.contains('typed')) return;

      const text = element.textContent;
      element.textContent = '';
      element.classList.add('typed');

      setTimeout(() => {
        let i = 0;
        const timer = setInterval(() => {
          if (i < text.length) {
            element.textContent += text.charAt(i);
            i++;
          } else {
            clearInterval(timer);
          }
        }, 50);
      }, delay);
    }

    addScrollAnimations() {
      if ('IntersectionObserver' in window) {
        const observer = new IntersectionObserver(
          entries => {
            entries.forEach(entry => {
              if (entry.isIntersecting) {
                entry.target.classList.add('animate-on-scroll');
                observer.unobserve(entry.target);
              }
            });
          },
          { threshold: 0.1 }
        );

        document.querySelectorAll('.project-section, h2, h3').forEach(el => {
          observer.observe(el);
        });
      }
    }
  }

  // Enhanced Interactions
  class InteractionEnhancer {
    constructor() {
      this.init();
    }

    init() {
      this.addHoverEffects();
      this.addClickFeedback();
      this.enhanceNavigation();
    }

    addHoverEffects() {
      document.querySelectorAll('.project-section, .skill-category').forEach(element => {
        element.classList.add('interactive-element');
      });
    }

    addClickFeedback() {
      document.addEventListener('click', e => {
        const clickable = e.target.closest('button, a, .nav-link');
        if (clickable) {
          clickable.style.transform = 'scale(0.95)';
          setTimeout(() => {
            clickable.style.transform = '';
          }, 100);
        }
      });
    }

    enhanceNavigation() {
      // Add loading indicator for navigation
      document.querySelectorAll('.nav-link').forEach(link => {
        link.addEventListener('click', e => {
          if (!link.classList.contains('nav-active')) {
            link.classList.add('loading-dots');
            setTimeout(() => {
              link.classList.remove('loading-dots');
            }, 2000);
          }
        });
      });
    }
  }

  // Initialize Application
  class PortfolioApp {
    constructor() {
      this.themeManager = new ThemeManager();
      this.router = new SPARouter();
      this.keyboardShortcuts = new KeyboardShortcuts();
      this.performanceOptimizer = new PerformanceOptimizer();
      this.typingAnimationManager = new TypingAnimationManager();
      this.interactionEnhancer = new InteractionEnhancer();
    }
  }

  // Wait for DOM ready
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => {
      window.portfolioApp = new PortfolioApp();
    });
  } else {
    window.portfolioApp = new PortfolioApp();
  }
})();
