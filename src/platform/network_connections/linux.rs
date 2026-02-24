use crate::types::NetworkConnection;
use std::fs;

use std::collections::HashMap;

pub fn get_network_connections() -> Vec<NetworkConnection> {
    let mut connections = Vec::new();

    // Map of inode to (pid, process_name)
    let inode_map = get_inode_process_map();

    // Process TCP
    if let Ok(content) = fs::read_to_string("/proc/net/tcp") {
        connections.extend(parse_proc_net(&content, "TCP", &inode_map));
    }

    // Process TCP6
    if let Ok(content) = fs::read_to_string("/proc/net/tcp6") {
        connections.extend(parse_proc_net(&content, "TCP6", &inode_map));
    }

    // Process UDP
    if let Ok(content) = fs::read_to_string("/proc/net/udp") {
        connections.extend(parse_proc_net(&content, "UDP", &inode_map));
    }

    // Process UDP6
    if let Ok(content) = fs::read_to_string("/proc/net/udp6") {
        connections.extend(parse_proc_net(&content, "UDP6", &inode_map));
    }

    connections
}

fn get_inode_process_map() -> HashMap<u64, (u32, String)> {
    let mut map = HashMap::new();

    if let Ok(entries) = fs::read_dir("/proc") {
        for entry in entries.flatten() {
            let path = entry.path();
            let pid_str = match path.file_name().and_then(|n| n.to_str()) {
                Some(s) if s.chars().all(|c| c.is_ascii_digit()) => s,
                _ => continue,
            };

            let pid = match pid_str.parse::<u32>() {
                Ok(p) => p,
                Err(_) => continue,
            };

            let comm = fs::read_to_string(path.join("comm"))
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|_| "Unknown".to_string());

            let fd_path = path.join("fd");
            if let Ok(fds) = fs::read_dir(fd_path) {
                for fd in fds.flatten() {
                    if let Ok(link) = fs::read_link(fd.path()) {
                        let link_str = link.to_string_lossy();
                        if link_str.starts_with("socket:[") {
                            if let Some(inode_str) = link_str
                                .strip_prefix("socket:[")
                                .and_then(|s| s.strip_suffix("]"))
                            {
                                if let Ok(inode) = inode_str.parse::<u64>() {
                                    map.insert(inode, (pid, comm.clone()));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    map
}

fn parse_proc_net(
    content: &str,
    protocol: &str,
    inode_map: &HashMap<u64, (u32, String)>,
) -> Vec<NetworkConnection> {
    let mut connections = Vec::new();
    let lines = content.lines().skip(1); // Skip header

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 10 {
            continue;
        }

        let local_addr = parse_address(parts[1]);
        let remote_addr = parse_address(parts[2]);
        let state = parse_state(parts[3], protocol);
        let inode = parts[9].parse::<u64>().unwrap_or(0);

        let (pid, process_name) = inode_map
            .get(&inode)
            .map(|(p, n)| (Some(*p), Some(n.clone())))
            .unwrap_or((None, None));

        connections.push(NetworkConnection {
            protocol: protocol.to_string(),
            local_address: local_addr.0,
            local_port: local_addr.1,
            remote_address: remote_addr.0,
            remote_port: remote_addr.1,
            state,
            pid,
            process_name,
        });
    }

    connections
}

fn parse_address(hex: &str) -> (String, u16) {
    let parts: Vec<&str> = hex.split(':').collect();
    if parts.len() != 2 {
        return ("Unknown".to_string(), 0);
    }

    let addr_hex = parts[0];
    let port = u16::from_str_radix(parts[1], 16).unwrap_or(0);

    let address = if addr_hex.len() == 8 {
        // IPv4
        let val = u32::from_str_radix(addr_hex, 16).unwrap_or(0);
        format!(
            "{}.{}.{}.{}",
            val & 0xFF,
            (val >> 8) & 0xFF,
            (val >> 16) & 0xFF,
            (val >> 24) & 0xFF
        )
    } else if addr_hex.len() == 32 {
        // IPv6
        let mut addr = String::new();
        for i in 0..4 {
            let part_hex = &addr_hex[i * 8..(i + 1) * 8];
            let val = u32::from_str_radix(part_hex, 16).unwrap_or(0);
            let bytes = val.to_ne_bytes();
            for b in bytes.iter().rev() {
                addr.push_str(&format!("{:02x}", b));
                if addr.len() % 5 == 4 && addr.len() < 39 {
                    addr.push(':');
                }
            }
        }
        addr
    } else {
        "Unknown".to_string()
    };

    (address, port)
}

fn parse_state(hex: &str, protocol: &str) -> String {
    if protocol.starts_with("UDP") {
        return "N/A".to_string();
    }

    match hex {
        "01" => "ESTABLISHED",
        "02" => "SYN_SENT",
        "03" => "SYN_RECV",
        "04" => "FIN_WAIT1",
        "05" => "FIN_WAIT2",
        "06" => "TIME_WAIT",
        "07" => "CLOSE",
        "08" => "CLOSE_WAIT",
        "09" => "LAST_ACK",
        "0A" => "LISTEN",
        "0B" => "CLOSING",
        _ => "UNKNOWN",
    }
    .to_string()
}
