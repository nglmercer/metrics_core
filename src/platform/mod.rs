mod common;

#[cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))]
pub use common::*;

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
pub fn get_cpus() -> Vec<CpuMetrics> {
    Vec::new()
}

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
pub fn get_memory() -> MemoryMetrics {
    MemoryMetrics {
        total_bytes: 0,
        free_bytes: 0,
        used_bytes: 0,
        available_bytes: 0,
        swap_total_bytes: 0,
        swap_used_bytes: 0,
    }
}

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
pub fn get_disks() -> Vec<DiskMetrics> {
    Vec::new()
}

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
pub fn get_networks() -> Vec<NetworkMetrics> {
    Vec::new()
}

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
pub fn get_uptime() -> u64 {
    0
}
