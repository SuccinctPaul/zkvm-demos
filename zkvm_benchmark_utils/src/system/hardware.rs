//! Hardware information collection module
//!
//! Collects system hardware information for benchmark reproducibility

use crate::core::metrics::HardwareInfo;
use sysinfo::System;

/// Collect current system hardware information
pub fn collect_hardware_info() -> HardwareInfo {
    let mut sys = System::new_all();
    sys.refresh_all();

    // CPU information
    let cpus = sys.cpus();
    let cpu_brand = if let Some(cpu) = cpus.first() {
        cpu.brand().to_string()
    } else {
        "Unknown".to_string()
    };

    let cpu_cores = sys.cpus().len() as u32;
    let cpu_physical_cores = sys.physical_core_count().unwrap_or(cpu_cores as usize) as u32;

    let cpu_frequency_mhz = cpus.first().map(|cpu| cpu.frequency());

    // Memory information (convert from KB to MB)
    let total_memory_mb = sys.total_memory() / 1024;
    let available_memory_mb = sys.available_memory() / 1024;

    // OS information
    let os_name = System::name().unwrap_or_else(|| "Unknown".to_string());
    let os_version = System::os_version().unwrap_or_else(|| "Unknown".to_string());
    let kernel_version = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());
    let hostname = System::host_name().unwrap_or_else(|| "Unknown".to_string());

    // Architecture
    let arch = std::env::consts::ARCH.to_string();

    HardwareInfo {
        cpu_brand,
        cpu_cores,
        cpu_physical_cores,
        cpu_frequency_mhz,
        total_memory_mb,
        available_memory_mb,
        os_name,
        os_version,
        kernel_version,
        hostname,
        arch,
    }
}

/// Format hardware info as human-readable string
pub fn format_hardware_info(hw: &HardwareInfo) -> String {
    format!(
        "CPU: {} ({} cores, {} physical)\n\
         Memory: {:.1} GB / {:.1} GB available\n\
         OS: {} {} (Kernel: {})\n\
         Arch: {} | Host: {}",
        hw.cpu_brand,
        hw.cpu_cores,
        hw.cpu_physical_cores,
        hw.available_memory_mb as f64 / 1024.0,
        hw.total_memory_mb as f64 / 1024.0,
        hw.os_name,
        hw.os_version,
        hw.kernel_version,
        hw.arch,
        hw.hostname
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_hardware_info() {
        let hw = collect_hardware_info();

        // Basic sanity checks
        assert!(hw.cpu_cores > 0);
        assert!(hw.total_memory_mb > 0);
        assert!(!hw.os_name.is_empty());
        assert!(!hw.arch.is_empty());
    }

    #[test]
    fn test_format_hardware_info() {
        let hw = collect_hardware_info();
        let formatted = format_hardware_info(&hw);

        assert!(formatted.contains("CPU:"));
        assert!(formatted.contains("Memory:"));
        assert!(formatted.contains("OS:"));
    }
}
