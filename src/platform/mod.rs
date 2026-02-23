pub mod linux;
pub mod macos;
pub mod windows;

use crate::types::*;

pub fn get_cpus() -> Vec<CpuMetrics> {
    #[cfg(target_os = "linux")]
    {
        linux::get_cpus()
    }
    #[cfg(target_os = "windows")]
    {
        windows::get_cpus()
    }
    #[cfg(target_os = "macos")]
    {
        macos::get_cpus()
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        panic!("Unsupported platform")
    }
}

pub fn get_memory() -> MemoryMetrics {
    #[cfg(target_os = "linux")]
    {
        linux::get_memory()
    }
    #[cfg(target_os = "windows")]
    {
        windows::get_memory()
    }
    #[cfg(target_os = "macos")]
    {
        macos::get_memory()
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        panic!("Unsupported platform")
    }
}

pub fn get_disks() -> Vec<DiskMetrics> {
    #[cfg(target_os = "linux")]
    {
        linux::get_disks()
    }
    #[cfg(target_os = "windows")]
    {
        windows::get_disks()
    }
    #[cfg(target_os = "macos")]
    {
        macos::get_disks()
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        panic!("Unsupported platform")
    }
}

pub fn get_networks() -> Vec<NetworkMetrics> {
    #[cfg(target_os = "linux")]
    {
        linux::get_networks()
    }
    #[cfg(target_os = "windows")]
    {
        windows::get_networks()
    }
    #[cfg(target_os = "macos")]
    {
        macos::get_networks()
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        panic!("Unsupported platform")
    }
}

pub fn get_uptime() -> u64 {
    #[cfg(target_os = "linux")]
    {
        linux::get_uptime()
    }
    #[cfg(target_os = "windows")]
    {
        windows::get_uptime()
    }
    #[cfg(target_os = "macos")]
    {
        macos::get_uptime()
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        panic!("Unsupported platform")
    }
}
