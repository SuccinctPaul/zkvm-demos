//! 进度追踪模块 - Phase 2 优化
//!
//! 提供实时进度反馈和可视化

use indicatif::{MultiProgress, ProgressBar, ProgressDrawTarget, ProgressStyle};
use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

/// 进度追踪器
pub struct ProgressTracker {
    multi: Arc<MultiProgress>,
    bars: Arc<Mutex<HashMap<String, ProgressBar>>>,
    main_bar: ProgressBar,
    enabled: bool,
}

impl ProgressTracker {
    /// 创建新的进度追踪器
    pub fn new(total_tests: usize) -> Self {
        let multi = Arc::new(MultiProgress::new());
        let main_bar = multi.add(ProgressBar::new(total_tests as u64));

        main_bar.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) {msg}"
                )
                .unwrap()
                .progress_chars("█▓▒░ ")
                .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
        );

        main_bar.set_message("Initializing benchmarks...");
        main_bar.enable_steady_tick(Duration::from_millis(100));

        Self {
            multi,
            bars: Arc::new(Mutex::new(HashMap::new())),
            main_bar,
            enabled: true,
        }
    }

    /// 创建禁用的进度追踪器（用于 CI 环境）
    pub fn disabled() -> Self {
        let multi = Arc::new(MultiProgress::with_draw_target(ProgressDrawTarget::hidden()));
        let main_bar = multi.add(ProgressBar::hidden());

        Self {
            multi,
            bars: Arc::new(Mutex::new(HashMap::new())),
            main_bar,
            enabled: false,
        }
    }

    /// 为 zkVM 创建子进度条
    pub fn create_zkvm_bar(&self, zkvm_name: &str, test_count: usize) -> Option<ProgressBar> {
        if !self.enabled {
            return None;
        }

        let bar = self.multi.add(ProgressBar::new(test_count as u64));
        bar.set_style(
            ProgressStyle::default_bar()
                .template(&format!(
                    "  {{spinner:.blue}} [{{bar:30.cyan/blue}}] {{pos}}/{{len}} {} {{msg}}",
                    zkvm_name
                ))
                .unwrap()
                .progress_chars("█▓▒░ "),
        );

        bar.set_message("Pending...");
        bar.enable_steady_tick(Duration::from_millis(100));

        let mut bars = self.bars.lock();
        bars.insert(zkvm_name.to_string(), bar.clone());

        Some(bar)
    }

    /// 更新 zkVM 进度
    pub fn update_zkvm(&self, zkvm_name: &str, message: String) {
        if !self.enabled {
            return;
        }

        let bars = self.bars.lock();
        if let Some(bar) = bars.get(zkvm_name) {
            bar.set_message(message);
            bar.inc(1);
        }
    }

    /// 更新主进度
    pub fn update_main(&self, message: String) {
        if !self.enabled {
            return;
        }

        self.main_bar.set_message(message);
        self.main_bar.inc(1);
    }

    /// 完成 zkVM 进度条
    pub fn finish_zkvm(&self, zkvm_name: &str, success_count: usize, total_count: usize) {
        if !self.enabled {
            return;
        }

        let bars = self.bars.lock();
        if let Some(bar) = bars.get(zkvm_name) {
            let message = if success_count == total_count {
                format!("✓ Completed ({}/{})", success_count, total_count)
            } else {
                format!(
                    "⚠ Completed ({}/{}) - {} failed",
                    success_count,
                    total_count,
                    total_count - success_count
                )
            };
            bar.finish_with_message(message);
        }
    }

    /// 完成主进度条
    pub fn finish(&self, success_count: usize, total_count: usize, duration: Duration) {
        if !self.enabled {
            return;
        }

        let message = if success_count == total_count {
            format!(
                "✅ All benchmarks completed successfully in {:.1}s",
                duration.as_secs_f64()
            )
        } else {
            format!(
                "⚠ Completed with {} failures in {:.1}s ({}/{})",
                total_count - success_count,
                duration.as_secs_f64(),
                success_count,
                total_count
            )
        };

        self.main_bar.finish_with_message(message);
    }

    /// 设置总数（用于动态调整）
    pub fn set_length(&self, length: u64) {
        if self.enabled {
            self.main_bar.set_length(length);
        }
    }

    /// 打印消息（不影响进度条）
    pub fn println(&self, message: &str) {
        if self.enabled {
            self.main_bar.println(message);
        } else {
            println!("{}", message);
        }
    }

    /// 检查是否启用
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// 简化的进度追踪器（用于简单场景）
pub struct SimpleProgressTracker {
    total: usize,
    current: Arc<Mutex<usize>>,
    bar: Option<ProgressBar>,
}

impl SimpleProgressTracker {
    /// 创建新的简单进度追踪器
    pub fn new(total: usize, show_progress: bool) -> Self {
        let bar = if show_progress {
            let pb = ProgressBar::new(total as u64);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
                    .unwrap()
                    .progress_chars("█▓▒░ ")
            );
            pb.enable_steady_tick(Duration::from_millis(100));
            Some(pb)
        } else {
            None
        };

        Self {
            total,
            current: Arc::new(Mutex::new(0)),
            bar,
        }
    }

    /// 增加进度
    pub fn inc(&self, message: &str) {
        let mut current = self.current.lock();
        *current += 1;

        if let Some(ref bar) = self.bar {
            bar.set_message(message.to_string());
            bar.inc(1);
        }
    }

    /// 完成
    pub fn finish(&self, message: &str) {
        if let Some(ref bar) = self.bar {
            bar.finish_with_message(message.to_string());
        }
    }

    /// 获取当前进度
    pub fn current(&self) -> usize {
        *self.current.lock()
    }

    /// 获取总数
    pub fn total(&self) -> usize {
        self.total
    }

    /// 打印消息
    pub fn println(&self, message: &str) {
        if let Some(ref bar) = self.bar {
            bar.println(message);
        } else {
            println!("{}", message);
        }
    }
}

/// 进度统计
#[derive(Debug, Clone)]
pub struct ProgressStats {
    pub total: usize,
    pub completed: usize,
    pub successful: usize,
    pub failed: usize,
    pub in_progress: usize,
}

impl ProgressStats {
    pub fn new(total: usize) -> Self {
        Self {
            total,
            completed: 0,
            successful: 0,
            failed: 0,
            in_progress: 0,
        }
    }

    pub fn start_task(&mut self) {
        self.in_progress += 1;
    }

    pub fn complete_task(&mut self, success: bool) {
        self.in_progress = self.in_progress.saturating_sub(1);
        self.completed += 1;

        if success {
            self.successful += 1;
        } else {
            self.failed += 1;
        }
    }

    pub fn progress_percent(&self) -> f32 {
        if self.total == 0 {
            0.0
        } else {
            (self.completed as f32 / self.total as f32) * 100.0
        }
    }

    pub fn success_rate(&self) -> f32 {
        if self.completed == 0 {
            0.0
        } else {
            (self.successful as f32 / self.completed as f32) * 100.0
        }
    }

    pub fn format_summary(&self) -> String {
        format!(
            "{}/{} completed ({:.1}%) - {} successful, {} failed",
            self.completed,
            self.total,
            self.progress_percent(),
            self.successful,
            self.failed
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_stats() {
        let mut stats = ProgressStats::new(10);

        assert_eq!(stats.total, 10);
        assert_eq!(stats.completed, 0);
        assert_eq!(stats.progress_percent(), 0.0);

        stats.start_task();
        assert_eq!(stats.in_progress, 1);

        stats.complete_task(true);
        assert_eq!(stats.completed, 1);
        assert_eq!(stats.successful, 1);
        assert_eq!(stats.in_progress, 0);
        assert_eq!(stats.progress_percent(), 10.0);
        assert_eq!(stats.success_rate(), 100.0);

        stats.start_task();
        stats.complete_task(false);
        assert_eq!(stats.completed, 2);
        assert_eq!(stats.failed, 1);
        assert_eq!(stats.success_rate(), 50.0);
    }

    #[test]
    fn test_simple_progress_tracker() {
        let tracker = SimpleProgressTracker::new(5, false);

        assert_eq!(tracker.total(), 5);
        assert_eq!(tracker.current(), 0);

        tracker.inc("Task 1");
        assert_eq!(tracker.current(), 1);

        tracker.inc("Task 2");
        assert_eq!(tracker.current(), 2);

        tracker.finish("Done");
    }

    #[test]
    fn test_progress_tracker_disabled() {
        let tracker = ProgressTracker::disabled();
        assert!(!tracker.is_enabled());

        // 这些调用不应该产生任何输出
        tracker.update_main("Test".to_string());
        tracker.update_zkvm("sp1", "Test".to_string());
        tracker.finish(5, 5, Duration::from_secs(10));
    }
}
