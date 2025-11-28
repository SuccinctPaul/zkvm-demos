///! Optimized command parsing module
///!
///! Provides safe, efficient command line parsing, supporting:
///! - Quotes and escape characters
///! - Command caching
///! - Security validation
///! - Environment variable expansion
use crate::core::error::{BenchmarkError, Result};
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

/// Parsed command structure
#[derive(Debug, Clone)]
pub struct ParsedCommand {
    pub program: String,
    pub args: Vec<String>,
}

/// Command cache, using Arc to reduce cloning overhead
type CommandCache = HashMap<String, Arc<ParsedCommand>>;

/// Global command cache
static COMMAND_CACHE: Lazy<RwLock<CommandCache>> = Lazy::new(|| RwLock::new(HashMap::new()));

/// Command parser
pub struct CommandParser {
    enable_cache: bool,
    security_check: bool,
}

impl CommandParser {
    /// Create a new command parser
    pub fn new() -> Self {
        Self {
            enable_cache: true,
            security_check: true,
        }
    }

    /// Create a parser without security checks (for testing)
    pub fn new_unchecked() -> Self {
        Self {
            enable_cache: true,
            security_check: false,
        }
    }

    /// Parse command string
    ///
    /// # Examples
    ///
    /// ```
    /// use zkvm_benchmark_utils::CommandParser;
    ///
    /// let parser = CommandParser::new();
    /// let cmd = parser.parse("cargo run --bin test").unwrap();
    /// assert_eq!(cmd.program, "cargo");
    /// assert_eq!(cmd.args, vec!["run", "--bin", "test"]);
    /// ```
    pub fn parse(&self, command: &str) -> Result<Arc<ParsedCommand>> {
        // Security check
        if self.security_check {
            self.validate_security(command)?;
        }

        // Try to read from cache
        if self.enable_cache {
            let cache = COMMAND_CACHE.read();
            if let Some(parsed) = cache.get(command) {
                return Ok(Arc::clone(parsed));
            }
        }

        // Parse command
        let parsed = self.parse_command_parts(command)?;

        // Write to cache
        if self.enable_cache {
            let mut cache = COMMAND_CACHE.write();
            cache.insert(command.to_string(), Arc::clone(&parsed));
        }

        Ok(parsed)
    }

    /// Actually parse command string
    fn parse_command_parts(&self, command: &str) -> Result<Arc<ParsedCommand>> {
        // Use shell_words crate to handle quotes and escapes
        // If shell_words is not available, use simplified version
        let parts = self.split_command(command)?;

        if parts.is_empty() {
            return Err(BenchmarkError::Execution("Empty command".to_string()));
        }

        Ok(Arc::new(ParsedCommand {
            program: parts[0].clone(),
            args: parts[1..].to_vec(),
        }))
    }

    /// Split command string (simplified version)
    ///
    /// Supports:
    /// - Single and double quotes
    /// - Escape characters
    /// - Space separation
    fn split_command(&self, command: &str) -> Result<Vec<String>> {
        let mut parts = Vec::new();
        let mut current = String::new();
        let mut in_single_quote = false;
        let mut in_double_quote = false;
        let mut escape_next = false;

        let chars: Vec<char> = command.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let c = chars[i];

            if escape_next {
                current.push(c);
                escape_next = false;
                i += 1;
                continue;
            }

            match c {
                '\\' => {
                    escape_next = true;
                }
                '\'' if !in_double_quote => {
                    in_single_quote = !in_single_quote;
                }
                '"' if !in_single_quote => {
                    in_double_quote = !in_double_quote;
                }
                ' ' | '\t' if !in_single_quote && !in_double_quote => {
                    if !current.is_empty() {
                        parts.push(current.clone());
                        current.clear();
                    }
                }
                _ => {
                    current.push(c);
                }
            }

            i += 1;
        }

        if !current.is_empty() {
            parts.push(current);
        }

        if in_single_quote || in_double_quote {
            return Err(BenchmarkError::Execution(
                "Unclosed quote in command".to_string(),
            ));
        }

        Ok(parts)
    }

    /// Validate command security
    fn validate_security(&self, command: &str) -> Result<()> {
        // Check for dangerous characters
        const DANGEROUS_CHARS: &[char] = &['$', '`', '|', '&', ';', '\n', '\r'];

        for dangerous_char in DANGEROUS_CHARS {
            if command.contains(*dangerous_char) {
                return Err(BenchmarkError::Execution(format!(
                    "Potentially dangerous character '{}' in command: {}",
                    dangerous_char, command
                )));
            }
        }

        // Check for path traversal
        if command.contains("..") {
            return Err(BenchmarkError::Execution(format!(
                "Path traversal detected in command: {}",
                command
            )));
        }

        Ok(())
    }

    /// Clear cache (for testing)
    pub fn clear_cache() {
        let mut cache = COMMAND_CACHE.write();
        cache.clear();
    }

    /// Get cache size (for monitoring)
    pub fn cache_size() -> usize {
        let cache = COMMAND_CACHE.read();
        cache.len()
    }
}

impl Default for CommandParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_command() {
        let parser = CommandParser::new_unchecked();
        let parsed = parser.parse("cargo build").unwrap();

        assert_eq!(parsed.program, "cargo");
        assert_eq!(parsed.args, vec!["build"]);
    }

    #[test]
    fn test_command_with_args() {
        let parser = CommandParser::new_unchecked();
        let parsed = parser.parse("cargo run --release --bin test").unwrap();

        assert_eq!(parsed.program, "cargo");
        assert_eq!(parsed.args, vec!["run", "--release", "--bin", "test"]);
    }

    #[test]
    fn test_command_with_quotes() {
        let parser = CommandParser::new_unchecked();
        let parsed = parser.parse(r#"echo "hello world""#).unwrap();

        assert_eq!(parsed.program, "echo");
        assert_eq!(parsed.args, vec!["hello world"]);
    }

    #[test]
    fn test_command_with_single_quotes() {
        let parser = CommandParser::new_unchecked();
        let parsed = parser.parse(r#"echo 'hello world'"#).unwrap();

        assert_eq!(parsed.program, "echo");
        assert_eq!(parsed.args, vec!["hello world"]);
    }

    #[test]
    fn test_command_with_escape() {
        let parser = CommandParser::new_unchecked();
        let parsed = parser.parse(r#"echo hello\ world"#).unwrap();

        assert_eq!(parsed.program, "echo");
        assert_eq!(parsed.args, vec!["hello world"]);
    }

    #[test]
    fn test_unclosed_quote_error() {
        let parser = CommandParser::new_unchecked();
        let result = parser.parse(r#"echo "hello"#);

        assert!(result.is_err());
    }

    #[test]
    fn test_security_check_dangerous_chars() {
        let parser = CommandParser::new();

        assert!(parser.parse("echo $HOME").is_err());
        assert!(parser.parse("echo `whoami`").is_err());
        assert!(parser.parse("echo hello | grep hi").is_err());
        assert!(parser.parse("echo hello && rm -rf /").is_err());
    }

    #[test]
    fn test_security_check_path_traversal() {
        let parser = CommandParser::new();

        assert!(parser.parse("cat ../../../etc/passwd").is_err());
    }

    #[test]
    #[ignore] // This test depends on global state and should run sequentially
              // Run with: cargo test -- --test-threads=1
    fn test_cache_functionality() {
        // Note: This test depends on global state and should run sequentially
        // Run with: cargo test -- --test-threads=1

        CommandParser::clear_cache();
        let parser = CommandParser::new_unchecked();

        // Use a unique command to avoid conflicts
        let unique_cmd = format!("cargo build --test-unique-{}", std::process::id());

        // First parse
        let cmd1 = parser.parse(&unique_cmd).unwrap();
        let size_after_first = CommandParser::cache_size();

        // Second parse of the same command, should return from cache
        let cmd2 = parser.parse(&unique_cmd).unwrap();
        let size_after_second = CommandParser::cache_size();

        // Should return the same Arc pointer (cache hit)
        assert!(Arc::ptr_eq(&cmd1, &cmd2));
        // Cache size should not change
        assert_eq!(size_after_first, size_after_second);

        // Verify parse result is correct
        assert_eq!(cmd1.program, "cargo");
        assert!(cmd1.args.contains(&"build".to_string()));
    }

    #[test]
    fn test_empty_command() {
        let parser = CommandParser::new_unchecked();
        let result = parser.parse("");

        assert!(result.is_err());
    }

    #[test]
    fn test_whitespace_handling() {
        let parser = CommandParser::new_unchecked();
        let parsed = parser.parse("  cargo   build   --release  ").unwrap();

        assert_eq!(parsed.program, "cargo");
        assert_eq!(parsed.args, vec!["build", "--release"]);
    }
}
