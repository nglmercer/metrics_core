#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "linux")]
pub use linux::get_network_connections;

#[cfg(target_os = "windows")]
pub use windows::get_network_connections;

#[cfg(target_os = "macos")]
pub use macos::get_network_connections;

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
pub fn get_network_connections() -> Vec<crate::types::NetworkConnection> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_connections() {
        let conns = get_network_connections();
        println!("Found {} network connections", conns.len());
        for (i, conn) in conns.iter().take(10).enumerate() {
            println!(
                "Conn {}: {} {} -> {} (Stat: {}, PID: {:?}, Name: {:?})",
                i,
                conn.protocol,
                conn.local_address,
                conn.remote_address,
                conn.state,
                conn.pid,
                conn.process_name
            );
        }
    }
}
