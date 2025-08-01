// Ultra-lightweight WASM syntax highlighter integration
// Target: <300 bytes, compatible with existing ThemeManager

class SyntaxHighlighter {
  constructor() {
    this.wasmModule = null;
    this.isLoaded = false;
    this.pendingBlocks = [];
  }

  async init() {
    if (this.isLoaded) return;

    try {
      // Load WASM module
      const wasmPath = '/js/syntax_highlighter.js';
      const wasmModule = await import(wasmPath);

      // Initialize WASM
      await wasmModule.default();

      // Store the module functions
      this.wasmModule = wasmModule;
      this.isLoaded = true;

      // Process any pending blocks
      this.processPendingBlocks();

      console.log('✨ Syntax highlighter loaded');
    } catch (error) {
      console.warn('⚠️ Syntax highlighter failed to load:', error);
    }
  }

  highlightAll() {
    if (this.isLoaded && this.wasmModule) {
      try {
        this.wasmModule.highlight_all_code_blocks();
      } catch (error) {
        console.warn('Syntax highlighting error:', error);
      }
    } else {
      this.pendingBlocks.push(() => this.highlightAll());
    }
  }

  processPendingBlocks() {
    while (this.pendingBlocks.length > 0) {
      const fn = this.pendingBlocks.shift();
      fn();
    }
  }

  // Theme change handler - re-highlight when theme changes
  onThemeChange() {
    if (this.isLoaded) {
      // Small delay to ensure CSS has loaded
      setTimeout(() => this.highlightAll(), 50);
    }
  }
}

// Create global instance
window.syntaxHighlighter = new SyntaxHighlighter();

// Auto-initialize when DOM is ready
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', () => {
    window.syntaxHighlighter.init().then(() => {
      window.syntaxHighlighter.highlightAll();
    });
  });
} else {
  window.syntaxHighlighter.init().then(() => {
    window.syntaxHighlighter.highlightAll();
  });
}

// Integration with existing ThemeManager
if (window.portfolioApp?.themeManager) {
  const originalSetTheme = window.portfolioApp.themeManager.setTheme;
  window.portfolioApp.themeManager.setTheme = function (theme) {
    originalSetTheme.call(this, theme);
    window.syntaxHighlighter.onThemeChange();
  };
}

// Expose global function for manual highlighting (debugging/testing)
window.highlightAllCodeBlocks = () => {
  if (window.syntaxHighlighter.isLoaded) {
    window.syntaxHighlighter.highlightAll();
  } else {
    console.warn('Syntax highlighter not yet loaded');
  }
};
