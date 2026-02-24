#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub use linux::get_network_connections;

#[cfg(not(target_os = "linux"))]
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
                "Conn {}: {} {} -> {} (Stat: {}, PID: {:?})",
                i, conn.protocol, conn.local_address, conn.remote_address, conn.state, conn.pid
            );
        }
    }
}
