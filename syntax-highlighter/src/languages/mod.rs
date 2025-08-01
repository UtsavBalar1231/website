pub mod c;
pub mod bash;
pub mod makefile;
pub mod yaml;

use crate::tokenizer::{Language, TokenType};

/// Language detection based on file extension or content analysis
pub fn detect_language(content: &str, filename: Option<&str>) -> Language {
    // Try filename first
    if let Some(name) = filename {
        let name = name.to_lowercase();
        if name.ends_with(".c") || name.ends_with(".h") || name.ends_with(".cpp") || name.ends_with(".hpp") {
            return Language::C;
        }
        if name.ends_with(".sh") || name.ends_with(".bash") || name == "bashrc" || name == ".bashrc" {
            return Language::Bash;
        }
        if name == "makefile" || name.ends_with(".mk") || name.starts_with("makefile") {
            return Language::Makefile;
        }
        if name.ends_with(".yml") || name.ends_with(".yaml") {
            return Language::Yaml;
        }
    }

    // Content-based detection for code fence languages
    if content.trim_start().starts_with("#!/bin/bash") || 
       content.trim_start().starts_with("#!/bin/sh") {
        return Language::Bash;
    }

    // Look for C-style patterns
    if content.contains("#include") || 
       content.contains("int main") || 
       content.contains("printf") ||
       content.contains("MODULE_") {
        return Language::C;
    }

    // Look for Makefile patterns (strong indicators first)
    if content.contains("$(") || 
       content.contains("\t") { // Makefiles use tabs for commands (very strong indicator)
        return Language::Makefile;
    }

    // Check for Makefile target patterns
    let has_makefile_target = content.lines().any(|line| {
        let trimmed = line.trim();
        if trimmed.contains(":") && !trimmed.starts_with("#") {
            // Split on colon
            if let Some(before_colon) = trimmed.split(':').next() {
                let before_colon = before_colon.trim();
                // Makefile targets are typically single identifiers
                if !before_colon.is_empty() && 
                   !before_colon.contains(' ') &&
                   before_colon.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == '.') {
                    // This looks like a makefile target
                    return true;
                }
            }
        }
        false
    });

    // Look for YAML patterns
    let has_yaml_pattern = content.contains("---") || 
       content.lines().any(|line| {
           let trimmed = line.trim();
           // YAML key-value pairs: key: value with space after colon
           (trimmed.contains(": ") && !trimmed.starts_with("#")) ||
           // YAML list items starting with dash
           (trimmed.starts_with("- ") && !trimmed.starts_with("#"))
       });

    if has_makefile_target && !has_yaml_pattern {
        return Language::Makefile;
    }

    if has_yaml_pattern {
        return Language::Yaml;
    }

    Language::Auto
}

/// Get CSS class name for token type
pub fn get_css_class(token_type: &TokenType) -> &'static str {
    match token_type {
        TokenType::Keyword => "hl-keyword",
        TokenType::Identifier => "hl-identifier",
        TokenType::Function => "hl-function",
        TokenType::String => "hl-string",
        TokenType::Number => "hl-number",
        TokenType::Comment => "hl-comment",
        TokenType::Operator => "hl-operator",
        TokenType::Punctuation => "hl-punctuation",
        TokenType::Whitespace => "hl-whitespace",
        TokenType::Unknown => "hl-unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_detection() {
        assert_eq!(detect_language("", Some("test.c")), Language::C);
        assert_eq!(detect_language("#!/bin/bash", None), Language::Bash);
        assert_eq!(detect_language("#include <stdio.h>", None), Language::C);
        assert_eq!(detect_language("all: clean\n\tgcc -o test", None), Language::Makefile);
        assert_eq!(detect_language("name: value\nother: test", None), Language::Yaml);
    }

    // Comprehensive language detection tests
    #[test]
    fn test_filename_based_detection() {
        // C/C++ files
        assert_eq!(detect_language("", Some("main.c")), Language::C);
        assert_eq!(detect_language("", Some("header.h")), Language::C);
        assert_eq!(detect_language("", Some("program.cpp")), Language::C);
        assert_eq!(detect_language("", Some("header.hpp")), Language::C);
        assert_eq!(detect_language("", Some("MAIN.C")), Language::C); // case insensitive

        // Bash files
        assert_eq!(detect_language("", Some("script.sh")), Language::Bash);
        assert_eq!(detect_language("", Some("install.bash")), Language::Bash);
        assert_eq!(detect_language("", Some("bashrc")), Language::Bash);
        assert_eq!(detect_language("", Some(".bashrc")), Language::Bash);

        // Makefile variants
        assert_eq!(detect_language("", Some("Makefile")), Language::Makefile);
        assert_eq!(detect_language("", Some("makefile")), Language::Makefile);
        assert_eq!(detect_language("", Some("Makefile.debug")), Language::Makefile);
        assert_eq!(detect_language("", Some("build.mk")), Language::Makefile);

        // YAML files
        assert_eq!(detect_language("", Some("config.yml")), Language::Yaml);
        assert_eq!(detect_language("", Some("docker-compose.yaml")), Language::Yaml);
    }

    #[test]
    fn test_content_based_detection() {
        // Bash shebang variations
        assert_eq!(detect_language("#!/bin/bash\necho hello", None), Language::Bash);
        assert_eq!(detect_language("#!/bin/sh\necho hello", None), Language::Bash);
        assert_eq!(detect_language("  #!/bin/bash", None), Language::Bash); // with leading whitespace

        // C patterns
        assert_eq!(detect_language("#include <stdio.h>\nint main() {}", None), Language::C);
        assert_eq!(detect_language("int main(void) { return 0; }", None), Language::C);
        assert_eq!(detect_language("printf(\"hello world\");", None), Language::C);
        assert_eq!(detect_language("MODULE_LICENSE(\"GPL\");", None), Language::C);

        // Makefile patterns with tabs
        assert_eq!(detect_language("target:\n\tcommand", None), Language::Makefile);
        assert_eq!(detect_language("CC=$(shell which gcc)", None), Language::Makefile);
        assert_eq!(detect_language("$(info Building project)", None), Language::Makefile);

        // YAML patterns
        assert_eq!(detect_language("---\nkey: value", None), Language::Yaml);
        assert_eq!(detect_language("version: '3.8'\nservices:", None), Language::Yaml);
        assert_eq!(detect_language("- item1\n- item2", None), Language::Yaml);
    }

    #[test]
    fn test_edge_cases_language_detection() {
        // Empty content with no filename
        assert_eq!(detect_language("", None), Language::Auto);

        // Mixed content that could be ambiguous
        let mixed_content = "# This could be bash or makefile\nall:\n\ttest";
        assert_eq!(detect_language(mixed_content, None), Language::Makefile); // Should prefer Makefile due to target

        // Content with only comments
        assert_eq!(detect_language("# Just a comment", Some("test.sh")), Language::Bash);
        assert_eq!(detect_language("// Just a comment", Some("test.c")), Language::C);

        // Filename takes precedence over content
        assert_eq!(detect_language("echo hello", Some("test.c")), Language::C);
    }

    #[test]
    fn test_complex_detection_scenarios() {
        // Complex Makefile
        let makefile_content = r#"
            CC=gcc
            CFLAGS=-Wall -Wextra
            
            all: main.o utils.o
                $(CC) $(CFLAGS) -o program $^
            
            %.o: %.c
                $(CC) $(CFLAGS) -c $< -o $@
            
            clean:
                rm -f *.o program
        "#;
        assert_eq!(detect_language(makefile_content, None), Language::Makefile);

        // Complex YAML
        let yaml_content = r#"
            version: '3.8'
            services:
              web:
                build: .
                ports:
                  - "8000:8000"
                volumes:
                  - .:/code
              database:
                image: postgres
                environment:
                  POSTGRES_DB: myapp
        "#;
        assert_eq!(detect_language(yaml_content, None), Language::Yaml);

        // Complex C code
        let c_content = r#"
            #include <stdio.h>
            #include <stdlib.h>
            
            int main(int argc, char *argv[]) {
                printf("Hello, world!\n");
                return EXIT_SUCCESS;
            }
        "#;
        assert_eq!(detect_language(c_content, None), Language::C);

        // Complex Bash script
        let bash_content = r#"
            #!/bin/bash
            
            set -euo pipefail
            
            if [[ $# -eq 0 ]]; then
                echo "Usage: $0 <filename>"
                exit 1
            fi
            
            for file in "$@"; do
                echo "Processing: $file"
            done
        "#;
        assert_eq!(detect_language(bash_content, None), Language::Bash);
    }

    #[test]
    fn test_language_detection_priorities() {
        // Strong indicators should override weaker ones
        
        // Tab indicates Makefile even with colon-space pattern
        let makefile_with_spaces = "target: dependency\n\tcommand with spaces";
        assert_eq!(detect_language(makefile_with_spaces, None), Language::Makefile);

        // Shebang should be strong indicator for Bash
        let bash_with_colons = "#!/bin/bash\nkey: value"; // could look like YAML
        assert_eq!(detect_language(bash_with_colons, None), Language::Bash);

        // C includes should be strong indicator
        let c_with_colons = "#include <stdio.h>\nlabel: statement"; // could look like other languages
        assert_eq!(detect_language(c_with_colons, None), Language::C);
    }

    #[test]
    fn test_css_class_mapping() {
        assert_eq!(get_css_class(&TokenType::Keyword), "hl-keyword");
        assert_eq!(get_css_class(&TokenType::Identifier), "hl-identifier");
        assert_eq!(get_css_class(&TokenType::Function), "hl-function");
        assert_eq!(get_css_class(&TokenType::String), "hl-string");
        assert_eq!(get_css_class(&TokenType::Number), "hl-number");
        assert_eq!(get_css_class(&TokenType::Comment), "hl-comment");
        assert_eq!(get_css_class(&TokenType::Operator), "hl-operator");
        assert_eq!(get_css_class(&TokenType::Punctuation), "hl-punctuation");
        assert_eq!(get_css_class(&TokenType::Whitespace), "hl-whitespace");
        assert_eq!(get_css_class(&TokenType::Unknown), "hl-unknown");
    }

    #[test]
    fn test_language_detection_false_positives() {
        // These should NOT be detected as the specified language
        
        // YAML-like but not YAML (no space after colon)
        assert_ne!(detect_language("target:dependency", None), Language::Yaml);
        
        // Makefile-like but actually YAML (space after colon, no tabs)
        let yaml_like = "build: ./Dockerfile\nports: 8080:80";
        assert_eq!(detect_language(yaml_like, None), Language::Yaml);
        
        // Not C just because it has some keywords (no clear C indicators)
        assert_ne!(detect_language("int return void", None), Language::C);
        
        // Not Bash just because it has # (could be Makefile comment)
        let makefile_comment = "# This is a makefile comment\nall:\n\tbuild";
        assert_eq!(detect_language(makefile_comment, None), Language::Makefile);
    }

    #[test]
    fn test_unicode_in_detection() {
        // Language detection should work with Unicode content
        let unicode_yaml = "名前: テスト\n値: 42";
        // Should be detected as YAML due to colon-space pattern
        assert_eq!(detect_language(unicode_yaml, None), Language::Yaml);

        // Unicode with filename should work
        assert_eq!(detect_language("printf(\"こんにちは\");", Some("test.c")), Language::C);
    }

    #[test]
    fn test_malformed_content() {
        // Malformed but shouldn't crash
        assert_eq!(detect_language(":", None), Language::Auto);
        assert_eq!(detect_language(":::", None), Language::Auto);
        assert_eq!(detect_language("$()", None), Language::Makefile); // Has $( pattern
        assert_eq!(detect_language("---", None), Language::Yaml); // Has YAML marker
    }
}