use crate::types::{CpuMetrics, DiskMetrics, MemoryMetrics, NetworkMetrics};
use sysinfo::{Disks, Networks, System};

pub fn get_cpus() -> Vec<CpuMetrics> {
    let mut sys = System::new_all();
    sys.refresh_cpu_usage();
    sys.cpus()
        .iter()
        .map(|cpu| CpuMetrics {
            usage_pct: cpu.cpu_usage(),
            brand: cpu.brand().to_string(),
            frequency: cpu.frequency(),
        })
        .collect()
}

pub fn get_memory() -> MemoryMetrics {
    let mut sys = System::new_all();
    sys.refresh_memory();
    MemoryMetrics {
        total_kb: sys.total_memory(),
        free_kb: sys.free_memory(),
        used_kb: sys.used_memory(),
        available_kb: sys.available_memory(),
    }
}

pub fn get_disks() -> Vec<DiskMetrics> {
    let disks_obj = Disks::new_with_refreshed_list();
    disks_obj
        .iter()
        .map(|disk| DiskMetrics {
            name: disk.name().to_string_lossy().into_owned(),
            total_space: disk.total_space(),
            available_space: disk.available_space(),
            mount_point: disk.mount_point().to_string_lossy().into_owned(),
        })
        .collect()
}

pub fn get_networks() -> Vec<NetworkMetrics> {
    let networks_obj = Networks::new_with_refreshed_list();
    networks_obj
        .iter()
        .map(|(name, data)| NetworkMetrics {
            interface: name.clone(),
            received_bytes: data.received(),
            transmitted_bytes: data.transmitted(),
        })
        .collect()
}

pub fn get_uptime() -> u64 {
    System::uptime()
}
