pub mod linux;
pub mod macos;
pub mod windows;

use crate::types::AllMetrics;

pub fn get_metrics() -> AllMetrics {
    #[cfg(target_os = "linux")]
    {
        linux::get_metrics()
    }
    #[cfg(target_os = "windows")]
    {
        windows::get_metrics()
    }
    #[cfg(target_os = "macos")]
    {
        macos::get_metrics()
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        // Fallback or compile error
        panic!("Unsupported platform");
    }
}
