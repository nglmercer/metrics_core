use crate::types::NetworkConnection;
use std::process::Command;

pub fn get_network_connections() -> Vec<NetworkConnection> {
    let mut connections = Vec::new();

    // On macOS, we can parse netstat -n -v (which includes PIDs in newer macOS versions)
    // or just netstat -n.
    // For a robust implementation without private APIs, netstat is a good fallback.

    if let Ok(output) = Command::new("netstat").args(["-n", "-p", "tcp"]).output() {
        connections.extend(parse_netstat(
            &String::from_utf8_lossy(&output.stdout),
            "TCP",
        ));
    }

    if let Ok(output) = Command::new("netstat").args(["-n", "-p", "udp"]).output() {
        connections.extend(parse_netstat(
            &String::from_utf8_lossy(&output.stdout),
            "UDP",
        ));
    }

    connections
}

fn parse_netstat(output: &str, protocol: &str) -> Vec<NetworkConnection> {
    let mut connections = Vec::new();
    let lines = output.lines().skip(2); // Skip header lines

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 4 {
            continue;
        }

        // Format is usually: Proto Recv-Q Send-Q Local Address Foreign Address State
        // But it varies between versions.
        // This is a simplified parser.

        let local = parts[3];
        let remote = parts[4];
        let state = if parts.len() > 5 { parts[5] } else { "N/A" };

        let (l_addr, l_port) = split_addr_port(local);
        let (r_addr, r_port) = split_addr_port(remote);

        connections.push(NetworkConnection {
            protocol: protocol.to_string(),
            local_address: l_addr,
            local_port: l_port,
            remote_address: r_addr,
            remote_port: r_port,
            state: state.to_string(),
            pid: None, // netstat doesn't always give PID without root/specific flags
            process_name: None,
        });
    }

    connections
}

fn split_addr_port(s: &str) -> (String, u16) {
    if let Some(pos) = s.rfind('.') {
        let addr = &s[..pos];
        let port = s[pos + 1..].parse::<u16>().unwrap_or(0);
        return (addr.to_string(), port);
    }
    (s.to_string(), 0)
}
