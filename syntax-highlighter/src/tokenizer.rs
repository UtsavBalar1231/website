/// UTF-8 safe tokenizer for real-time syntax highlighting
/// Fixed version that handles Unicode characters correctly

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    // Keywords and identifiers
    Keyword,
    Identifier,
    Function,

    // Literals
    String,
    Number,
    Comment,

    // Operators and punctuation
    Operator,
    Punctuation,

    // Special
    Whitespace,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub start: usize,
    pub end: usize,
}

pub struct Tokenizer<'a> {
    input: &'a str,
    chars: std::str::Chars<'a>,
    position: usize, // Current byte position
    current_char: Option<char>,
    language: Language,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    C,
    Bash,
    Makefile,
    Yaml,
    Auto,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str, language: Language) -> Self {
        let mut chars = input.chars();
        let current_char = chars.next();

        Self {
            input,
            chars,
            position: 0,
            current_char,
            language,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.current_char.is_some() {
            if let Some(token) = self.next_token() {
                tokens.push(token);
            }
        }

        tokens
    }

    fn next_token(&mut self) -> Option<Token> {
        let ch = self.current_char?;
        let start = self.position;

        // Skip whitespace but track it for positions
        if ch.is_whitespace() {
            self.skip_whitespace();
            return Some(Token {
                token_type: TokenType::Whitespace,
                start,
                end: self.position,
            });
        }

        // Comments
        if self.is_comment_start() {
            return self.read_comment(start);
        }

        // Strings
        if ch == '"' || ch == '\'' {
            return self.read_string(start, ch);
        }

        // Numbers
        if ch.is_ascii_digit() {
            return self.read_number(start);
        }

        // Identifiers and keywords
        if ch.is_alphabetic() || ch == '_' {
            return self.read_identifier(start);
        }

        // Operators and punctuation
        if self.is_operator_char(ch) {
            return self.read_operator(start);
        }

        // Punctuation characters
        if self.is_punctuation_char(ch) {
            self.advance();
            return Some(Token {
                token_type: TokenType::Punctuation,
                start,
                end: self.position,
            });
        }

        // Default: single character token (including Unicode)
        self.advance();
        Some(Token {
            token_type: TokenType::Unknown,
            start,
            end: self.position,
        })
    }

    fn advance(&mut self) {
        if let Some(ch) = self.current_char {
            self.position += ch.len_utf8();
            self.current_char = self.chars.next();
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.chars.as_str().chars().next()
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn is_comment_start(&self) -> bool {
        match self.language {
            Language::C => {
                if let (Some('/'), Some('/')) = (self.current_char, self.peek_char()) {
                    return true;
                }
                if let (Some('/'), Some('*')) = (self.current_char, self.peek_char()) {
                    return true;
                }
                false
            }
            Language::Bash | Language::Makefile | Language::Yaml => self.current_char == Some('#'),
            Language::Auto => {
                // Try to detect comment style
                self.current_char == Some('#')
                    || (self.current_char == Some('/')
                        && (self.peek_char() == Some('/') || self.peek_char() == Some('*')))
            }
        }
    }

    fn read_comment(&mut self, start: usize) -> Option<Token> {
        match self.language {
            Language::C | Language::Auto => {
                if self.current_char == Some('/') && self.peek_char() == Some('/') {
                    // Line comment
                    while let Some(ch) = self.current_char {
                        if ch == '\n' {
                            break;
                        }
                        self.advance();
                    }
                } else if self.current_char == Some('/') && self.peek_char() == Some('*') {
                    // Block comment
                    self.advance(); // Skip '/'
                    self.advance(); // Skip '*'

                    while self.current_char.is_some() {
                        if self.current_char == Some('*') && self.peek_char() == Some('/') {
                            self.advance(); // Skip '*'
                            self.advance(); // Skip '/'
                            break;
                        }
                        self.advance();
                    }
                }
            }
            _ => {
                // Hash comments
                while let Some(ch) = self.current_char {
                    if ch == '\n' {
                        break;
                    }
                    self.advance();
                }
            }
        }

        Some(Token {
            token_type: TokenType::Comment,
            start,
            end: self.position,
        })
    }

    fn read_string(&mut self, start: usize, quote: char) -> Option<Token> {
        self.advance(); // Skip opening quote

        while let Some(ch) = self.current_char {
            if ch == quote {
                self.advance(); // Skip closing quote
                break;
            }
            if ch == '\\' {
                self.advance(); // Skip escape char
                if self.current_char.is_some() {
                    self.advance(); // Skip escaped char
                }
            } else {
                self.advance();
            }
        }

        Some(Token {
            token_type: TokenType::String,
            start,
            end: self.position,
        })
    }

    fn read_number(&mut self, start: usize) -> Option<Token> {
        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() || ch == '.' || ch == 'x' || ch == 'X' {
                self.advance();
            } else {
                break;
            }
        }

        Some(Token {
            token_type: TokenType::Number,
            start,
            end: self.position,
        })
    }

    fn read_identifier(&mut self, start: usize) -> Option<Token> {
        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }

        // Extract text safely using string slicing
        let text = &self.input[start..self.position];
        let token_type = if self.is_keyword(text) {
            TokenType::Keyword
        } else if self.is_function_call() {
            TokenType::Function
        } else {
            TokenType::Identifier
        };

        Some(Token {
            token_type,
            start,
            end: self.position,
        })
    }

    fn read_operator(&mut self, start: usize) -> Option<Token> {
        let ch = self.current_char?;
        self.advance();

        // Handle multi-character operators
        if let Some(next_ch) = self.current_char {
            match (ch, next_ch) {
                ('=', '=')
                | ('!', '=')
                | ('<', '=')
                | ('>', '=')
                | ('&', '&')
                | ('|', '|')
                | ('+', '+')
                | ('-', '-')
                | ('<', '<')
                | ('>', '>') => {
                    self.advance();
                }
                _ => {}
            }
        }

        Some(Token {
            token_type: if self.is_operator_char(ch) {
                TokenType::Operator
            } else {
                TokenType::Punctuation
            },
            start,
            end: self.position,
        })
    }

    fn is_keyword(&self, text: &str) -> bool {
        use crate::languages::{
            bash::BashLanguage, c::CLanguage, makefile::MakefileLanguage, yaml::YamlLanguage,
        };

        match self.language {
            Language::C => CLanguage::is_keyword(text),
            Language::Bash => BashLanguage::is_keyword(text),
            Language::Makefile => MakefileLanguage::is_keyword(text),
            Language::Yaml => YamlLanguage::is_keyword(text),
            Language::Auto => {
                CLanguage::is_keyword(text)
                    || BashLanguage::is_keyword(text)
                    || YamlLanguage::is_keyword(text)
            }
        }
    }

    fn is_function_call(&self) -> bool {
        // Look ahead to see if next non-whitespace char is '('
        let remaining = &self.input[self.position..];
        for ch in remaining.chars() {
            if ch.is_whitespace() {
                continue;
            }
            return ch == '(';
        }
        false
    }

    fn is_operator_char(&self, ch: char) -> bool {
        matches!(
            ch,
            '+' | '-'
                | '*'
                | '/'
                | '%'
                | '='
                | '!'
                | '<'
                | '>'
                | '&'
                | '|'
                | '^'
                | '~'
                | '?'
                | ':'
                | '.'
                | ','
        )
    }

    fn is_punctuation_char(&self, ch: char) -> bool {
        matches!(ch, '(' | ')' | '{' | '}' | '[' | ']' | ';')
    }
}
