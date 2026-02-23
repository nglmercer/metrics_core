use crate::types::{CpuMetrics, DiskMetrics, MemoryMetrics, NetworkMetrics};
use std::sync::Mutex;
use std::sync::OnceLock;
use sysinfo::{CpuRefreshKind, Disks, MemoryRefreshKind, Networks, System};

static SYSTEM: OnceLock<Mutex<System>> = OnceLock::new();
static DISKS: OnceLock<Mutex<Disks>> = OnceLock::new();
static NETWORKS: OnceLock<Mutex<Networks>> = OnceLock::new();

fn get_system() -> &'static Mutex<System> {
    SYSTEM.get_or_init(|| {
        let mut sys = System::new();
        sys.refresh_all();
        Mutex::new(sys)
    })
}

fn get_disks_obj() -> &'static Mutex<Disks> {
    DISKS.get_or_init(|| Mutex::new(Disks::new_with_refreshed_list()))
}

fn get_networks_obj() -> &'static Mutex<Networks> {
    NETWORKS.get_or_init(|| Mutex::new(Networks::new_with_refreshed_list()))
}

pub fn get_cpus() -> Vec<CpuMetrics> {
    let sys_mutex = get_system();
    let mut sys = sys_mutex.lock().unwrap();
    sys.refresh_cpu_specifics(CpuRefreshKind::new().with_cpu_usage());

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
    let sys_mutex = get_system();
    let mut sys = sys_mutex.lock().unwrap();
    sys.refresh_memory_specifics(MemoryRefreshKind::new().with_ram());

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
    let disks_mutex = get_disks_obj();
    let mut disks = disks_mutex.lock().unwrap();
    disks.refresh_list();

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
    let networks_mutex = get_networks_obj();
    let mut networks = networks_mutex.lock().unwrap();
    networks.refresh();

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
