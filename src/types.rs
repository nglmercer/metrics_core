use serde::Serialize;

/// Library version info
#[derive(Serialize, Clone)]
pub struct LibraryVersion {
    pub version: String,
    pub name: String,
}

#[derive(Serialize)]
pub struct LoadAverage {
    pub one_min: f64,
    pub five_min: f64,
    pub fifteen_min: f64,
}

#[derive(Serialize)]
pub struct CpuMetrics {
    pub usage_pct: f32,
    pub brand: String,
    pub frequency: u64,
}

#[derive(Serialize)]
pub struct MemoryMetrics {
    pub total_bytes: u64,
    pub free_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub swap_total_bytes: u64,
    pub swap_used_bytes: u64,
}

#[derive(Serialize)]
pub struct DiskMetrics {
    pub name: String,
    pub total_space: u64,
    pub available_space: u64,
    pub used_space: u64,
    pub mount_point: String,
    pub file_system: String,
}

#[derive(Serialize)]
pub struct NetworkMetrics {
    pub interface: String,
    pub received_bytes: u64,
    pub transmitted_bytes: u64,
    pub packets_received: u64,
    pub packets_transmitted: u64,
}

#[derive(Serialize, Clone)]
pub struct OsInfo {
    pub name: String,
    pub kernel_version: String,
    pub os_version: String,
    pub host_name: String,
}

#[derive(Serialize)]
pub struct ComponentMetrics {
    pub label: String,
    pub temperature: f32,
    pub max: f32,
    pub critical: Option<f32>,
}

#[derive(Serialize)]
pub struct ProcessMetrics {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_bytes: u64,
    pub disk_read_bytes: u64,
    pub disk_written_bytes: u64,
    pub status: String,
    pub user_id: Option<String>,
}

#[derive(Serialize)]
pub struct ExtendedProcessMetrics {
    pub pid: u32,
    pub parent_pid: Option<u32>,
    pub name: String,
    pub command: Option<String>,
    pub cpu_usage: f32,
    pub memory_bytes: u64,
    pub disk_read_bytes: u64,
    pub disk_written_bytes: u64,
    pub status: String,
    pub user_id: Option<String>,
    pub start_time: u64,
}

#[derive(Serialize)]
pub struct DiskIoMetrics {
    pub read_bytes: u64,
    pub written_bytes: u64,
}

#[derive(Serialize)]
pub struct NetworkIoMetrics {
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_packets: u64,
    pub tx_packets: u64,
}

#[derive(Serialize)]
pub struct BatteryInfo {
    pub state: String,
    pub vendor: Option<String>,
    pub model: Option<String>,
    pub cycle_count: Option<u32>,
    pub health_pct: f32,
    pub energy_pct: f32,
    pub energy_full_design_wh: f32,
    pub energy_full_wh: f32,
    pub energy_wh: f32,
}

/// Complete system metrics snapshot
#[derive(Serialize)]
pub struct AllMetrics {
    pub cpu: Vec<CpuMetrics>,
    pub memory: MemoryMetrics,
    pub disks: Vec<DiskMetrics>,
    pub networks: Vec<NetworkMetrics>,
    pub uptime: u64,
    pub os_info: OsInfo,
    pub load_avg: LoadAverage,
    pub batteries: Vec<BatteryInfo>,
    pub components: Vec<ComponentMetrics>,
}

/// Refresh flags for selective metric updates
pub const REFRESH_CPU: u32 = 1;
pub const REFRESH_MEMORY: u32 = 2;
pub const REFRESH_DISKS: u32 = 4;
pub const REFRESH_NETWORKS: u32 = 8;
pub const REFRESH_PROCESSES: u32 = 16;
pub const REFRESH_COMPONENTS: u32 = 32;
pub const REFRESH_ALL: u32 = 0xFFFFFFFF;
