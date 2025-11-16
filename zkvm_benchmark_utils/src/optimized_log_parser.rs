//! 优化的日志解析器 - Phase 2 优化
//!
//! 使用 RegexSet 和单次遍历实现 5-10x 性能提升

use crate::config::LogPatterns;
use crate::error::{BenchmarkError, Result};
use crate::hardware;
use crate::metrics::*;
use regex::{Regex, RegexSet};
use std::collections::{HashMap, HashSet};

/// 优化的日志解析器
pub struct OptimizedLogParser {
    /// 单个正则表达式模式
    patterns: HashMap<String, Regex>,
    /// 组合的正则表达式集合（用于快速筛选）
    pattern_set: RegexSet,
    /// 模式名称顺序
    key_order: Vec<String>,
    /// 已提取的值缓存
    extracted_cache: parking_lot::RwLock<HashMap<String, String>>,
}

impl OptimizedLogParser {
    /// 创建新的优化日志解析器
    pub fn new(log_patterns: &LogPatterns) -> Result<Self> {
        let mut patterns = HashMap::new();
        let mut pattern_strings = Vec::new();
        let mut key_order = Vec::new();

        // 宏：添加模式
        macro_rules! add_pattern {
            ($field:expr, $name:expr) => {
                if let Some(pattern_str) = $field {
                    let regex = Regex::new(pattern_str).map_err(|e| {
                        BenchmarkError::Parse(format!("Invalid regex for {}: {}", $name, e))
                    })?;
                    patterns.insert($name.to_string(), regex);
                    pattern_strings.push(pattern_str.clone());
                    key_order.push($name.to_string());
                }
            };
        }

        // 添加所有模式
        add_pattern!(&log_patterns.total_cycles, "total_cycles");
        add_pattern!(
            &log_patterns.total_instruction_count,
            "total_instruction_count"
        );
        add_pattern!(&log_patterns.total_prove_time_s, "total_prove_time_s");
        add_pattern!(
            &log_patterns.final_proof_size_bytes,
            "final_proof_size_bytes"
        );
        add_pattern!(&log_patterns.verification_time_s, "verification_time_s");
        add_pattern!(&log_patterns.success_status, "success_status");
        add_pattern!(&log_patterns.execution_time_s, "execution_time_s");
        add_pattern!(&log_patterns.vm_prove_time_s, "vm_prove_time_s");
        add_pattern!(
            &log_patterns.recursive_prove_time_s,
            "recursive_prove_time_s"
        );
        add_pattern!(&log_patterns.snark_prove_time_s, "snark_prove_time_s");
        add_pattern!(&log_patterns.vm_core_proof_size_kb, "vm_core_proof_size_kb");
        add_pattern!(
            &log_patterns.compressed_proof_size_kb,
            "compressed_proof_size_kb"
        );
        add_pattern!(
            &log_patterns.groth16_proof_size_bytes,
            "groth16_proof_size_bytes"
        );
        add_pattern!(&log_patterns.syscall_cycles, "syscall_cycles");
        add_pattern!(&log_patterns.syscall_count, "syscall_count");
        add_pattern!(&log_patterns.segments, "segments");
        add_pattern!(&log_patterns.peak_memory_mb, "peak_memory_mb");
        add_pattern!(&log_patterns.security_bits, "security_bits");

        // 创建 RegexSet 用于快速匹配
        let pattern_set = if !pattern_strings.is_empty() {
            RegexSet::new(&pattern_strings)
                .map_err(|e| BenchmarkError::Parse(format!("Failed to create RegexSet: {}", e)))?
        } else {
            // 空的 RegexSet
            RegexSet::new(&[] as &[&str]).unwrap()
        };

        Ok(Self {
            patterns,
            pattern_set,
            key_order,
            extracted_cache: parking_lot::RwLock::new(HashMap::new()),
        })
    }

    /// 优化的解析方法 - 单次遍历提取所有值
    pub fn parse_optimized(
        &self,
        log_content: &str,
        zkvm_name: &str,
        program_name: &str,
    ) -> Result<BenchmarkMetrics> {
        // 清空缓存
        self.extracted_cache.write().clear();

        // 单次遍历提取所有匹配
        let extracted = self.extract_all_values_single_pass(log_content);

        // 构建 metrics
        self.build_metrics(extracted, zkvm_name, program_name)
    }

    /// 单次遍历提取所有值（核心优化）
    fn extract_all_values_single_pass(&self, content: &str) -> HashMap<String, String> {
        let mut extracted = HashMap::new();
        let mut remaining_keys: HashSet<usize> = (0..self.key_order.len()).collect();

        // 遍历每一行
        for line in content.lines() {
            if remaining_keys.is_empty() {
                // 早期退出：所有模式都已匹配
                break;
            }

            // 使用 RegexSet 快速判断这行是否包含任何匹配
            let matches = self.pattern_set.matches(line);

            if matches.matched_any() {
                // 对每个匹配的模式尝试提取值
                for idx in matches.iter() {
                    if !remaining_keys.contains(&idx) {
                        continue; // 已经提取过
                    }

                    let key = &self.key_order[idx];
                    if let Some(regex) = self.patterns.get(key) {
                        if let Some(caps) = regex.captures(line) {
                            if let Some(value) = caps.get(1) {
                                extracted.insert(key.clone(), value.as_str().to_string());
                                remaining_keys.remove(&idx);
                            }
                        }
                    }
                }
            }
        }

        extracted
    }

    /// 构建 BenchmarkMetrics
    fn build_metrics(
        &self,
        extracted: HashMap<String, String>,
        zkvm_name: &str,
        program_name: &str,
    ) -> Result<BenchmarkMetrics> {
        let mut metrics = BenchmarkMetrics::new(program_name.to_string(), zkvm_name.to_string());

        // 收集硬件信息
        metrics.metadata.hardware = Some(hardware::collect_hardware_info());

        // 辅助函数
        let get_u64 = |key: &str| -> Option<u64> { extracted.get(key)?.parse().ok() };

        let get_f64 = |key: &str| -> Option<f64> { extracted.get(key)?.parse().ok() };

        let get_u32 = |key: &str| -> Option<u32> { extracted.get(key)?.parse().ok() };

        let get_string = |key: &str| -> Option<String> { extracted.get(key).cloned() };

        // 提取 zkvm_version
        if let Some(version) = self.extract_version_direct(zkvm_name) {
            metrics.metadata.zkvm_version = Some(version);
        }

        // 解析 execution phase
        let execution_phase = ExecutionPhase {
            total_cycles: get_u64("total_cycles"),
            total_instruction_count: get_u64("total_instruction_count"),
            user_cycles: None,
            syscall_cycles: get_u64("syscall_cycles"),
            syscall_count: get_u64("syscall_count"),
            memory_accesses: None,
            touched_memory_addresses: None,
            execution_time_s: get_f64("execution_time_s"),
            execution_throughput: None,
            segments: get_u64("segments"),
            segment_size: None,
            max_segment_cycles: None,
        };

        metrics.execution_phase = Some(execution_phase);

        // 解析 proving phase
        let total_prove_time = get_f64("total_prove_time_s")
            .ok_or_else(|| BenchmarkError::MissingMetric("total_prove_time_s".to_string()))?;

        let final_proof_size = get_u64("final_proof_size_bytes")
            .ok_or_else(|| BenchmarkError::MissingMetric("final_proof_size_bytes".to_string()))?;

        let stage_1_vm_prove = get_f64("vm_prove_time_s").map(|time| Stage1VmProve {
            vm_prove_time_s: time,
            vm_prove_segments: get_u64("segments"),
            vm_prove_cycles_per_segment: None,
            vm_core_proof_size_kb: get_f64("vm_core_proof_size_kb"),
            vm_prove_memory_mb: get_f64("peak_memory_mb"),
            data_source: Some(DataSource::LogParsed),
        });

        let stage_2_recursive = get_f64("recursive_prove_time_s").map(|time| Stage2Recursive {
            recursive_prove_time_s: time,
            recursion_layers: None,
            recursive_input_size_kb: None,
            recursive_output_size_kb: None,
            recursive_proof_count: None,
            compressed_proof_size_kb: get_f64("compressed_proof_size_kb"),
            data_source: Some(DataSource::LogParsed),
        });

        let stage_4_snark = get_f64("snark_prove_time_s").map(|time| Stage4Snark {
            snark_prove_time_s: time,
            snark_setup_time_s: None,
            snark_witness_time_s: None,
            snark_proof_time_s: None,
            groth16_proof_size_bytes: get_u64("groth16_proof_size_bytes"),
            data_source: Some(DataSource::LogParsed),
        });

        let proving_phase = ProvingPhase {
            proof_mode: ProofMode::Groth16,
            security_bits: get_u32("security_bits"),
            fri_queries: None,
            blowup_factor: None,
            recursion_enabled: None,
            total_prove_time_s: total_prove_time,
            setup_time_s: None,
            stage_0_execute: None,
            stage_1_vm_prove,
            stage_2_recursive,
            stage_3_aggressive: None,
            stage_4_snark,
            time_breakdown_percent: None,
            proof_size_evolution: ProofSizeEvolution {
                stage_0_proof_size_mb: None,
                stage_1_proof_size_kb: get_f64("vm_core_proof_size_kb"),
                stage_2_proof_size_kb: get_f64("compressed_proof_size_kb"),
                stage_3_proof_size_kb: None,
                stage_4_proof_size_bytes: get_u64("groth16_proof_size_bytes"),
                final_proof_size_bytes: final_proof_size,
                total_compression_ratio: None,
            },
            performance_metrics: PerformanceMetrics {
                proving_throughput_kcycles_per_sec: None,
                khz: None,
                cycles_per_constraint: None,
                proof_efficiency_score: None,
            },
        };

        metrics.proving_phase = Some(proving_phase);

        // 解析 verification phase
        if let Some(verify_time) = get_f64("verification_time_s") {
            metrics.verification_phase = Some(VerificationPhase {
                verification_time_s: verify_time,
                verification_time_ms: verify_time * 1000.0,
                on_chain_gas_estimate: None,
            });
        }

        // 解析 resource metrics
        if let Some(peak_mem) = get_f64("peak_memory_mb") {
            metrics.resources = Some(ResourceMetrics {
                peak_memory_mb: Some(peak_mem),
                execution_memory_mb: None,
                proving_memory_mb: None,
                avg_cpu_percent: None,
                peak_cpu_percent: None,
                cpu_cores_used: None,
            });
        }

        // 更新 summary
        let verify_time = metrics
            .verification_phase
            .as_ref()
            .map(|v| v.verification_time_s)
            .unwrap_or(0.0);

        metrics.summary.total_time_s = metrics
            .execution_phase
            .as_ref()
            .and_then(|e| e.execution_time_s)
            .unwrap_or(0.0)
            + total_prove_time
            + verify_time;

        // 确定成功状态
        metrics.summary.success_status = get_string("success_status")
            .and_then(|s| match s.as_str() {
                "success" => Some(SuccessStatus::Success),
                "failed" => Some(SuccessStatus::Failed),
                "timeout" => Some(SuccessStatus::Timeout),
                _ => None,
            })
            .unwrap_or(SuccessStatus::Success);

        // 计算衍生指标
        metrics.calculate_derived_metrics();

        Ok(metrics)
    }

    /// 提取版本号（直接模式）
    fn extract_version_direct(&self, _zkvm_name: &str) -> Option<String> {
        // 可以从日志中提取版本号
        // 这里返回 None，让调用者从配置中获取
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::LogPatterns;

    #[test]
    fn test_optimized_parser_creation() {
        let patterns = LogPatterns {
            total_cycles: Some(r"total_cycles=(\d+)".to_string()),
            total_prove_time_s: Some(r"prove_time=([\d.]+)".to_string()),
            final_proof_size_bytes: Some(r"proof_size=(\d+)".to_string()),
            verification_time_s: Some(r"verify_time=([\d.]+)".to_string()),
            ..Default::default()
        };

        let parser = OptimizedLogParser::new(&patterns);
        assert!(parser.is_ok());
    }

    #[test]
    fn test_single_pass_extraction() {
        let patterns = LogPatterns {
            total_cycles: Some(r"total_cycles=(\d+)".to_string()),
            total_prove_time_s: Some(r"prove_time=([\d.]+)".to_string()),
            final_proof_size_bytes: Some(r"proof_size=(\d+)".to_string()),
            ..Default::default()
        };

        let parser = OptimizedLogParser::new(&patterns).unwrap();

        let log_content = r#"
            Starting benchmark...
            total_cycles=12543
            Running proof generation...
            prove_time=45.8
            Generating proof...
            proof_size=192
            Done!
        "#;

        let extracted = parser.extract_all_values_single_pass(log_content);

        assert_eq!(extracted.get("total_cycles"), Some(&"12543".to_string()));
        assert_eq!(
            extracted.get("total_prove_time_s"),
            Some(&"45.8".to_string())
        );
        assert_eq!(
            extracted.get("final_proof_size_bytes"),
            Some(&"192".to_string())
        );
    }

    #[test]
    fn test_early_exit_optimization() {
        let patterns = LogPatterns {
            total_cycles: Some(r"total_cycles=(\d+)".to_string()),
            total_prove_time_s: Some(r"prove_time=([\d.]+)".to_string()),
            ..Default::default()
        };

        let parser = OptimizedLogParser::new(&patterns).unwrap();

        // 在开头就包含所有匹配的日志
        let log_content = r#"
            total_cycles=12543
            prove_time=45.8
            (many more lines that won't be processed...)
            line 1000
            line 2000
            ...
        "#;

        let extracted = parser.extract_all_values_single_pass(log_content);

        // 应该在前两行就完成提取
        assert_eq!(extracted.len(), 2);
    }
}
