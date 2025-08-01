import { nodeResolve } from '@rollup/plugin-node-resolve';
import terser from '@rollup/plugin-terser';

export default {
  input: 'src/js/main.js',
  output: {
    file: '_site/js/bundle.js',
    format: 'iife',
    name: 'Portfolio'
  },
  plugins: [
    nodeResolve(),
    terser({
      compress: {
        drop_console: true,
        drop_debugger: true
      },
      mangle: true,
      format: {
        comments: false
      }
    })
  ]
};
