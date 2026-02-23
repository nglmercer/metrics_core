pub mod platform;
pub mod types;

use libc::c_char;
use std::ffi::CString;

/// Returns a JSON string containing the VPS metrics.
/// The caller is responsible for freeing the memory using `free_metrics_string`.
#[no_mangle]
pub extern "C" fn get_vps_metrics() -> *mut c_char {
    let all_metrics = platform::get_metrics();

    match serde_json::to_string(&all_metrics) {
        Ok(json) => match CString::new(json) {
            Ok(c_str) => c_str.into_raw(),
            Err(_) => std::ptr::null_mut(),
        },
        Err(_) => std::ptr::null_mut(),
    }
}

/// Frees a string allocated by `get_vps_metrics`.
#[no_mangle]
pub extern "C" fn free_metrics_string(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    unsafe {
        let _ = CString::from_raw(s);
    }
}
