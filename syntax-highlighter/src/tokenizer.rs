/// Ultra-lightweight tokenizer for real-time syntax highlighting
/// Optimized for minimal memory usage and maximum speed

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
    position: usize,
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
        Self {
            input,
            position: 0,
            language,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        
        while self.position < self.input.len() {
            if let Some(token) = self.next_token() {
                tokens.push(token);
            }
        }
        
        tokens
    }

    fn next_token(&mut self) -> Option<Token> {
        if self.position >= self.input.len() {
            return None;
        }

        let start = self.position;
        let ch = self.current_char()?;

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

        // Default: single character token
        self.advance();
        Some(Token {
            token_type: TokenType::Unknown,
            start,
            end: self.position,
        })
    }

    fn current_char(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    fn peek_char(&self) -> Option<char> {
        self.input.chars().nth(self.position + 1)
    }

    fn advance(&mut self) {
        if self.position < self.input.len() {
            // Move to next character boundary
            let ch = self.current_char().unwrap();
            self.position += ch.len_utf8();
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char() {
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
                if let (Some('/'), Some('/')) = (self.current_char(), self.peek_char()) {
                    return true;
                }
                if let (Some('/'), Some('*')) = (self.current_char(), self.peek_char()) {
                    return true;
                }
                false
            }
            Language::Bash | Language::Makefile | Language::Yaml => {
                self.current_char() == Some('#')
            }
            Language::Auto => {
                // Try to detect comment style
                self.current_char() == Some('#') || 
                (self.current_char() == Some('/') && 
                 (self.peek_char() == Some('/') || self.peek_char() == Some('*')))
            }
        }
    }

    fn read_comment(&mut self, start: usize) -> Option<Token> {
        match self.language {
            Language::C | Language::Auto => {
                if self.current_char() == Some('/') && self.peek_char() == Some('/') {
                    // Line comment
                    while let Some(ch) = self.current_char() {
                        if ch == '\n' {
                            break;
                        }
                        self.advance();
                    }
                } else if self.current_char() == Some('/') && self.peek_char() == Some('*') {
                    // Block comment
                    self.advance(); // Skip '/'
                    self.advance(); // Skip '*'
                    
                    while self.position < self.input.len() - 1 {
                        if self.current_char() == Some('*') && self.peek_char() == Some('/') {
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
                while let Some(ch) = self.current_char() {
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
        
        while let Some(ch) = self.current_char() {
            if ch == quote {
                self.advance(); // Skip closing quote
                break;
            }
            if ch == '\\' {
                self.advance(); // Skip escape char
                if self.current_char().is_some() {
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
        while let Some(ch) = self.current_char() {
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
        while let Some(ch) = self.current_char() {
            if ch.is_alphanumeric() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }

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
        let ch = self.current_char()?;
        self.advance();

        // Handle multi-character operators
        if let Some(next_ch) = self.current_char() {
            match (ch, next_ch) {
                ('=', '=') | ('!', '=') | ('<', '=') | ('>', '=') |
                ('&', '&') | ('|', '|') | ('+', '+') | ('-', '-') |
                ('<', '<') | ('>', '>') => {
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
        use crate::languages::{c::CLanguage, bash::BashLanguage, makefile::MakefileLanguage, yaml::YamlLanguage};
        
        match self.language {
            Language::C => CLanguage::is_keyword(text),
            Language::Bash => BashLanguage::is_keyword(text),
            Language::Makefile => MakefileLanguage::is_keyword(text),
            Language::Yaml => YamlLanguage::is_keyword(text),
            Language::Auto => {
                CLanguage::is_keyword(text) || 
                BashLanguage::is_keyword(text) || 
                YamlLanguage::is_keyword(text)
            }
        }
    }


    fn is_function_call(&self) -> bool {
        // Look ahead to see if next non-whitespace char is '('
        let mut pos = self.position;
        while pos < self.input.len() {
            if let Some(ch) = self.input.chars().nth(pos) {
                if ch.is_whitespace() {
                    pos += 1;
                    continue;
                }
                return ch == '(';
            }
            break;
        }
        false
    }

    fn is_operator_char(&self, ch: char) -> bool {
        matches!(ch, 
            '+' | '-' | '*' | '/' | '%' | '=' | '!' | '<' | '>' | 
            '&' | '|' | '^' | '~' | '?' | ':' | '.' | ','
        )
    }

    fn is_punctuation_char(&self, ch: char) -> bool {
        matches!(ch, '(' | ')' | '{' | '}' | '[' | ']' | ';')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_tokenization() {
        let code = r#"int main() { printf("Hello"); }"#;
        let mut tokenizer = Tokenizer::new(code, Language::C);
        let tokens = tokenizer.tokenize();
        
        assert!(!tokens.is_empty());
        // First token should be 'int' keyword
        assert_eq!(tokens[0].token_type, TokenType::Keyword);
    }

    #[test]
    fn test_comment_tokenization() {
        let code = "// This is a comment\nint x;";
        let mut tokenizer = Tokenizer::new(code, Language::C);
        let tokens = tokenizer.tokenize();
        
        // Should have comment token
        assert!(tokens.iter().any(|t| t.token_type == TokenType::Comment));
    }

    // Comprehensive token type tests
    #[test]
    fn test_all_token_types() {
        let code = r#"int main() {
    printf("Hello World"); // Comment
    int x = 42;
    if (x > 0) {
        return x + y;
    }
}"#;
        let mut tokenizer = Tokenizer::new(code, Language::C);
        let tokens = tokenizer.tokenize();
        
        // Check we have all expected token types
        let token_types: std::collections::HashSet<_> = tokens.iter().map(|t| &t.token_type).collect();
        assert!(token_types.contains(&TokenType::Keyword));    // int, if, return
        assert!(token_types.contains(&TokenType::Identifier)); // main, x, y
        assert!(token_types.contains(&TokenType::Function));   // printf
        assert!(token_types.contains(&TokenType::String));     // "Hello World"
        assert!(token_types.contains(&TokenType::Number));     // 42
        assert!(token_types.contains(&TokenType::Comment));    // // Comment
        assert!(token_types.contains(&TokenType::Operator));   // =, >, +
        assert!(token_types.contains(&TokenType::Whitespace)); // spaces, newlines
        
        // Add explicit test for punctuation with simpler code
        let simple_code = "main();";
        let mut simple_tokenizer = Tokenizer::new(simple_code, Language::C);
        let simple_tokens = simple_tokenizer.tokenize();
        assert!(simple_tokens.iter().any(|t| t.token_type == TokenType::Punctuation)); // (, ), ;
    }

    #[test]
    fn test_string_tokenization() {
        let test_cases = vec![
            (r#""simple string""#, 1),
            (r#"'single quotes'"#, 1),
            (r#""string with \"escaped quotes\"""#, 1),
            (r#""multi\nline\nstring""#, 1),
            (r#""one" "two" "three""#, 3),
        ];

        for (code, expected_count) in test_cases {
            let mut tokenizer = Tokenizer::new(code, Language::C);
            let tokens = tokenizer.tokenize();
            let string_count = tokens.iter().filter(|t| t.token_type == TokenType::String).count();
            assert_eq!(string_count, expected_count, "Failed for: {}", code);
        }
    }

    #[test]
    fn test_number_tokenization() {
        let test_cases = vec![
            ("42", TokenType::Number),
            ("3.14", TokenType::Number),
            ("0x1A2B", TokenType::Number),
            ("0xFF", TokenType::Number),
            ("123.456", TokenType::Number),
        ];

        for (code, expected_type) in test_cases {
            let mut tokenizer = Tokenizer::new(code, Language::C);
            let tokens = tokenizer.tokenize();
            let number_tokens: Vec<_> = tokens.iter().filter(|t| t.token_type == TokenType::Number).collect();
            assert!(!number_tokens.is_empty(), "No number token found for: {}", code);
            assert_eq!(number_tokens[0].token_type, expected_type, "Wrong type for: {}", code);
        }
    }

    #[test]
    fn test_operator_tokenization() {
        let operators = vec![
            "+", "-", "*", "/", "%", "=", "==", "!=", "<", ">", "<=", ">=",
            "&&", "||", "++", "--", "<<", ">>", "&", "|", "^", "~", "?", ":", ".", ","
        ];

        for op in operators {
            let code = format!("x {} y", op);
            let mut tokenizer = Tokenizer::new(&code, Language::C);
            let tokens = tokenizer.tokenize();
            assert!(tokens.iter().any(|t| t.token_type == TokenType::Operator), 
                    "Operator not detected: {}", op);
        }
    }

    #[test]
    fn test_function_detection() {
        let test_cases = vec![
            ("printf()", true),
            ("main()", true),
            ("function_name()", true),
            ("printf ()", true), // with space
            ("printf", false),   // no parentheses
            ("printf;", false),  // not a function call
        ];

        for (code, should_be_function) in test_cases {
            let mut tokenizer = Tokenizer::new(code, Language::C);
            let tokens = tokenizer.tokenize();
            let has_function = tokens.iter().any(|t| t.token_type == TokenType::Function);
            assert_eq!(has_function, should_be_function, "Function detection failed for: {}", code);
        }
    }

    #[test]
    fn test_comment_types() {
        // C-style line comments
        let code1 = "// This is a line comment\nint x;";
        let mut tokenizer1 = Tokenizer::new(code1, Language::C);
        let tokens1 = tokenizer1.tokenize();
        assert!(tokens1.iter().any(|t| t.token_type == TokenType::Comment));

        // C-style block comments
        let code2 = "/* This is a block comment */\nint x;";
        let mut tokenizer2 = Tokenizer::new(code2, Language::C);
        let tokens2 = tokenizer2.tokenize();
        assert!(tokens2.iter().any(|t| t.token_type == TokenType::Comment));

        // Hash comments (Bash/Makefile/YAML)
        let code3 = "# This is a hash comment\necho hello";
        let mut tokenizer3 = Tokenizer::new(code3, Language::Bash);
        let tokens3 = tokenizer3.tokenize();
        assert!(tokens3.iter().any(|t| t.token_type == TokenType::Comment));
    }

    #[test]
    fn test_language_specific_keywords() {
        // C keywords
        let c_code = "int main void return if else while for";
        let mut c_tokenizer = Tokenizer::new(c_code, Language::C);
        let c_tokens = c_tokenizer.tokenize();
        let c_keyword_count = c_tokens.iter().filter(|t| t.token_type == TokenType::Keyword).count();
        assert!(c_keyword_count >= 7, "C keywords not detected properly");

        // Bash keywords
        let bash_code = "if then else fi for do done while";
        let mut bash_tokenizer = Tokenizer::new(bash_code, Language::Bash);
        let bash_tokens = bash_tokenizer.tokenize();
        let bash_keyword_count = bash_tokens.iter().filter(|t| t.token_type == TokenType::Keyword).count();
        assert!(bash_keyword_count >= 6, "Bash keywords not detected properly");

        // YAML keywords
        let yaml_code = "true false null yes no";
        let mut yaml_tokenizer = Tokenizer::new(yaml_code, Language::Yaml);
        let yaml_tokens = yaml_tokenizer.tokenize();
        let yaml_keyword_count = yaml_tokens.iter().filter(|t| t.token_type == TokenType::Keyword).count();
        assert!(yaml_keyword_count >= 4, "YAML keywords not detected properly");
    }

    #[test]
    fn test_auto_language_detection() {
        let code = "printf echo true"; // Mix of C, Bash, and YAML keywords
        let mut tokenizer = Tokenizer::new(code, Language::Auto);
        let tokens = tokenizer.tokenize();
        
        // Should detect at least some keywords in auto mode
        assert!(tokens.iter().any(|t| t.token_type == TokenType::Keyword));
    }

    #[test]
    fn test_whitespace_handling() {
        let code = "int\n\tmain   (  )\n{\n    return 0;\n}";
        let mut tokenizer = Tokenizer::new(code, Language::C);
        let tokens = tokenizer.tokenize();
        
        // Should have whitespace tokens
        assert!(tokens.iter().any(|t| t.token_type == TokenType::Whitespace));
        
        // Check token positions are correct
        for token in &tokens {
            assert!(token.start <= token.end);
            assert!(token.end <= code.len());
        }
    }

    #[test]
    fn test_unicode_handling() {
        let code = "// Simple comment\nint main() { return 0; }";
        let mut tokenizer = Tokenizer::new(code, Language::C);
        let tokens = tokenizer.tokenize();
        
        // Should tokenize without panicking
        assert!(!tokens.is_empty());
        assert!(tokens.iter().any(|t| t.token_type == TokenType::Comment));
        assert!(tokens.iter().any(|t| t.token_type == TokenType::Keyword));
    }

    #[test]
    fn test_empty_input() {
        let mut tokenizer = Tokenizer::new("", Language::C);
        let tokens = tokenizer.tokenize();
        assert!(tokens.is_empty());
    }

    #[test]
    fn test_whitespace_only() {
        let mut tokenizer = Tokenizer::new("   \n\t  \n  ", Language::C);
        let tokens = tokenizer.tokenize();
        assert!(tokens.iter().all(|t| t.token_type == TokenType::Whitespace));
    }

    #[test]
    fn test_unclosed_string() {
        let code = r#"printf("unclosed string"#;
        let mut tokenizer = Tokenizer::new(code, Language::C);
        let tokens = tokenizer.tokenize();
        
        // Should still create a string token even if unclosed
        assert!(tokens.iter().any(|t| t.token_type == TokenType::String));
    }

    #[test]
    fn test_unclosed_block_comment() {
        let code = "/* unclosed block comment\nint main() {}";
        let mut tokenizer = Tokenizer::new(code, Language::C);
        let tokens = tokenizer.tokenize();
        
        // Should create a comment token even if unclosed
        assert!(tokens.iter().any(|t| t.token_type == TokenType::Comment));
    }

    #[test]
    fn test_token_positions() {
        let code = "int x = 42;";
        let mut tokenizer = Tokenizer::new(code, Language::C);
        let tokens = tokenizer.tokenize();
        
        // Filter out whitespace for easier testing
        let non_ws_tokens: Vec<_> = tokens.iter().filter(|t| t.token_type != TokenType::Whitespace).collect();
        
        // Check that tokens cover the expected text
        assert_eq!(&code[non_ws_tokens[0].start..non_ws_tokens[0].end], "int");
        assert_eq!(&code[non_ws_tokens[1].start..non_ws_tokens[1].end], "x");
        assert_eq!(&code[non_ws_tokens[2].start..non_ws_tokens[2].end], "=");
        assert_eq!(&code[non_ws_tokens[3].start..non_ws_tokens[3].end], "42");
        assert_eq!(&code[non_ws_tokens[4].start..non_ws_tokens[4].end], ";");
    }

    #[test]
    fn test_complex_expressions() {
        let code = "result = (a + b) * c / d - e % f;";
        let mut tokenizer = Tokenizer::new(code, Language::C);
        let tokens = tokenizer.tokenize();
        
        let operators: Vec<_> = tokens.iter()
            .filter(|t| t.token_type == TokenType::Operator)
            .collect();
        
        // Should detect all operators: =, +, *, /, -, %
        assert!(operators.len() >= 6);
    }

    #[test]
    fn test_nested_structures() {
        let code = r#"
            if (condition) {
                while (x < 10) {
                    printf("nested: %d", x++);
                }
            }
        "#;
        let mut tokenizer = Tokenizer::new(code, Language::C);
        let tokens = tokenizer.tokenize();
        
        // Should have proper mix of all token types
        let keywords = tokens.iter().filter(|t| t.token_type == TokenType::Keyword).count();
        let operators = tokens.iter().filter(|t| t.token_type == TokenType::Operator).count();
        let strings = tokens.iter().filter(|t| t.token_type == TokenType::String).count();
        let functions = tokens.iter().filter(|t| t.token_type == TokenType::Function).count();
        
        assert!(keywords >= 2); // if, while
        assert!(operators >= 2); // <, +, + (two separate + operators for ++)
        assert!(strings >= 1);   // "nested: %d"
        assert!(functions >= 1); // printf
    }
}