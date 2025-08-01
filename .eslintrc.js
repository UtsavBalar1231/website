module.exports = {
  env: {
    browser: true,
    es2021: true,
    serviceworker: true
  },
  extends: [
    'eslint:recommended'
  ],
  parserOptions: {
    ecmaVersion: 'latest',
    sourceType: 'module'
  },
  rules: {
    'no-console': 'warn',
    'no-unused-vars': 'error',
    'no-undef': 'error',
    'prefer-const': 'error',
    'no-var': 'error',
    'semi': ['error', 'always'],
    'quotes': ['error', 'single'],
    'indent': ['error', 2],
    'comma-dangle': ['error', 'never'],
    'eol-last': ['error', 'always']
  },
  globals: {
    'self': 'readonly',
    'caches': 'readonly',
    'clients': 'readonly',
    'registration': 'readonly'
  }
};