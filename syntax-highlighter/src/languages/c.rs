/// C/C++ language-specific tokenization rules

pub struct CLanguage;

impl CLanguage {
    /// Enhanced C/C++ keyword detection
    pub fn is_keyword(text: &str) -> bool {
        matches!(
            text,
            // C keywords
            "auto" | "break" | "case" | "char" | "const" | "continue" | "default" | "do" |
            "double" | "else" | "enum" | "extern" | "float" | "for" | "goto" | "if" |
            "int" | "long" | "register" | "return" | "short" | "signed" | "sizeof" | "static" |
            "struct" | "switch" | "typedef" | "union" | "unsigned" | "void" | "volatile" | "while" |

            // C99/C11 keywords
            "inline" | "restrict" | "_Bool" | "_Complex" | "_Imaginary" | "_Static_assert" |
            "_Alignas" | "_Alignof" | "_Atomic" | "_Generic" | "_Noreturn" | "_Thread_local" |

            // C++ keywords (common ones)
            "class" | "namespace" | "template" | "typename" | "public" | "private" | "protected" |
            "virtual" | "override" | "final" | "explicit" | "operator" | "new" | "delete" |
            "this" | "nullptr" | "true" | "false" | "try" | "catch" | "throw" |

            // Common types
            "size_t" | "ssize_t" | "uint8_t" | "uint16_t" | "uint32_t" | "uint64_t" |
            "int8_t" | "int16_t" | "int32_t" | "int64_t" | "bool" | "string" |

            // Linux kernel specific
            "MODULE_LICENSE" | "MODULE_AUTHOR" | "MODULE_DESCRIPTION" | "MODULE_VERSION" |
            "module_init" | "module_exit" | "__init" | "__exit" | "KERN_INFO" | "KERN_ERR" |
            "printk" | "kmalloc" | "kfree" | "GFP_KERNEL" | "EXPORT_SYMBOL" | "EXPORT_SYMBOL_GPL" |
            "static_assert" | "__attribute__" | "__packed" | "__aligned" | "likely" | "unlikely" |

            // Preprocessor directives
            "include" | "define" | "ifdef" | "ifndef" | "endif" | "pragma" | "undef" |
            "error" | "warning" | "line" | "elif"
        )
    }

    /// Check if character is part of C preprocessor directive
    pub fn is_preprocessor_char(ch: char) -> bool {
        ch == '#'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_keywords() {
        assert!(CLanguage::is_keyword("int"));
        assert!(CLanguage::is_keyword("struct"));
        assert!(CLanguage::is_keyword("MODULE_LICENSE"));
        assert!(!CLanguage::is_keyword("my_variable"));
    }
}

