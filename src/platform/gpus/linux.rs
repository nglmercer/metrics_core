use crate::types::GpuMetrics;
use std::fs;
use std::path::{Path, PathBuf};

/// Returns AMD GPU metrics by reading from sysfs and hwmon
pub fn get_amd_gpus() -> Vec<GpuMetrics> {
    let mut results = Vec::new();
    let mut gpu_index: u32 = 0;

    // Read AMD GPU info from /sys/class/drm/
    let drm_path = PathBuf::from("/sys/class/drm");
    if let Ok(entries) = fs::read_dir(&drm_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

            // Look for AMD GPU card entries (card0, card1, card2, etc.) - no hyphen means it's a card
            if name.starts_with("card") && !name.contains("-") {
                let sysfs_path = path.join("device");

                // Check if this is an AMD GPU by reading the vendor
                let vendor_path = sysfs_path.join("vendor");

                // AMD vendor ID is 0x1002
                let is_amd = fs::read_to_string(&vendor_path)
                    .map(|v| v.trim() == "0x1002")
                    .unwrap_or(false);

                if is_amd {
                    // Get GPU name from uevent PCI_ID
                    let name = fs::read_to_string(sysfs_path.join("uevent"))
                        .ok()
                        .and_then(|uevent| {
                            // Extract PCI_ID from uevent (format: PCI_ID=1002:13FE)
                            uevent
                                .lines()
                                .find(|l| l.starts_with("PCI_ID="))
                                .map(|l| l.trim_start_matches("PCI_ID=").to_string())
                        })
                        .map(|pci_id| format_amd_gpu_name(&pci_id))
                        .unwrap_or_else(|| "AMD GPU".to_string());

                    // Read VRAM memory info from sysfs (more reliable than meminfo)
                    let vram_total = read_sysfs_u64(&sysfs_path.join("mem_info_vram_total"));
                    let vram_used = read_sysfs_u64(&sysfs_path.join("mem_info_vram_used"));

                    // Also try GTT memory as fallback
                    let gtt_total = read_sysfs_u64(&sysfs_path.join("mem_info_gtt_total"));
                    let gtt_used = read_sysfs_u64(&sysfs_path.join("mem_info_gtt_used"));

                    // Use VRAM if available, otherwise use GTT
                    let total_mem = if vram_total > 0 {
                        vram_total
                    } else {
                        gtt_total
                    };
                    let used_mem = if vram_used > 0 { vram_used } else { gtt_used };

                    // Get temperature from hwmon
                    let hwmon_path = find_hwmon_for_device(&sysfs_path);
                    let temperature = read_hwmon_temp(&hwmon_path);

                    // Get GPU utilization from hwmon (might be available in some AMD GPUs)
                    let utilization = read_amdgpu_utilization(&sysfs_path);

                    let driver_version = fs::read_to_string("/sys/module/amdgpu/version")
                        .map(|v| v.trim().to_string())
                        .unwrap_or_else(|_| "Unknown".to_string());

                    // Get power usage if available
                    let (power_usage, power_limit) = read_amdgpu_power(&hwmon_path);

                    results.push(GpuMetrics {
                        index: gpu_index,
                        name,
                        brand: "AMD".to_string(),
                        driver_version,
                        memory_total_bytes: total_mem,
                        memory_used_bytes: used_mem,
                        memory_free_bytes: total_mem.saturating_sub(used_mem),
                        utilization_gpu_pct: utilization,
                        utilization_memory_pct: 0.0, // Not easily available
                        temperature_celsius: temperature,
                        power_usage_watts: power_usage,
                        power_limit_watts: power_limit,
                        fan_speed_pct: read_hwmon_fan(&hwmon_path),
                    });

                    gpu_index += 1;
                }
            }
        }
    }

    results
}

/// Fallback GPU detection from sysfs (for any GPU not caught by other methods)
pub fn get_gpus_from_sysfs() -> Vec<GpuMetrics> {
    let mut results = Vec::new();
    let mut gpu_index: u32 = 0;

    // Look for GPUs in /sys/bus/pci/devices/
    let pci_path = PathBuf::from("/sys/bus/pci/devices");
    if let Ok(entries) = fs::read_dir(&pci_path) {
        for entry in entries.flatten() {
            let path = entry.path();

            // Check if this is a VGA compatible controller (class 0x030000)
            let class_path = path.join("class");
            if let Ok(class) = fs::read_to_string(&class_path) {
                let class_trimmed = class.trim();
                if class_trimmed.starts_with("0x0300") {
                    // Get vendor ID
                    let vendor_path = path.join("vendor");
                    let vendor = fs::read_to_string(&vendor_path)
                        .map(|v| v.trim().to_string())
                        .unwrap_or_default();

                    // Skip if we already detected this via NVIDIA or AMD
                    if vendor == "0x10de" || vendor == "0x1002" {
                        continue;
                    }

                    // Get device name
                    let device_path = path.join("device");
                    let device = fs::read_to_string(&device_path)
                        .map(|d| d.trim().to_string())
                        .unwrap_or_else(|_| "Unknown".to_string());

                    // Get vendor name
                    let vendor_name = match vendor.as_str() {
                        "0x8086" => "Intel",
                        "0x1002" => "AMD",
                        "0x10de" => "NVIDIA",
                        _ => "Unknown",
                    };

                    if vendor_name != "Unknown" {
                        let name = format!("{} GPU {}", vendor_name, device);

                        results.push(GpuMetrics {
                            index: gpu_index,
                            name,
                            brand: vendor_name.to_string(),
                            driver_version: "Unknown".to_string(),
                            memory_total_bytes: 0,
                            memory_used_bytes: 0,
                            memory_free_bytes: 0,
                            utilization_gpu_pct: 0.0,
                            utilization_memory_pct: 0.0,
                            temperature_celsius: 0.0,
                            power_usage_watts: 0.0,
                            power_limit_watts: 0.0,
                            fan_speed_pct: None,
                        });

                        gpu_index += 1;
                    }
                }
            }
        }
    }

    results
}

fn format_amd_gpu_name(pci_id: &str) -> String {
    let name = match pci_id.to_uppercase().as_str() {
        "1002:13FE" => "AMD Radeon RX 6750 GRE",
        "1002:1478" => "AMD Radeon RX 6800",
        "1002:1479" => "AMD Radeon RX 6800 XT",
        "1002:147A" => "AMD Radeon RX 6800M",
        "1002:147B" => "AMD Radeon RX 6850M XT",
        "1002:1480" => "AMD Radeon RX 6900 XT",
        "1002:1481" => "AMD Radeon RX 6900 XTX",
        "1002:1485" => "AMD Radeon RX 6600 XT",
        "1002:1486" => "AMD Radeon RX 6600",
        "1002:1487" => "AMD Radeon RX 6600M",
        "1002:1488" => "AMD Radeon RX 6500 XT",
        "1002:1489" => "AMD Radeon RX 6500",
        "1002:15BF" => "AMD Radeon RX 7900 XTX",
        "1002:15C0" => "AMD Radeon RX 7900 XT",
        "1002:15C1" => "AMD Radeon RX 7900 GRE",
        "1002:15C2" => "AMD Radeon RX 7800 XT",
        "1002:15C3" => "AMD Radeon RX 7800M",
        "1002:15C4" => "AMD Radeon RX 7700 XT",
        "1002:15C5" => "AMD Radeon RX 7700M",
        "1002:15C6" => "AMD Radeon RX 7600 XT",
        "1002:15C7" => "AMD Radeon RX 7600",
        "1002:15C8" => "AMD Radeon RX 7600M",
        "1002:15D8" => "AMD Radeon RX 7500 XT",
        "1002:15D9" => "AMD Radeon RX 7600M XT",
        "1002:15E3" => "AMD Radeon RX 7700S",
        "1002:15E7" => "AMD Radeon RX 7600S",
        "1002:6810" => "AMD Radeon RX 590",
        "1002:6811" => "AMD Radeon RX 580",
        "1002:6812" => "AMD Radeon RX 570",
        "1002:6818" => "AMD Radeon RX 560",
        "1002:6819" => "AMD Radeon RX 560",
        "1002:6820" => "AMD Radeon RX 550",
        "1002:6821" => "AMD Radeon RX 550",
        "1002:6823" => "AMD Radeon RX 540",
        "1002:687F" => "AMD Radeon Vega Frontier Edition",
        "1002:6880" => "AMD Radeon RX Vega 56",
        "1002:6881" => "AMD Radeon RX Vega 64",
        "1002:6888" => "AMD Radeon Vega 56 Mobile",
        "1002:6889" => "AMD Radeon Vega 64 Mobile",
        "1002:69AF" => "AMD Radeon VII",
        "1002:15DD" => "AMD Radeon RX Vega 11 (APU)",
        "1002:15D7" => "AMD Radeon Vega 8 (APU)",
        "1002:15D6" => "AMD Radeon Vega 6 (APU)",
        "1002:7340" => "AMD Radeon RX 7600M XT",
        "1002:7341" => "AMD Radeon RX 7600M",
        "1002:7347" => "AMD Radeon RX 7700M",
        _ => {
            if pci_id.contains(':') {
                let parts: Vec<&str> = pci_id.split(':').collect();
                if parts.len() == 2 {
                    let device_id = parts[1].trim_start_matches('0');
                    if !device_id.is_empty() {
                        return format!("AMD GPU {}", device_id);
                    }
                }
            }
            "AMD GPU"
        }
    };
    name.to_string()
}

fn read_sysfs_u64(path: &Path) -> u64 {
    fs::read_to_string(path)
        .ok()
        .and_then(|s| s.trim().parse::<u64>().ok())
        .unwrap_or(0)
}

fn find_hwmon_for_device(device_path: &Path) -> Option<PathBuf> {
    let hwmon_path = device_path.join("hwmon");
    if hwmon_path.exists() {
        if let Ok(read_dir) = fs::read_dir(&hwmon_path) {
            for entry in read_dir.flatten() {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();
                if name_str.starts_with("hwmon") {
                    return Some(entry.path());
                }
            }
        }
    }

    if let Some(parent) = device_path.parent() {
        return find_hwmon_for_device(parent);
    }

    None
}

fn read_hwmon_temp(path: &Option<PathBuf>) -> f32 {
    let path = match path {
        Some(p) => p,
        None => return 0.0,
    };

    for i in 1..=10 {
        let temp_path = path.join(format!("temp{}_input", i));
        if temp_path.exists() {
            if let Ok(content) = fs::read_to_string(&temp_path) {
                if let Ok(temp_millidegrees) = content.trim().parse::<i32>() {
                    return temp_millidegrees as f32 / 1000.0;
                }
            }
        }
    }

    let edge_path = path.join("temp1_edge");
    if edge_path.exists() {
        if let Ok(content) = fs::read_to_string(&edge_path) {
            if let Ok(temp) = content.trim().parse::<i32>() {
                return temp as f32;
            }
        }
    }

    0.0
}

fn read_hwmon_fan(path: &Option<PathBuf>) -> Option<u32> {
    let path = match path {
        Some(p) => p,
        None => return None,
    };

    let fan_path = path.join("fan1_input");
    if fan_path.exists() {
        if let Ok(content) = fs::read_to_string(&fan_path) {
            return content.trim().parse::<u32>().ok();
        }
    }

    None
}

fn read_amdgpu_utilization(device_path: &Path) -> f32 {
    let busy_path = device_path.join("gpu_busy_percent");
    if busy_path.exists() {
        if let Ok(content) = fs::read_to_string(&busy_path) {
            return content.trim().parse::<f32>().unwrap_or(0.0);
        }
    }

    let busy_path_alt = device_path.join("metrics/gpu_busy_percent");
    if busy_path_alt.exists() {
        if let Ok(content) = fs::read_to_string(&busy_path_alt) {
            return content.trim().parse::<f32>().unwrap_or(0.0);
        }
    }

    0.0
}

fn read_amdgpu_power(path: &Option<PathBuf>) -> (f32, f32) {
    let path = match path {
        Some(p) => p,
        None => return (0.0, 0.0),
    };

    let power_path = path.join("power1_average");
    let power_input_path = path.join("power1_input");

    let mut power_usage = 0.0;

    if power_path.exists() {
        if let Ok(content) = fs::read_to_string(&power_path) {
            if let Ok(pw) = content.trim().parse::<i64>() {
                power_usage = pw as f32 / 1_000_000.0;
            }
        }
    }

    if power_input_path.exists() && power_usage == 0.0 {
        if let Ok(content) = fs::read_to_string(&power_input_path) {
            if let Ok(pw) = content.trim().parse::<i64>() {
                power_usage = pw as f32 / 1_000_000.0;
            }
        }
    }

    (power_usage, 0.0)
}
