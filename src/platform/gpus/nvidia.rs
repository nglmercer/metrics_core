use crate::types::GpuMetrics;

/// Returns GPU metrics using NVML (NVIDIA GPUs only)
pub fn get_nvidia_gpus() -> Vec<GpuMetrics> {
    // Try to initialize NVML
    let nvml = match nvml_wrapper::Nvml::init() {
        Ok(n) => n,
        Err(_) => return Vec::new(), // No NVIDIA GPU or NVML not available
    };

    // Get device count
    let device_count = match nvml.device_count() {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    let mut results = Vec::new();
    for i in 0..device_count {
        if let Ok(device) = nvml.device_by_index(i) {
            let memory_info = device.memory_info().ok();
            let utilization = device.utilization_rates().ok();

            let gpu = GpuMetrics {
                index: i,
                name: device.name().unwrap_or_else(|_| "Unknown".to_string()),
                brand: "NVIDIA".to_string(),
                driver_version: nvml
                    .sys_driver_version()
                    .unwrap_or_else(|_| "Unknown".to_string()),
                memory_total_bytes: memory_info.as_ref().map(|m| m.total).unwrap_or(0),
                memory_used_bytes: memory_info.as_ref().map(|m| m.used).unwrap_or(0),
                memory_free_bytes: memory_info.as_ref().map(|m| m.free).unwrap_or(0),
                utilization_gpu_pct: utilization.as_ref().map(|u| u.gpu as f32).unwrap_or(0.0),
                utilization_memory_pct: utilization
                    .as_ref()
                    .map(|u| u.memory as f32)
                    .unwrap_or(0.0),
                temperature_celsius: device
                    .temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)
                    .unwrap_or(0) as f32,
                power_usage_watts: device
                    .power_usage()
                    .map(|p| p as f32 / 1000.0)
                    .unwrap_or(0.0),
                power_limit_watts: device
                    .power_management_limit()
                    .map(|p| p as f32 / 1000.0)
                    .unwrap_or(0.0),
                fan_speed_pct: device.fan_speed(0).ok(),
            };
            results.push(gpu);
        }
    }

    results
}
