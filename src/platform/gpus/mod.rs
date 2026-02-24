use crate::types::GpuMetrics;

#[cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))]
mod nvidia;

#[cfg(target_os = "linux")]
mod linux;

pub fn get_gpus() -> Vec<GpuMetrics> {
    let mut results = Vec::new();

    // Try NVIDIA first (Cross-platform)
    #[cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))]
    {
        let nvidia_results = nvidia::get_nvidia_gpus();
        if !nvidia_results.is_empty() {
            results.extend(nvidia_results);
        }
    }

    // Platform-specific discovery
    #[cfg(target_os = "linux")]
    {
        // Try AMD GPUs on Linux
        let amd_results = linux::get_amd_gpus();
        if !amd_results.is_empty() {
            results.extend(amd_results);
        }

        // If no GPUs found, try to detect any GPU from sysfs
        if results.is_empty() {
            let sysfs_results = linux::get_gpus_from_sysfs();
            results.extend(sysfs_results);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpus() {
        let gpus = get_gpus();
        println!("Found {} GPUs", gpus.len());
        for gpu in &gpus {
            println!(
                "GPU {}: {} - Utilization: {}%, Temp: {}°C",
                gpu.index, gpu.name, gpu.utilization_gpu_pct, gpu.temperature_celsius
            );
        }
    }
}
