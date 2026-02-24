pub mod platform;
pub mod types;
pub mod ffi;

use libc::c_char;
use serde::Serialize;
use std::ffi::CString;

fn to_c_string<T: Serialize>(data: &T) -> *mut c_char {
    match serde_json::to_string(data) {
        Ok(json) => match CString::new(json) {
            Ok(c_str) => c_str.into_raw(),
            Err(e) => {
                let err_json = format!(r#"{{"error": "CString conversion failed: {}"}}"#, e);
                CString::new(err_json)
                    .unwrap_or_else(|_| {
                        CString::new(r#"{"error": "Fatal CString error"}"#).unwrap()
                    })
                    .into_raw()
            }
        },
        Err(e) => {
            let err_json = format!(r#"{{"error": "JSON serialization failed: {}"}}"#, e);
            CString::new(err_json)
                .unwrap_or_else(|_| CString::new(r#"{"error": "Fatal JSON error"}"#).unwrap())
                .into_raw()
        }
    }
}

/// Returns a JSON string containing the library version.
#[no_mangle]
pub extern "C" fn get_library_version() -> *mut c_char {
    to_c_string(&types::LibraryVersion {
        version: env!("CARGO_PKG_VERSION").to_string(),
        name: "METRICS".to_string(),
    })
}

/// Returns a JSON string containing CPU metrics.
#[no_mangle]
pub extern "C" fn get_cpu_metrics() -> *mut c_char {
    to_c_string(&platform::get_cpus())
}

/// Returns a JSON string containing Memory metrics.
#[no_mangle]
pub extern "C" fn get_memory_metrics() -> *mut c_char {
    to_c_string(&platform::get_memory())
}

/// Returns a JSON string containing Disk metrics.
#[no_mangle]
pub extern "C" fn get_disk_metrics() -> *mut c_char {
    to_c_string(&platform::get_disks())
}

/// Returns a JSON string containing Network metrics.
#[no_mangle]
pub extern "C" fn get_network_metrics() -> *mut c_char {
    to_c_string(&platform::get_networks())
}

/// Returns the system uptime in seconds.
#[no_mangle]
pub extern "C" fn get_uptime() -> u64 {
    platform::get_uptime()
}

/// Returns a JSON string containing system load averages.
#[no_mangle]
pub extern "C" fn get_load_average() -> *mut c_char {
    to_c_string(&platform::get_load_average())
}

/// Returns a JSON string containing OS information.
#[no_mangle]
pub extern "C" fn get_os_info() -> *mut c_char {
    to_c_string(&platform::get_os_info())
}

/// Returns a JSON string containing CPU components (temperature sensors).
#[no_mangle]
pub extern "C" fn get_cpu_components() -> *mut c_char {
    to_c_string(&platform::get_components())
}

/// Returns a JSON string containing the process list.
#[no_mangle]
pub extern "C" fn get_processes() -> *mut c_char {
    to_c_string(&platform::get_processes())
}

/// Returns a JSON string containing extended process metrics with parent PID and command line.
#[no_mangle]
pub extern "C" fn get_extended_processes() -> *mut c_char {
    to_c_string(&platform::get_extended_processes())
}

/// Returns a JSON string containing information for a specific PID.
/// Returns null if not found.
#[no_mangle]
pub extern "C" fn get_process_by_pid(pid: u32) -> *mut c_char {
    match platform::get_process_by_pid(pid) {
        Some(p) => to_c_string(&p),
        None => std::ptr::null_mut(),
    }
}

/// Returns a JSON string containing global Disk I/O metrics.
#[no_mangle]
pub extern "C" fn get_disk_io() -> *mut c_char {
    to_c_string(&platform::get_disk_io())
}

/// Returns a JSON string containing global Network I/O metrics.
#[no_mangle]
pub extern "C" fn get_network_io() -> *mut c_char {
    to_c_string(&platform::get_network_io())
}

/// Returns a JSON string containing Battery information.
#[no_mangle]
pub extern "C" fn get_battery_info() -> *mut c_char {
    to_c_string(&platform::get_batteries())
}

/// Returns a JSON string containing all system metrics at once.
#[no_mangle]
pub extern "C" fn get_all_metrics() -> *mut c_char {
    to_c_string(&platform::get_all_metrics())
}

/// Returns a JSON string containing GPU metrics (NVIDIA).
#[no_mangle]
pub extern "C" fn get_gpu_metrics() -> *mut c_char {
    to_c_string(&platform::get_gpus())
}

/// Returns a JSON string containing network connections.
#[no_mangle]
pub extern "C" fn get_network_connections() -> *mut c_char {
    to_c_string(&platform::get_network_connections())
}

/// Returns a JSON string containing CPU core temperatures.
#[no_mangle]
pub extern "C" fn get_cpu_core_temperatures() -> *mut c_char {
    to_c_string(&platform::get_cpu_core_temperatures())
}

/// Refreshes internal metric caches based on the provided flags.
///
/// Flags:
/// - 1 (REFRESH_CPU): Refresh CPU metrics
/// - 2 (REFRESH_MEMORY): Refresh memory metrics
/// - 4 (REFRESH_DISKS): Refresh disk metrics
/// - 8 (REFRESH_NETWORKS): Refresh network metrics
/// - 16 (REFRESH_PROCESSES): Refresh process list
/// - 32 (REFRESH_COMPONENTS): Refresh component (temperature) data
/// - 0xFFFFFFFF (REFRESH_ALL): Refresh all metrics
#[no_mangle]
pub extern "C" fn refresh_metrics(flags: u32) {
    platform::refresh_metrics(flags)
}

/// Cleans up internal resources.
#[no_mangle]
pub extern "C" fn cleanup_metrics() {
    platform::cleanup_metrics()
}

/// Frees a string allocated by any of the metrics functions.
///
/// # Safety
/// The pointer must have been returned by one of the metrics functions in this library
/// and must not have been freed yet.
#[no_mangle]
pub unsafe extern "C" fn free_metrics_string(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    let _ = CString::from_raw(s);
}
