use crate::types::{
    BatteryInfo, ComponentMetrics, CpuMetrics, DiskIoMetrics, DiskMetrics, LoadAverage,
    MemoryMetrics, NetworkIoMetrics, NetworkMetrics, OsInfo, ProcessMetrics, AllMetrics,
};
use std::sync::RwLock;
use std::sync::OnceLock;
use sysinfo::{Components, CpuRefreshKind, Disks, MemoryRefreshKind, Networks, System};

static SYSTEM: OnceLock<RwLock<System>> = OnceLock::new();
static DISKS: OnceLock<RwLock<Disks>> = OnceLock::new();
static NETWORKS: OnceLock<RwLock<Networks>> = OnceLock::new();
static COMPONENTS: OnceLock<RwLock<Components>> = OnceLock::new();

fn get_system() -> &'static RwLock<System> {
    SYSTEM.get_or_init(|| {
        RwLock::new(System::new())
    })
}

fn get_disks_obj() -> &'static RwLock<Disks> {
    DISKS.get_or_init(|| RwLock::new(Disks::new_with_refreshed_list()))
}

fn get_networks_obj() -> &'static RwLock<Networks> {
    NETWORKS.get_or_init(|| RwLock::new(Networks::new_with_refreshed_list()))
}

fn get_components_obj() -> &'static RwLock<Components> {
    COMPONENTS.get_or_init(|| RwLock::new(Components::new_with_refreshed_list()))
}

pub fn get_cpus() -> Vec<CpuMetrics> {
    let sys_rwlock = get_system();
    {
        let mut sys = sys_rwlock.write().unwrap();
        sys.refresh_cpu_specifics(CpuRefreshKind::new().with_cpu_usage());
    }

    let sys = sys_rwlock.read().unwrap();
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
    let sys_rwlock = get_system();
    {
        let mut sys = sys_rwlock.write().unwrap();
        sys.refresh_memory_specifics(MemoryRefreshKind::new().with_ram());
    }

    let sys = sys_rwlock.read().unwrap();
    MemoryMetrics {
        total_bytes: sys.total_memory(),
        free_bytes: sys.free_memory(),
        used_bytes: sys.used_memory(),
        available_bytes: sys.available_memory(),
        swap_total_bytes: sys.total_swap(),
        swap_used_bytes: sys.used_swap(),
    }
}

pub fn get_disks() -> Vec<DiskMetrics> {
    let disks_rwlock = get_disks_obj();
    {
        let mut disks = disks_rwlock.write().unwrap();
        disks.refresh_list();
    }

    let disks = disks_rwlock.read().unwrap();
    disks
        .iter()
        .map(|disk| DiskMetrics {
            name: disk.name().to_string_lossy().into_owned(),
            total_space: disk.total_space(),
            available_space: disk.available_space(),
            used_space: disk.total_space() - disk.available_space(),
            mount_point: disk.mount_point().to_string_lossy().into_owned(),
            file_system: disk.file_system().to_string_lossy().into_owned(),
        })
        .collect()
}

pub fn get_networks() -> Vec<NetworkMetrics> {
    let networks_rwlock = get_networks_obj();
    {
        let mut networks = networks_rwlock.write().unwrap();
        networks.refresh();
    }

    let networks = networks_rwlock.read().unwrap();
    networks
        .iter()
        .map(|(name, data)| NetworkMetrics {
            interface: name.clone(),
            received_bytes: data.received(),
            transmitted_bytes: data.transmitted(),
            packets_received: data.packets_received(),
            packets_transmitted: data.packets_transmitted(),
        })
        .collect()
}

pub fn get_uptime() -> u64 {
    System::uptime()
}

pub fn get_os_info() -> OsInfo {
    OsInfo {
        name: System::name().unwrap_or_default(),
        kernel_version: System::kernel_version().unwrap_or_default(),
        os_version: System::os_version().unwrap_or_default(),
        host_name: System::host_name().unwrap_or_default(),
    }
}

pub fn get_components() -> Vec<ComponentMetrics> {
    let components_rwlock = get_components_obj();
    {
        let mut components = components_rwlock.write().unwrap();
        components.refresh_list();
        components.refresh();
    }

    let components = components_rwlock.read().unwrap();
    components
        .iter()
        .map(|c| ComponentMetrics {
            label: c.label().to_string(),
            temperature: c.temperature(),
            max: c.max(),
            critical: c.critical(),
        })
        .collect()
}

pub fn get_processes() -> Vec<ProcessMetrics> {
    let sys_rwlock = get_system();
    {
        let mut sys = sys_rwlock.write().unwrap();
        sys.refresh_processes();
    }

    let sys = sys_rwlock.read().unwrap();
    sys.processes()
        .iter()
        .map(|(pid, process)| ProcessMetrics {
            pid: pid.as_u32(),
            name: process.name().to_string(),
            cpu_usage: process.cpu_usage(),
            memory_bytes: process.memory(),
            disk_read_bytes: process.disk_usage().read_bytes,
            disk_written_bytes: process.disk_usage().written_bytes,
            status: process.status().to_string(),
            user_id: process.user_id().map(|u| u.to_string()),
        })
        .collect()
}

pub fn get_process_by_pid(pid: u32) -> Option<ProcessMetrics> {
    let sys_rwlock = get_system();
    let pid_obj = sysinfo::Pid::from(pid as usize);
    
    {
        let mut sys = sys_rwlock.write().unwrap();
        if !sys.refresh_process(pid_obj) {
            return None;
        }
    }

    let sys = sys_rwlock.read().unwrap();
    sys.process(pid_obj).map(|process| ProcessMetrics {
        pid,
        name: process.name().to_string(),
        cpu_usage: process.cpu_usage(),
        memory_bytes: process.memory(),
        disk_read_bytes: process.disk_usage().read_bytes,
        disk_written_bytes: process.disk_usage().written_bytes,
        status: process.status().to_string(),
        user_id: process.user_id().map(|u| u.to_string()),
    })
}

pub fn get_disk_io() -> DiskIoMetrics {
    let sys_rwlock = get_system();
    {
        let mut sys = sys_rwlock.write().unwrap();
        sys.refresh_processes();
    }

    let sys = sys_rwlock.read().unwrap();
    let mut read = 0;
    let mut written = 0;

    for process in sys.processes().values() {
        let usage = process.disk_usage();
        read += usage.read_bytes;
        written += usage.written_bytes;
    }

    DiskIoMetrics {
        read_bytes: read,
        written_bytes: written,
    }
}

pub fn get_network_io() -> NetworkIoMetrics {
    let networks_rwlock = get_networks_obj();
    {
        let mut networks = networks_rwlock.write().unwrap();
        networks.refresh();
    }

    let networks = networks_rwlock.read().unwrap();
    let mut rx = 0;
    let mut tx = 0;
    let mut rx_p = 0;
    let mut tx_p = 0;

    for data in networks.values() {
        rx += data.received();
        tx += data.transmitted();
        rx_p += data.packets_received();
        tx_p += data.packets_transmitted();
    }

    NetworkIoMetrics {
        rx_bytes: rx,
        tx_bytes: tx,
        rx_packets: rx_p,
        tx_packets: tx_p,
    }
}

pub fn get_batteries() -> Vec<BatteryInfo> {
    let mut results = Vec::new();
    let manager = match starship_battery::Manager::new() {
        Ok(m) => m,
        Err(_) => return Vec::new(),
    };

    let batteries = match manager.batteries() {
        Ok(b) => b,
        Err(_) => return Vec::new(),
    };

    for battery in batteries.flatten() {
        results.push(BatteryInfo {
            state: format!("{:?}", battery.state()),
            vendor: battery.vendor().map(|s| s.to_string()),
            model: battery.model().map(|s| s.to_string()),
            cycle_count: battery.cycle_count(),
            health_pct: battery.state_of_health().value * 100.0,
            energy_pct: battery.state_of_charge().value * 100.0,
            energy_full_design_wh: battery.energy_full_design().value,
            energy_full_wh: battery.energy_full().value,
            energy_wh: battery.energy().value,
        });
    }

    results
}

pub fn get_load_average() -> LoadAverage {
    let load = System::load_average();
    LoadAverage {
        one_min: load.one,
        five_min: load.five,
        fifteen_min: load.fifteen,
    }
}

pub fn get_all_metrics() -> AllMetrics {
    AllMetrics {
        cpu: get_cpus(),
        memory: get_memory(),
        disks: get_disks(),
        networks: get_networks(),
        uptime: get_uptime(),
        os_info: get_os_info(),
        load_avg: get_load_average(),
    }
}

pub fn cleanup_metrics() {
    // Note: OnceLock cannot be "cleared" currently in stable Rust.
    // This function can be expanded if we switch to a different lazy init strategy
    // or when OnceLock::take() becomes stable (though it's not clear it will).
    // For now, this is a placeholder to satisfy the FFI requirement.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_os_info() {
        let os = get_os_info();
        assert!(!os.name.is_empty());
        println!("OS Name: {}", os.name);
        println!("Kernel: {}", os.kernel_version);
    }

    #[test]
    fn test_load_average() {
        let load = get_load_average();
        println!("Load Average: 1m={}, 5m={}, 15m={}", load.one_min, load.five_min, load.fifteen_min);
    }

    #[test]
    fn test_all_metrics() {
        let all = get_all_metrics();
        assert!(!all.cpu.is_empty());
        println!("All metrics: CPU count={}, Memory used={}", all.cpu.len(), all.memory.used_bytes);
    }

    #[test]
    fn test_components_output() {
        let components = get_components();
        println!("Found {} components", components.len());
        for c in &components {
            println!(
                "Label: {}, Temp: {}°C, Max: {}°C",
                c.label, c.temperature, c.max
            );
        }
    }

    #[test]
    fn test_processes() {
        let processes = get_processes();
        assert!(!processes.is_empty());
        println!("Found {} processes", processes.len());
        if let Some(p) = processes.first() {
            println!("First process: {} (PID: {})", p.name, p.pid);
        }
    }

    #[test]
    fn test_get_process_by_pid() {
        let processes = get_processes();
        if let Some(first) = processes.first() {
            let pid = first.pid;
            let p_info = get_process_by_pid(pid);
            assert!(p_info.is_some());
            let p = p_info.unwrap();
            assert_eq!(p.pid, pid);
            println!("Verified process info for PID {}: {}", pid, p.name);
            println!("  CPU: {}%, MEM: {} bytes", p.cpu_usage, p.memory_bytes);
        }
    }

    #[test]
    fn test_disk_io() {
        let io = get_disk_io();
        println!(
            "Global Disk I/O - Read: {} bytes, Written: {} bytes",
            io.read_bytes, io.written_bytes
        );
    }

    #[test]
    fn test_network_io() {
        let io = get_network_io();
        let networks_rwlock = get_networks_obj();
        let networks = networks_rwlock.read().unwrap();
        println!("Found {} interfaces", networks.len());
        println!(
            "Global Network I/O - RX: {} bytes, TX: {} bytes",
            io.rx_bytes, io.tx_bytes
        );
    }

    #[test]
    fn test_batteries() {
        let batteries = get_batteries();
        println!("Found {} batteries", batteries.len());
        for b in batteries {
            println!(
                "Battery: {} {}, State: {}, Energy: {}%",
                b.vendor.unwrap_or_default(),
                b.model.unwrap_or_default(),
                b.state,
                b.energy_pct
            );
        }
    }
}
