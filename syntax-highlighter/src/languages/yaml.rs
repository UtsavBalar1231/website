/// YAML/JSON language-specific tokenization rules

pub struct YamlLanguage;

impl YamlLanguage {
    /// Enhanced YAML keyword and value detection
    pub fn is_keyword(text: &str) -> bool {
        matches!(text,
            // YAML boolean values
            "true" | "True" | "TRUE" | "false" | "False" | "FALSE" |
            "yes" | "Yes" | "YES" | "no" | "No" | "NO" |
            "on" | "On" | "ON" | "off" | "Off" | "OFF" |
            
            // YAML null values
            "null" | "Null" | "NULL" | "~" |
            
            // YAML special values
            ".nan" | ".NaN" | ".NAN" | ".inf" | ".Inf" | ".INF" |
            "-.inf" | "-.Inf" | "-.INF" | "+.inf" | "+.Inf" | "+.INF" |
            
            // Common YAML document markers
            "---" | "..." |
            
            // Common CI/CD and configuration keywords
            "version" | "name" | "description" | "author" | "license" | "main" |
            "scripts" | "dependencies" | "devDependencies" | "keywords" |
            "repository" | "bugs" | "homepage" | "engines" | "private" |
            
            // GitHub Actions / GitLab CI keywords
            "jobs" | "runs-on" | "steps" | "uses" | "with" | "run" |
            "env" | "if" | "needs" | "strategy" | "matrix" | "include" | "exclude" |
            "services" | "container" | "volumes" | "ports" | "options" |
            "timeout-minutes" | "continue-on-error" | "outputs" | "secrets" |
            "workflow_dispatch" | "push" | "pull_request" | "schedule" | "cron" |
            
            // Docker Compose keywords
            "networks" | "configs" |
            "image" | "build" | "command" | "entrypoint" | "working_dir" |
            "user" | "expose" | "environment" |
            "env_file" | "depends_on" | "links" | "restart" |
            
            // Kubernetes keywords
            "apiVersion" | "kind" | "metadata" | "spec" | "status" |
            "namespace" | "annotations" | "selector" |
            "template" | "containers" | "volumeMounts" |
            "resources" | "limits" | "requests" | "cpu" | "memory" |
            "replicas" | "rollingUpdate" | "maxSurge" | "maxUnavailable"
        )
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yaml_keywords() {
        assert!(YamlLanguage::is_keyword("true"));
        assert!(YamlLanguage::is_keyword("false"));
        assert!(YamlLanguage::is_keyword("null"));
        assert!(YamlLanguage::is_keyword("yes"));
        assert!(YamlLanguage::is_keyword("no"));
        assert!(!YamlLanguage::is_keyword("my_value"));
    }

}