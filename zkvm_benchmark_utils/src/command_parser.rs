///! 优化的命令解析模块
///!
///! 提供安全、高效的命令行解析，支持：
///! - 引号和转义字符
///! - 命令缓存
///! - 安全验证
///! - 环境变量展开
use crate::error::{BenchmarkError, Result};
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

/// 解析后的命令结构
#[derive(Debug, Clone)]
pub struct ParsedCommand {
    pub program: String,
    pub args: Vec<String>,
}

/// 命令缓存，使用 Arc 减少克隆开销
type CommandCache = HashMap<String, Arc<ParsedCommand>>;

/// 全局命令缓存
static COMMAND_CACHE: Lazy<RwLock<CommandCache>> = Lazy::new(|| RwLock::new(HashMap::new()));

/// 命令解析器
pub struct CommandParser {
    enable_cache: bool,
    security_check: bool,
}

impl CommandParser {
    /// 创建新的命令解析器
    pub fn new() -> Self {
        Self {
            enable_cache: true,
            security_check: true,
        }
    }

    /// 创建不带安全检查的解析器（用于测试）
    pub fn new_unchecked() -> Self {
        Self {
            enable_cache: true,
            security_check: false,
        }
    }

    /// 解析命令字符串
    ///
    /// # Examples
    ///
    /// ```
    /// use zkvm_benchmark_utils::command_parser::CommandParser;
    ///
    /// let parser = CommandParser::new();
    /// let cmd = parser.parse("cargo run --bin test").unwrap();
    /// assert_eq!(cmd.program, "cargo");
    /// assert_eq!(cmd.args, vec!["run", "--bin", "test"]);
    /// ```
    pub fn parse(&self, command: &str) -> Result<Arc<ParsedCommand>> {
        // 安全检查
        if self.security_check {
            self.validate_security(command)?;
        }

        // 尝试从缓存读取
        if self.enable_cache {
            let cache = COMMAND_CACHE.read();
            if let Some(parsed) = cache.get(command) {
                return Ok(Arc::clone(parsed));
            }
        }

        // 解析命令
        let parsed = self.parse_command_parts(command)?;

        // 写入缓存
        if self.enable_cache {
            let mut cache = COMMAND_CACHE.write();
            cache.insert(command.to_string(), Arc::clone(&parsed));
        }

        Ok(parsed)
    }

    /// 实际解析命令字符串
    fn parse_command_parts(&self, command: &str) -> Result<Arc<ParsedCommand>> {
        // 使用 shell_words crate 处理引号和转义
        // 如果 shell_words 不可用，使用简化版本
        let parts = self.split_command(command)?;

        if parts.is_empty() {
            return Err(BenchmarkError::Execution("Empty command".to_string()));
        }

        Ok(Arc::new(ParsedCommand {
            program: parts[0].clone(),
            args: parts[1..].to_vec(),
        }))
    }

    /// 分割命令字符串（简化版本）
    ///
    /// 支持：
    /// - 单引号和双引号
    /// - 转义字符
    /// - 空格分隔
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

    /// 验证命令安全性
    fn validate_security(&self, command: &str) -> Result<()> {
        // 检查危险字符
        const DANGEROUS_CHARS: &[char] = &['$', '`', '|', '&', ';', '\n', '\r'];

        for dangerous_char in DANGEROUS_CHARS {
            if command.contains(*dangerous_char) {
                return Err(BenchmarkError::Execution(format!(
                    "Potentially dangerous character '{}' in command: {}",
                    dangerous_char, command
                )));
            }
        }

        // 检查路径遍历
        if command.contains("..") {
            return Err(BenchmarkError::Execution(format!(
                "Path traversal detected in command: {}",
                command
            )));
        }

        Ok(())
    }

    /// 清空缓存（用于测试）
    pub fn clear_cache() {
        let mut cache = COMMAND_CACHE.write();
        cache.clear();
    }

    /// 获取缓存大小（用于监控）
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
    #[cfg_attr(not(feature = "sequential-tests"), ignore)]
    fn test_cache_functionality() {
        // 注意：这个测试依赖全局状态，应该串行运行
        // 使用：cargo test -- --test-threads=1

        CommandParser::clear_cache();
        let parser = CommandParser::new_unchecked();

        // 使用唯一的命令来避免冲突
        let unique_cmd = format!("cargo build --test-unique-{}", std::process::id());

        // 第一次解析
        let cmd1 = parser.parse(&unique_cmd).unwrap();
        let size_after_first = CommandParser::cache_size();

        // 第二次解析相同命令，应该从缓存返回
        let cmd2 = parser.parse(&unique_cmd).unwrap();
        let size_after_second = CommandParser::cache_size();

        // 应该返回相同的 Arc 指针（缓存命中）
        assert!(Arc::ptr_eq(&cmd1, &cmd2));
        // 缓存大小不应该变化
        assert_eq!(size_after_first, size_after_second);

        // 验证解析结果正确
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
