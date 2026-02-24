use crate::types::{
    AllMetrics, BatteryInfo, ComponentMetrics, CpuCoreTemperature, CpuMetrics, DiskIoMetrics,
    DiskMetrics, ExtendedProcessMetrics, LoadAverage, MemoryMetrics, NetworkIoMetrics,
    NetworkMetrics, OsInfo, ProcessMetrics,
};
use std::sync::OnceLock;
use std::sync::RwLock;
use sysinfo::{
    Components, CpuRefreshKind, Disks, MemoryRefreshKind, Networks, ProcessesToUpdate, System,
};

static SYSTEM: OnceLock<RwLock<System>> = OnceLock::new();
static DISKS: OnceLock<RwLock<Disks>> = OnceLock::new();
static NETWORKS: OnceLock<RwLock<Networks>> = OnceLock::new();
static COMPONENTS: OnceLock<RwLock<Components>> = OnceLock::new();

// Cache for static OS info (doesn't change often)
static OS_INFO_CACHE: OnceLock<OsInfo> = OnceLock::new();

fn get_system() -> &'static RwLock<System> {
    SYSTEM.get_or_init(|| RwLock::new(System::new()))
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

/// Refresh internal caches based on flags
pub fn refresh_metrics(flags: u32) {
    // CPU refresh
    if flags & 1 != 0 {
        if let Some(sys) = SYSTEM.get() {
            if let Ok(mut sys) = sys.write() {
                sys.refresh_cpu_specifics(CpuRefreshKind::new().with_cpu_usage().with_frequency());
            }
        }
    }

    // Memory refresh
    if flags & 2 != 0 {
        if let Some(sys) = SYSTEM.get() {
            if let Ok(mut sys) = sys.write() {
                sys.refresh_memory_specifics(MemoryRefreshKind::new().with_ram());
            }
        }
    }

    // Disks refresh
    if flags & 4 != 0 {
        if let Some(disks) = DISKS.get() {
            if let Ok(mut disks) = disks.write() {
                disks.refresh_list();
            }
        }
    }

    // Networks refresh
    if flags & 8 != 0 {
        if let Some(networks) = NETWORKS.get() {
            if let Ok(mut networks) = networks.write() {
                networks.refresh();
            }
        }
    }

    // Processes refresh
    if flags & 16 != 0 {
        if let Some(sys) = SYSTEM.get() {
            if let Ok(mut sys) = sys.write() {
                sys.refresh_processes(ProcessesToUpdate::All, true);
            }
        }
    }

    // Components refresh
    if flags & 32 != 0 {
        if let Some(components) = COMPONENTS.get() {
            if let Ok(mut components) = components.write() {
                components.refresh_list();
                components.refresh();
            }
        }
    }
}

pub fn get_cpus() -> Vec<CpuMetrics> {
    let sys_rwlock = get_system();
    {
        let mut sys = sys_rwlock.write().unwrap_or_else(|e| e.into_inner());
        sys.refresh_cpu_specifics(CpuRefreshKind::new().with_cpu_usage().with_frequency());
    }

    let sys = sys_rwlock.read().unwrap_or_else(|e| e.into_inner());
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
        let mut sys = sys_rwlock.write().unwrap_or_else(|e| e.into_inner());
        sys.refresh_memory_specifics(MemoryRefreshKind::new().with_ram());
    }

    let sys = sys_rwlock.read().unwrap_or_else(|e| e.into_inner());
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
        let mut disks = disks_rwlock.write().unwrap_or_else(|e| e.into_inner());
        disks.refresh_list();
    }

    let disks = disks_rwlock.read().unwrap_or_else(|e| e.into_inner());
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
        let mut networks = networks_rwlock.write().unwrap_or_else(|e| e.into_inner());
        networks.refresh();
    }

    let networks = networks_rwlock.read().unwrap_or_else(|e| e.into_inner());
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
    // Use cached OS info if available
    if let Some(cached) = OS_INFO_CACHE.get() {
        return cached.clone();
    }

    let os_info = OsInfo {
        name: System::name().unwrap_or_default(),
        kernel_version: System::kernel_version().unwrap_or_default(),
        os_version: System::os_version().unwrap_or_default(),
        host_name: System::host_name().unwrap_or_default(),
    };

    // Cache the OS info
    let _ = OS_INFO_CACHE.set(os_info.clone());
    os_info
}

pub fn get_components() -> Vec<ComponentMetrics> {
    let components_rwlock = get_components_obj();
    {
        let mut components = components_rwlock.write().unwrap_or_else(|e| e.into_inner());
        components.refresh_list();
        components.refresh();
    }

    let components = components_rwlock.read().unwrap_or_else(|e| e.into_inner());
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
        let mut sys = sys_rwlock.write().unwrap_or_else(|e| e.into_inner());
        sys.refresh_processes(ProcessesToUpdate::All, true);
    }

    let sys = sys_rwlock.read().unwrap_or_else(|e| e.into_inner());
    sys.processes()
        .iter()
        .map(|(pid, process)| ProcessMetrics {
            pid: pid.as_u32(),
            name: process.name().to_string_lossy().into_owned(),
            cpu_usage: process.cpu_usage(),
            memory_bytes: process.memory(),
            disk_read_bytes: process.disk_usage().read_bytes,
            disk_written_bytes: process.disk_usage().written_bytes,
            status: process.status().to_string(),
            user_id: process.user_id().map(|u| u.to_string()),
        })
        .collect()
}

pub fn get_extended_processes() -> Vec<ExtendedProcessMetrics> {
    let sys_rwlock = get_system();
    {
        let mut sys = sys_rwlock.write().unwrap_or_else(|e| e.into_inner());
        sys.refresh_processes(ProcessesToUpdate::All, true);
    }

    let sys = sys_rwlock.read().unwrap_or_else(|e| e.into_inner());
    sys.processes()
        .iter()
        .map(|(pid, process)| ExtendedProcessMetrics {
            pid: pid.as_u32(),
            parent_pid: process.parent().map(|p| p.as_u32()),
            name: process.name().to_string_lossy().into_owned(),
            command: process
                .cmd()
                .first()
                .map(|c| c.to_string_lossy().into_owned()),
            cpu_usage: process.cpu_usage(),
            memory_bytes: process.memory(),
            disk_read_bytes: process.disk_usage().read_bytes,
            disk_written_bytes: process.disk_usage().written_bytes,
            status: process.status().to_string(),
            user_id: process.user_id().map(|u| u.to_string()),
            start_time: process.start_time(),
        })
        .collect()
}

pub fn get_process_by_pid(pid: u32) -> Option<ProcessMetrics> {
    let sys_rwlock = get_system();
    let pid_obj = sysinfo::Pid::from(pid as usize);

    {
        let mut sys = sys_rwlock.write().unwrap_or_else(|e| e.into_inner());
        sys.refresh_processes(ProcessesToUpdate::Some(&[pid_obj]), false);
    }

    let sys = sys_rwlock.read().unwrap_or_else(|e| e.into_inner());
    sys.process(pid_obj).map(|process| ProcessMetrics {
        pid,
        name: process.name().to_string_lossy().into_owned(),
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
        let mut sys = sys_rwlock.write().unwrap_or_else(|e| e.into_inner());
        sys.refresh_processes(ProcessesToUpdate::All, true);
    }

    let sys = sys_rwlock.read().unwrap_or_else(|e| e.into_inner());
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
        let mut networks = networks_rwlock.write().unwrap_or_else(|e| e.into_inner());
        networks.refresh();
    }

    let networks = networks_rwlock.read().unwrap_or_else(|e| e.into_inner());
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
        batteries: get_batteries(),
        components: get_components(),
        gpus: crate::platform::gpus::get_gpus(),
        network_connections: crate::platform::network_connections::get_network_connections(),
        cpu_core_temperatures: get_cpu_core_temperatures(),
    }
}

pub fn cleanup_metrics() {
    // Note: OnceLock cannot be "cleared" currently in stable Rust.
    // This function can be expanded if we switch to a different lazy init strategy
    // or when OnceLock::take() becomes stable (though it's not clear it will).
    // For now, this is a placeholder to satisfy the FFI requirement.
}

/// Returns CPU core temperatures (per-core temperature readings)
pub fn get_cpu_core_temperatures() -> Vec<CpuCoreTemperature> {
    let components_rwlock = get_components_obj();
    {
        let mut components = components_rwlock.write().unwrap_or_else(|e| e.into_inner());
        components.refresh_list();
        components.refresh();
    }

    let components = components_rwlock.read().unwrap_or_else(|e| e.into_inner());
    let mut core_temps = Vec::new();

    // Filter components that are CPU cores (typically labeled as "Core 0", "Core 1", etc.)
    for (index, component) in components.iter().enumerate() {
        let label = component.label();

        // Check if this is a CPU core temperature sensor
        if label.to_lowercase().contains("core")
            || label.to_lowercase().contains("cpu")
            || label.to_lowercase().contains("package")
        {
            core_temps.push(CpuCoreTemperature {
                core_index: index as u32,
                temperature_celsius: component.temperature(),
                max_temperature_celsius: Some(component.max()),
                critical_temperature_celsius: component.critical(),
            });
        }
    }

    // If no specific core temps found, return all temperature components
    if core_temps.is_empty() {
        for (index, component) in components.iter().enumerate() {
            core_temps.push(CpuCoreTemperature {
                core_index: index as u32,
                temperature_celsius: component.temperature(),
                max_temperature_celsius: Some(component.max()),
                critical_temperature_celsius: component.critical(),
            });
        }
    }

    core_temps
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
        println!(
            "Load Average: 1m={}, 5m={}, 15m={}",
            load.one_min, load.five_min, load.fifteen_min
        );
    }

    #[test]
    fn test_all_metrics() {
        let all = get_all_metrics();
        assert!(!all.cpu.is_empty());
        println!(
            "All metrics: CPU count={}, Memory used={}",
            all.cpu.len(),
            all.memory.used_bytes
        );
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
    fn test_extended_processes() {
        let processes = get_extended_processes();
        assert!(!processes.is_empty());
        println!("Found {} processes", processes.len());
        if let Some(p) = processes.first() {
            println!(
                "Extended process: {} (PID: {}, Parent: {:?})",
                p.name, p.pid, p.parent_pid
            );
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
        let networks = networks_rwlock.read().unwrap_or_else(|e| e.into_inner());
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
        for b in &batteries {
            println!(
                "Battery: {} {}, State: {}, Energy: {}%",
                b.vendor.as_deref().unwrap_or("Unknown"),
                b.model.as_deref().unwrap_or(""),
                b.state,
                b.energy_pct
            );
        }
    }

    #[test]
    fn test_refresh_metrics() {
        refresh_metrics(1); // Refresh CPU only
        let cpus = get_cpus();
        assert!(!cpus.is_empty());
        println!("CPU refresh test passed with {} cores", cpus.len());
    }

    #[test]
    fn test_cpu_frequency() {
        let cpus = get_cpus();
        assert!(!cpus.is_empty(), "Should have at least one CPU");

        for (index, cpu) in cpus.iter().enumerate() {
            println!("CPU {}: {} @ {} MHz", index, cpu.brand, cpu.frequency);

            // Frequency can be 0 on some systems/VMs, but we should log a warning
            if cpu.frequency == 0 {
                println!("WARNING: CPU frequency is 0 for core {}. This may be expected on VMs or certain hardware.", index);
            } else {
                // Verify frequency is in reasonable range (100 MHz to 10 GHz)
                assert!(
                    cpu.frequency >= 100 && cpu.frequency <= 10000,
                    "CPU frequency {} MHz seems unreasonable for core {}",
                    cpu.frequency,
                    index
                );
            }
        }
    }

    #[test]
    fn test_cpu_core_temperatures() {
        let temps = get_cpu_core_temperatures();
        println!("Found {} temperature sensors", temps.len());
        for temp in &temps {
            println!(
                "Core {}: {}°C (Max: {:?}°C, Critical: {:?}°C)",
                temp.core_index,
                temp.temperature_celsius,
                temp.max_temperature_celsius,
                temp.critical_temperature_celsius
            );
        }
    }
}
