/// Makefile language-specific tokenization rules

pub struct MakefileLanguage;

impl MakefileLanguage {
    /// Enhanced Makefile keyword and directive detection
    pub fn is_keyword(text: &str) -> bool {
        matches!(text,
            // Standard targets
            "all" | "clean" | "install" | "uninstall" | "distclean" | "check" | "test" |
            "dist" | "distcheck" | "maintainer-clean" | "mostlyclean" | "realclean" |
            "info" | "dvi" | "html" | "pdf" | "ps" | "tags" | "TAGS" |
            
            // Make directives
            "include" | "sinclude" | "-include" | "override" | "export" | "unexport" |
            "vpath" | "define" | "endef" | "ifdef" | "ifndef" | "ifeq" | "ifneq" |
            "else" | "endif" | "error" | "warning" | "eval" | "call" |
            
            // Built-in functions (without parentheses)
            "subst" | "patsubst" | "strip" | "findstring" | "filter" | "filter-out" |
            "sort" | "word" | "wordlist" | "words" | "firstword" | "lastword" |
            "dir" | "notdir" | "suffix" | "basename" | "addsuffix" | "addprefix" |
            "join" | "wildcard" | "realpath" | "abspath" | "if" | "or" | "and" |
            "foreach" | "file" | "shell" | "origin" | "flavor" | "value" |
            
            // Common build tools
            "gcc" | "g++" | "clang" | "clang++" | "ld" | "ar" | "ranlib" |
            "objcopy" | "objdump" | "nm" | "size" | "readelf" | "make" | "cmake" |
            "pkg-config" | "autoconf" | "automake" | "libtool" | "cp" |
            "rm" | "mkdir" | "rmdir" | "ln" | "chmod" | "chown" | "tar" | "gzip"
        )
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_makefile_keywords() {
        assert!(MakefileLanguage::is_keyword("all"));
        assert!(MakefileLanguage::is_keyword("clean"));
        assert!(MakefileLanguage::is_keyword("include"));
        assert!(MakefileLanguage::is_keyword("gcc"));
        assert!(!MakefileLanguage::is_keyword("my_target"));
    }

}