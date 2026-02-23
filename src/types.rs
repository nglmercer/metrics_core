use serde::Serialize;

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
}

#[derive(Serialize)]
pub struct DiskMetrics {
    pub name: String,
    pub total_space: u64,
    pub available_space: u64,
    pub mount_point: String,
}

#[derive(Serialize)]
pub struct NetworkMetrics {
    pub interface: String,
    pub received_bytes: u64,
    pub transmitted_bytes: u64,
}
