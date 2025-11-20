// Runtime Resource Monitoring for zkVM Benchmarks
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use sysinfo::{Pid, System};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSample {
    /// Timestamp relative to monitoring start (seconds)
    pub timestamp_s: f64,
    /// CPU usage percentage (0-100 per core, can exceed 100)
    pub cpu_percent: f32,
    /// Memory usage in bytes
    pub memory_bytes: u64,
    /// Disk read bytes (cumulative)
    pub disk_read_bytes: u64,
    /// Disk write bytes (cumulative)
    pub disk_write_bytes: u64,
    /// Number of threads
    pub num_threads: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceStats {
    pub samples: Vec<ResourceSample>,
    pub peak_memory_mb: f64,
    pub avg_memory_mb: f64,
    pub peak_cpu_percent: f32,
    pub avg_cpu_percent: f32,
    pub total_disk_read_mb: f64,
    pub total_disk_write_mb: f64,
    pub max_threads: usize,
    pub avg_threads: f64,
    pub duration_s: f64,
}

pub struct ResourceMonitor {
    pid: Pid,
    interval: Duration,
    samples: Vec<ResourceSample>,
    start_time: Instant,
    system: System,
}

impl ResourceMonitor {
    /// Create a new resource monitor for a process
    pub fn new(pid: u32, interval_ms: u64) -> Self {
        Self {
            pid: Pid::from_u32(pid),
            interval: Duration::from_millis(interval_ms),
            samples: Vec::new(),
            start_time: Instant::now(),
            system: System::new_all(),
        }
    }

    /// Sample current resource usage
    pub fn sample(&mut self) -> Option<ResourceSample> {
        self.system.refresh_process(self.pid);

        let process = self.system.process(self.pid)?;
        let elapsed = self.start_time.elapsed().as_secs_f64();

        Some(ResourceSample {
            timestamp_s: elapsed,
            cpu_percent: process.cpu_usage(),
            memory_bytes: process.memory(),
            disk_read_bytes: process.disk_usage().read_bytes,
            disk_write_bytes: process.disk_usage().written_bytes,
            num_threads: 1, // Note: thread count not directly available in sysinfo 0.30
        })
    }

    /// Record a sample
    pub fn record(&mut self) {
        if let Some(sample) = self.sample() {
            self.samples.push(sample);
        }
    }

    /// Get all samples
    pub fn samples(&self) -> &[ResourceSample] {
        &self.samples
    }

    /// Calculate statistics from samples
    pub fn calculate_stats(&self) -> Option<ResourceStats> {
        if self.samples.is_empty() {
            return None;
        }

        let peak_memory = self
            .samples
            .iter()
            .map(|s| s.memory_bytes)
            .max()
            .unwrap_or(0);

        let avg_memory = self
            .samples
            .iter()
            .map(|s| s.memory_bytes as f64)
            .sum::<f64>()
            / self.samples.len() as f64;

        let peak_cpu = self
            .samples
            .iter()
            .map(|s| s.cpu_percent)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0);

        let avg_cpu = self
            .samples
            .iter()
            .map(|s| s.cpu_percent as f64)
            .sum::<f64>()
            / self.samples.len() as f64;

        let max_threads = self
            .samples
            .iter()
            .map(|s| s.num_threads)
            .max()
            .unwrap_or(1);

        let avg_threads = self
            .samples
            .iter()
            .map(|s| s.num_threads as f64)
            .sum::<f64>()
            / self.samples.len() as f64;

        // Disk I/O is cumulative, so take the last sample
        let total_disk_read = self.samples.last().map(|s| s.disk_read_bytes).unwrap_or(0);

        let total_disk_write = self.samples.last().map(|s| s.disk_write_bytes).unwrap_or(0);

        let duration = self.samples.last().map(|s| s.timestamp_s).unwrap_or(0.0);

        Some(ResourceStats {
            samples: self.samples.clone(),
            peak_memory_mb: peak_memory as f64 / (1024.0 * 1024.0),
            avg_memory_mb: avg_memory / (1024.0 * 1024.0),
            peak_cpu_percent: peak_cpu,
            avg_cpu_percent: avg_cpu as f32,
            total_disk_read_mb: total_disk_read as f64 / (1024.0 * 1024.0),
            total_disk_write_mb: total_disk_write as f64 / (1024.0 * 1024.0),
            max_threads,
            avg_threads,
            duration_s: duration,
        })
    }

    /// Clear all samples
    pub fn clear(&mut self) {
        self.samples.clear();
        self.start_time = Instant::now();
    }
}

/// Spawn a background monitoring task
pub async fn monitor_process_async(
    pid: u32,
    interval_ms: u64,
    stop_signal: tokio::sync::watch::Receiver<bool>,
) -> ResourceStats {
    let mut monitor = ResourceMonitor::new(pid, interval_ms);
    let mut stop_rx = stop_signal;

    loop {
        tokio::select! {
            _ = tokio::time::sleep(monitor.interval) => {
                monitor.record();
            }
            result = stop_rx.changed() => {
                if result.is_err() || *stop_rx.borrow() {
                    break;
                }
            }
        }
    }

    monitor.calculate_stats().unwrap_or_else(|| ResourceStats {
        samples: Vec::new(),
        peak_memory_mb: 0.0,
        avg_memory_mb: 0.0,
        peak_cpu_percent: 0.0,
        avg_cpu_percent: 0.0,
        total_disk_read_mb: 0.0,
        total_disk_write_mb: 0.0,
        max_threads: 0,
        avg_threads: 0.0,
        duration_s: 0.0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process;

    #[test]
    fn test_resource_monitor_self() {
        let pid = process::id();
        let mut monitor = ResourceMonitor::new(pid, 100);

        // Take a few samples
        for _ in 0..5 {
            monitor.record();
            std::thread::sleep(Duration::from_millis(100));
        }

        let stats = monitor.calculate_stats().unwrap();
        assert!(stats.samples.len() >= 5);
        assert!(stats.peak_memory_mb > 0.0);
        assert!(stats.duration_s >= 0.4);
    }

    #[tokio::test]
    async fn test_async_monitoring() {
        let pid = std::process::id();
        let (stop_tx, stop_rx) = tokio::sync::watch::channel(false);

        let monitor_task =
            tokio::spawn(async move { monitor_process_async(pid, 50, stop_rx).await });

        // Let it run for a bit
        tokio::time::sleep(Duration::from_millis(300)).await;

        // Stop monitoring
        stop_tx.send(true).unwrap();

        let stats = monitor_task.await.unwrap();
        assert!(stats.samples.len() >= 5);
    }
}
