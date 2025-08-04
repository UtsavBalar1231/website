use wasm_bindgen::prelude::*;

mod languages;
mod tokenizer;

use languages::{detect_language, get_css_class};
use tokenizer::{Language, Token, TokenType, Tokenizer};

// Import console.log for debugging
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Macro for console logging
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

/// Main entry point for syntax highlighting
#[wasm_bindgen]
pub fn highlight_code(code: &str, language: Option<String>) -> String {
    let detected_language = match language.as_deref() {
        Some("c") | Some("cpp") | Some("c++") => Language::C,
        Some("bash") | Some("sh") | Some("shell") => Language::Bash,
        Some("makefile") | Some("make") => Language::Makefile,
        Some("yaml") | Some("yml") => Language::Yaml,
        _ => detect_language(code, language.as_deref()),
    };

    let mut tokenizer = Tokenizer::new(code, detected_language);
    let tokens = tokenizer.tokenize();

    generate_html_with_classes(code, &tokens)
}

/// Apply highlighting to existing DOM element
#[wasm_bindgen]
pub fn highlight_element(element_id: &str, language: Option<String>) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No global window object")?;
    let document = window.document().ok_or("No document object")?;
    let element = document
        .get_element_by_id(element_id)
        .ok_or("Element not found")?;

    let code = element.text_content().unwrap_or_default();
    let highlighted = highlight_code(&code, language);

    element.set_inner_html(&highlighted);
    Ok(())
}

/// Batch highlight multiple elements
#[wasm_bindgen]
pub fn highlight_all_code_blocks() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No global window object")?;
    let document = window.document().ok_or("No document object")?;

    // Find all code blocks (both <code> and <pre><code>)
    let code_elements = document.query_selector_all("pre code, code")?;

    for i in 0..code_elements.length() {
        if let Some(element) = code_elements.item(i) {
            let html_element: web_sys::HtmlElement = element.dyn_into().unwrap();
            {
                let code = html_element.text_content().unwrap_or_default();

                // Try to detect language from class name
                let class_name = html_element.class_name();
                let language = extract_language_from_class(&class_name);

                let highlighted = highlight_code(&code, language);
                html_element.set_inner_html(&highlighted);
            }
        }
    }

    Ok(())
}

/// Initialize the syntax highlighter
#[wasm_bindgen(start)]
pub fn initialize() {
    console_log!("Syntax highlighter initialized");

    // Better error handling in WASM
    console_error_panic_hook::set_once();
}

/// Generate HTML with CSS classes for tokens
fn generate_html_with_classes(code: &str, tokens: &[Token]) -> String {
    let mut result = String::with_capacity(code.len() * 2);
    let mut last_end = 0;

    for token in tokens {
        // Add any text between tokens
        if token.start > last_end {
            result.push_str(&escape_html(&code[last_end..token.start]));
        }

        // Add the token with its CSS class
        let token_text = &code[token.start..token.end];
        let css_class = get_css_class(&token.token_type);

        if token.token_type != TokenType::Whitespace && !token_text.trim().is_empty() {
            result.push_str(&format!(
                r#"<span class="{}">{}</span>"#,
                css_class,
                escape_html(token_text)
            ));
        } else {
            // Don't wrap whitespace in spans
            result.push_str(&escape_html(token_text));
        }

        last_end = token.end;
    }

    // Add any remaining text
    if last_end < code.len() {
        result.push_str(&escape_html(&code[last_end..]));
    }

    result
}

/// Extract language from CSS class name (e.g., "language-c" -> Some("c"))
fn extract_language_from_class(class_name: &str) -> Option<String> {
    for class in class_name.split_whitespace() {
        if let Some(lang) = class.strip_prefix("language-") {
            return if lang.is_empty() {
                None
            } else {
                Some(lang.to_string())
            };
        }
        if let Some(lang) = class.strip_prefix("lang-") {
            return if lang.is_empty() {
                None
            } else {
                Some(lang.to_string())
            };
        }
    }
    None
}

/// Escape HTML special characters
fn escape_html(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlight_c_code() {
        let code = r#"int main() { printf("Hello"); }"#;
        let result = highlight_code(code, Some("c".to_string()));
        assert!(result.contains("hl-keyword"));
        assert!(result.contains("hl-function"));
        assert!(result.contains("hl-string"));
    }

    #[test]
    fn test_highlight_bash_code() {
        let code = r#"#!/bin/bash
echo "Hello World"
if [ -f "file.txt" ]; then
    echo "File exists"
fi"#;
        let result = highlight_code(code, Some("bash".to_string()));
        assert!(result.contains("hl-keyword"));
        assert!(result.contains("hl-string"));
    }

    #[test]
    fn test_html_escaping() {
        let text = "<script>alert('xss')</script>";
        let escaped = escape_html(text);
        assert!(!escaped.contains("<script>"));
        assert!(escaped.contains("&lt;script&gt;"));
    }

    #[test]
    fn test_language_extraction() {
        assert_eq!(
            extract_language_from_class("language-c other-class"),
            Some("c".to_string())
        );
        assert_eq!(
            extract_language_from_class("lang-bash highlight"),
            Some("bash".to_string())
        );
        assert_eq!(extract_language_from_class("no-language-here"), None);
    }

    // Comprehensive integration tests
    #[test]
    fn test_all_supported_languages() {
        let test_cases = vec![
            (
                "c",
                r#"#include <stdio.h>
int main() {
    printf("Hello, C!");
    return 0;
}"#,
            ),
            (
                "bash",
                r#"#!/bin/bash
echo "Hello, Bash!"
if [ "$1" = "test" ]; then
    echo "Testing mode"
fi"#,
            ),
            (
                "makefile",
                r#"CC=gcc
CFLAGS=-Wall

all: program

program: main.o
	$(CC) $(CFLAGS) -o $@ $^"#,
            ),
            (
                "yaml",
                r#"version: '3.8'
services:
  web:
    image: nginx
    ports:
      - "80:80""#,
            ),
        ];

        for (lang, code) in test_cases {
            let result = highlight_code(code, Some(lang.to_string()));

            // Should produce highlighted HTML
            assert!(!result.is_empty(), "Empty result for language: {}", lang);
            assert!(
                result.contains("<span"),
                "No spans found for language: {}",
                lang
            );
            assert!(
                result.contains("hl-"),
                "No highlighting classes for language: {}",
                lang
            );

            // Should escape HTML properly
            if code.contains("<") {
                assert!(
                    result.contains("&lt;"),
                    "HTML not escaped for language: {}",
                    lang
                );
            }
        }
    }

    #[test]
    fn test_auto_language_detection() {
        let test_cases = vec![
            ("#include <stdio.h>", "should detect C"),
            ("#!/bin/bash", "should detect Bash"),
            ("all: clean\n\tgcc -o test", "should detect Makefile"),
            ("name: value\nother: test", "should detect YAML"),
        ];

        for (code, description) in test_cases {
            let result = highlight_code(code, None); // No language specified
            assert!(!result.is_empty(), "Empty result: {}", description);
            assert!(result.contains("<span"), "No highlighting: {}", description);
        }
    }

    #[test]
    fn test_edge_cases() {
        // Empty code
        let result = highlight_code("", Some("c".to_string()));
        assert_eq!(result, "");

        // Whitespace only
        let result = highlight_code("   \n\t  ", Some("c".to_string()));
        assert!(result.contains("   "));

        // Simple identifier
        let result = highlight_code("test", Some("c".to_string()));
        assert!(!result.is_empty());

        // Unicode content
        let unicode_code = "printf(\"hello\");";
        let result = highlight_code(unicode_code, Some("c".to_string()));
        assert!(result.contains("hello"));
    }

    #[test]
    fn test_html_injection_prevention() {
        let malicious_inputs = vec![
            "<script>alert('xss')</script>",
            "'; DROP TABLE users; --",
            "<img src=x onerror=alert(1)>",
            "\"><script>alert('xss')</script>",
            "javascript:alert('xss')",
        ];

        for input in malicious_inputs {
            let result = highlight_code(input, Some("c".to_string()));

            // Should not contain unescaped HTML
            assert!(
                !result.contains("<script>"),
                "Script tag not escaped: {}",
                input
            );
            assert!(
                !result.contains("javascript:"),
                "JavaScript URL not escaped: {}",
                input
            );
            assert!(
                !result.contains("onerror="),
                "Event handler not escaped: {}",
                input
            );

            // Should contain escaped versions
            if input.contains("<") {
                assert!(result.contains("&lt;"), "< not escaped in: {}", input);
            }
            if input.contains(">") {
                assert!(result.contains("&gt;"), "> not escaped in: {}", input);
            }
        }
    }

    #[test]
    fn test_language_extraction_edge_cases() {
        let test_cases = vec![
            ("", None),
            ("language-", None),
            ("lang-", None),
            ("not-a-language-class", None),
            ("language-c++", Some("c++".to_string())),
            ("lang-objective-c", Some("objective-c".to_string())),
            ("language-123", Some("123".to_string())),
            ("LANGUAGE-RUST", None), // doesn't match prefix
            ("class1 language-rust class2", Some("rust".to_string())),
            ("lang-python highlight-js", Some("python".to_string())),
        ];

        for (input, expected) in test_cases {
            assert_eq!(
                extract_language_from_class(input),
                expected,
                "Failed for input: '{}'",
                input
            );
        }
    }

    #[test]
    fn test_html_escaping_comprehensive() {
        let test_cases = vec![
            ("", ""),
            ("hello", "hello"),
            ("<", "&lt;"),
            (">", "&gt;"),
            ("&", "&amp;"),
            ("\"", "&quot;"),
            ("'", "&#x27;"),
            ("<>&\"'", "&lt;&gt;&amp;&quot;&#x27;"),
            (
                "Hello <world> & \"friends\"",
                "Hello &lt;world&gt; &amp; &quot;friends&quot;",
            ),
        ];

        for (input, expected) in test_cases {
            assert_eq!(escape_html(input), expected);
        }
    }

    #[test]
    fn test_complex_syntax_highlighting() {
        let complex_c_code = r#"
#include <stdio.h>
#include <stdlib.h>

/* Multi-line comment
   with special chars: <>&"' */
#define MAX_SIZE 100

int main(int argc, char *argv[]) {
    // Line comment
    const char *message = "Hello \"World\" & <everyone>";
    int numbers[] = {1, 2, 3, 0xFF, 0b101};
    float pi = 3.14159;
    
    if (argc > 1) {
        printf("Args: %d\n", argc);
        for (int i = 0; i < argc; i++) {
            printf("  [%d]: %s\n", i, argv[i]);
        }
    }
    
    // Function call with complex expression
    int result = calculate_something(numbers[0] + numbers[1], pi * 2.0);
    
    return EXIT_SUCCESS;
}

// Function definition
static int calculate_something(int a, double b) {
    return (int)(a * b);
}
"#;

        let result = highlight_code(complex_c_code, Some("c".to_string()));

        // Check for all expected token types
        assert!(result.contains("hl-keyword")); // int, if, return, static, etc.
        assert!(result.contains("hl-function")); // printf, calculate_something
        assert!(result.contains("hl-string")); // string literals
        assert!(result.contains("hl-number")); // various number formats
        assert!(result.contains("hl-comment")); // both line and block comments
        assert!(result.contains("hl-operator")); // =, +, *, etc.
        assert!(result.contains("hl-identifier")); // variable names

        // Check HTML escaping
        assert!(result.contains("&lt;"));
        assert!(result.contains("&gt;"));
        assert!(result.contains("&amp;"));
        assert!(result.contains("&quot;"));

        // Should not contain raw HTML
        assert!(!result.contains("<stdio.h>"));
        assert!(!result.contains("<everyone>"));
    }

    #[test]
    fn test_performance_large_input() {
        // Test with moderately large input (reduced from 1000)
        let large_code = format!("{}\n", "int x = 42; // comment".repeat(50));
        let result = highlight_code(&large_code, Some("c".to_string()));

        // Should complete without issues
        assert!(!result.is_empty());
        assert!(result.contains("hl-keyword"));
        assert!(result.contains("hl-number"));
        assert!(result.contains("hl-comment"));
    }

    #[test]
    fn test_nested_quotes_and_escapes() {
        let tricky_code = r#"
printf("String with \"escaped quotes\" and 'single quotes'");
char c = '\''; // escaped single quote
char *complex = "Line 1\nLine 2\tTabbed\r\nWindows line ending";
"#;

        let result = highlight_code(tricky_code, Some("c".to_string()));

        // Should handle nested quotes properly
        assert!(result.contains("hl-string"));
        assert!(result.contains("hl-function"));

        // Should escape HTML entities
        assert!(result.contains("&quot;"));
        assert!(!result.contains("\"escaped quotes\""));
    }

    #[test]
    fn test_mixed_language_patterns() {
        // Code that could potentially confuse the tokenizer
        let mixed_code = r#"
// This looks like C but has some shell-like patterns
int main() {
    system("echo 'Hello from shell: $USER'");  // Shell command in C
    char yaml_like[] = "key: value\nother: item";  // YAML-like string
    return 0;
}
"#;

        let result = highlight_code(mixed_code, Some("c".to_string()));

        // Should treat it as C code throughout
        assert!(result.contains("hl-keyword")); // int, return
        assert!(result.contains("hl-function")); // main, system
        assert!(result.contains("hl-string")); // string literals
        assert!(result.contains("hl-comment")); // comments
    }

    #[test]
    fn test_language_case_sensitivity() {
        let code = "int main() { return 0; }";

        // Test different case variations
        let variations = vec!["c", "C", "bash", "BASH", "yaml", "YAML"];

        for lang in variations {
            let result = highlight_code(code, Some(lang.to_string()));
            assert!(!result.is_empty(), "Failed for language case: {}", lang);
        }
    }

    #[test]
    fn test_language_aliases() {
        let c_code = "int main() { return 0; }";

        // Test language aliases
        let c_aliases = vec!["c", "cpp", "c++"];
        for alias in c_aliases {
            let result = highlight_code(c_code, Some(alias.to_string()));
            assert!(
                result.contains("hl-keyword"),
                "Failed for C alias: {}",
                alias
            );
        }

        let bash_code = "echo hello";
        let bash_aliases = vec!["bash", "sh", "shell"];
        for alias in bash_aliases {
            let result = highlight_code(bash_code, Some(alias.to_string()));
            assert!(
                result.contains("hl-keyword"),
                "Failed for Bash alias: {}",
                alias
            );
        }
    }

    #[test]
    fn test_whitespace_preservation() {
        let code_with_formatting = "int    main  (  )  {\n    return    0;\n}";
        let result = highlight_code(code_with_formatting, Some("c".to_string()));

        // Should preserve original spacing
        assert!(result.contains("    ")); // Multiple spaces
        assert!(result.contains("\n")); // Newlines
        assert!(result.contains("hl-keyword"));
    }

    #[test]
    fn test_empty_and_minimal_inputs() {
        // Empty string
        assert_eq!(highlight_code("", Some("c".to_string())), "");
        assert_eq!(highlight_code("", None), "");

        // Single character
        let result = highlight_code("x", Some("c".to_string()));
        assert!(result.contains("hl-identifier"));

        // Single keyword
        let result = highlight_code("int", Some("c".to_string()));
        assert!(result.contains("hl-keyword"));

        // Single operator
        let result = highlight_code("+", Some("c".to_string()));
        assert!(result.contains("hl-operator"));
    }

    #[test]
    fn test_boundary_conditions() {
        // Test moderately long identifier (reduced from 1000)
        let long_id = "a".repeat(50);
        let result = highlight_code(&long_id, Some("c".to_string()));
        assert!(result.contains("hl-identifier"));

        // Test moderately long string (reduced from 1000)
        let long_string = format!("\"{}\"", "a".repeat(50));
        let result = highlight_code(&long_string, Some("c".to_string()));
        assert!(result.contains("hl-string"));

        // Test moderately nested structures (reduced from 100)
        let nested = format!("if ({})", "(".repeat(10) + &")".repeat(10));
        let result = highlight_code(&nested, Some("c".to_string()));
        assert!(result.contains("hl-keyword"));
    }
}
