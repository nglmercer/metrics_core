use crate::types::{AllMetrics, CpuMetrics, DiskMetrics, MemoryMetrics, NetworkMetrics};
use sysinfo::{Disks, Networks, System};

pub fn get_metrics() -> AllMetrics {
    let mut sys = System::new_all();
    sys.refresh_all();

    let cpus: Vec<CpuMetrics> = sys
        .cpus()
        .iter()
        .map(|cpu| CpuMetrics {
            usage_pct: cpu.cpu_usage(),
            brand: cpu.brand().to_string(),
            frequency: cpu.frequency(),
        })
        .collect();

    let memory = MemoryMetrics {
        total_kb: sys.total_memory(),
        free_kb: sys.free_memory(),
        used_kb: sys.used_memory(),
        available_kb: sys.available_memory(),
    };

    let disks_obj = Disks::new_with_refreshed_list();
    let disks: Vec<DiskMetrics> = disks_obj
        .iter()
        .map(|disk| DiskMetrics {
            name: disk.name().to_string_lossy().into_owned(),
            total_space: disk.total_space(),
            available_space: disk.available_space(),
            mount_point: disk.mount_point().to_string_lossy().into_owned(),
        })
        .collect();

    let networks_obj = Networks::new_with_refreshed_list();
    let networks: Vec<NetworkMetrics> = networks_obj
        .iter()
        .map(|(name, data)| NetworkMetrics {
            interface: name.clone(),
            received_bytes: data.received(),
            transmitted_bytes: data.transmitted(),
        })
        .collect();

    let uptime_sec = System::uptime();

    AllMetrics {
        cpus,
        memory,
        disks,
        networks,
        uptime_sec,
    }
}
