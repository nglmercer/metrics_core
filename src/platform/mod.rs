mod common;

#[cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))]
pub use common::*;

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
use crate::types::*;

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

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
pub fn get_os_info() -> OsInfo {
    OsInfo {
        name: String::new(),
        kernel_version: String::new(),
        os_version: String::new(),
        host_name: String::new(),
    }
}

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
pub fn get_components() -> Vec<ComponentMetrics> {
    Vec::new()
}

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
pub fn get_processes() -> Vec<ProcessMetrics> {
    Vec::new()
}

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
pub fn get_process_by_pid(_pid: u32) -> Option<ProcessMetrics> {
    None
}

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
pub fn get_disk_io() -> DiskIoMetrics {
    DiskIoMetrics {
        read_bytes: 0,
        written_bytes: 0,
    }
}

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
pub fn get_network_io() -> NetworkIoMetrics {
    NetworkIoMetrics {
        rx_bytes: 0,
        tx_bytes: 0,
        rx_packets: 0,
        tx_packets: 0,
    }
}

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
pub fn get_batteries() -> Vec<BatteryInfo> {
    Vec::new()
}

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
pub fn get_load_average() -> LoadAverage {
    LoadAverage {
        one_min: 0.0,
        five_min: 0.0,
        fifteen_min: 0.0,
    }
}

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
pub fn get_all_metrics() -> AllMetrics {
    AllMetrics {
        cpu: Vec::new(),
        memory: get_memory(),
        disks: Vec::new(),
        networks: Vec::new(),
        uptime: 0,
        os_info: get_os_info(),
        load_avg: get_load_average(),
    }
}

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
pub fn cleanup_metrics() {}
