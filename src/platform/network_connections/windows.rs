use crate::types::NetworkConnection;
use std::ptr;
use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::NetworkManagement::IpHelper::*;
use windows_sys::Win32::Networking::WinSock::*;
use windows_sys::Win32::System::Threading::*;

pub fn get_network_connections() -> Vec<NetworkConnection> {
    let mut connections = Vec::new();

    connections.extend(get_tcp_connections());
    connections.extend(get_udp_connections());

    connections
}

fn get_tcp_connections() -> Vec<NetworkConnection> {
    let mut connections = Vec::new();
    let mut size = 0;

    // First call to get required size for IPv4
    unsafe {
        GetExtendedTcpTable(
            ptr::null_mut(),
            &mut size,
            FALSE,
            AF_INET as u32,
            TCP_TABLE_OWNER_PID_ALL,
            0,
        );
    }

    let mut buffer = vec![0u8; size as usize];
    if unsafe {
        GetExtendedTcpTable(
            buffer.as_mut_ptr() as *mut _,
            &mut size,
            FALSE,
            AF_INET as u32,
            TCP_TABLE_OWNER_PID_ALL,
            0,
        )
    } == NO_ERROR
    {
        let table = unsafe { &*(buffer.as_ptr() as *const MIB_TCPTABLE_OWNER_PID) };
        for i in 0..table.dwNumEntries {
            let row = unsafe { *table.table.as_ptr().add(i as usize) };
            connections.push(NetworkConnection {
                protocol: "TCP".to_string(),
                local_address: format_ipv4(row.dwLocalAddr),
                local_port: u16::from_be(row.dwLocalPort as u16),
                remote_address: format_ipv4(row.dwRemoteAddr),
                remote_port: u16::from_be(row.dwRemotePort as u16),
                state: format_tcp_state(row.dwState),
                pid: Some(row.dwOwningPid),
                process_name: get_process_name(row.dwOwningPid),
            });
        }
    }

    // IPv6 TCP
    size = 0;
    unsafe {
        GetExtendedTcpTable(
            ptr::null_mut(),
            &mut size,
            FALSE,
            AF_INET6 as u32,
            TCP_TABLE_OWNER_PID_ALL,
            0,
        );
    }

    if size > 0 {
        let mut buffer = vec![0u8; size as usize];
        if unsafe {
            GetExtendedTcpTable(
                buffer.as_mut_ptr() as *mut _,
                &mut size,
                FALSE,
                AF_INET6 as u32,
                TCP_TABLE_OWNER_PID_ALL,
                0,
            )
        } == NO_ERROR
        {
            let table = unsafe { &*(buffer.as_ptr() as *const MIB_TCP6TABLE_OWNER_PID) };
            for i in 0..table.dwNumEntries {
                let row = unsafe { *table.table.as_ptr().add(i as usize) };
                connections.push(NetworkConnection {
                    protocol: "TCP6".to_string(),
                    local_address: format_ipv6(row.ucLocalAddr),
                    local_port: u16::from_be(row.dwLocalPort as u16),
                    remote_address: format_ipv6(row.ucRemoteAddr),
                    remote_port: u16::from_be(row.dwRemotePort as u16),
                    state: format_tcp_state(row.dwState),
                    pid: Some(row.dwOwningPid),
                    process_name: get_process_name(row.dwOwningPid),
                });
            }
        }
    }

    connections
}

fn get_udp_connections() -> Vec<NetworkConnection> {
    let mut connections = Vec::new();
    let mut size = 0;

    // IPv4 UDP
    unsafe {
        GetExtendedUdpTable(
            ptr::null_mut(),
            &mut size,
            FALSE,
            AF_INET as u32,
            UDP_TABLE_OWNER_PID,
            0,
        );
    }

    let mut buffer = vec![0u8; size as usize];
    if unsafe {
        GetExtendedUdpTable(
            buffer.as_mut_ptr() as *mut _,
            &mut size,
            FALSE,
            AF_INET as u32,
            UDP_TABLE_OWNER_PID,
            0,
        )
    } == NO_ERROR
    {
        let table = unsafe { &*(buffer.as_ptr() as *const MIB_UDPTABLE_OWNER_PID) };
        for i in 0..table.dwNumEntries {
            let row = unsafe { *table.table.as_ptr().add(i as usize) };
            connections.push(NetworkConnection {
                protocol: "UDP".to_string(),
                local_address: format_ipv4(row.dwLocalAddr),
                local_port: u16::from_be(row.dwLocalPort as u16),
                remote_address: "0.0.0.0".to_string(),
                remote_port: 0,
                state: "N/A".to_string(),
                pid: Some(row.dwOwningPid),
                process_name: get_process_name(row.dwOwningPid),
            });
        }
    }

    // IPv6 UDP
    size = 0;
    unsafe {
        GetExtendedUdpTable(
            ptr::null_mut(),
            &mut size,
            FALSE,
            AF_INET6 as u32,
            UDP_TABLE_OWNER_PID,
            0,
        );
    }

    if size > 0 {
        let mut buffer = vec![0u8; size as usize];
        if unsafe {
            GetExtendedUdpTable(
                buffer.as_mut_ptr() as *mut _,
                &mut size,
                FALSE,
                AF_INET6 as u32,
                UDP_TABLE_OWNER_PID,
                0,
            )
        } == NO_ERROR
        {
            let table = unsafe { &*(buffer.as_ptr() as *const MIB_UDP6TABLE_OWNER_PID) };
            for i in 0..table.dwNumEntries {
                let row = unsafe { *table.table.as_ptr().add(i as usize) };
                connections.push(NetworkConnection {
                    protocol: "UDP6".to_string(),
                    local_address: format_ipv6(row.ucLocalAddr),
                    local_port: u16::from_be(row.dwLocalPort as u16),
                    remote_address: "::".to_string(),
                    remote_port: 0,
                    state: "N/A".to_string(),
                    pid: Some(row.dwOwningPid),
                    process_name: get_process_name(row.dwOwningPid),
                });
            }
        }
    }

    connections
}

fn format_ipv4(addr: u32) -> String {
    let bytes = addr.to_ne_bytes();
    format!("{}.{}.{}.{}", bytes[0], bytes[1], bytes[2], bytes[3])
}

fn format_ipv6(addr: [u8; 16]) -> String {
    let mut s = String::new();
    for i in 0..8 {
        let val = u16::from_be_bytes([addr[i * 2], addr[i * 2 + 1]]);
        s.push_str(&format!("{:x}", val));
        if i < 7 {
            s.push(':');
        }
    }
    s
}

fn format_tcp_state(state: u32) -> String {
    match state {
        1 => "CLOSED",
        2 => "LISTEN",
        3 => "SYN_SENT",
        4 => "SYN_RCVD",
        5 => "ESTABLISHED",
        6 => "FIN_WAIT1",
        7 => "FIN_WAIT2",
        8 => "CLOSE_WAIT",
        9 => "CLOSING",
        10 => "LAST_ACK",
        11 => "TIME_WAIT",
        12 => "DELETE_TCB",
        _ => "UNKNOWN",
    }
    .to_string()
}

fn get_process_name(pid: u32) -> Option<String> {
    if pid == 0 {
        return Some("System Idle Process".to_string());
    }
    if pid == 4 {
        return Some("System".to_string());
    }

    unsafe {
        let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, FALSE, pid);
        if handle != 0 {
            let mut buffer = [0u16; 1024];
            let mut size = buffer.len() as u32;
            if QueryFullProcessImageNameW(handle, 0, buffer.as_mut_ptr(), &mut size) != 0 {
                let path = String::from_utf16_lossy(&buffer[..size as usize]);
                CloseHandle(handle);
                return std::path::Path::new(&path)
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string());
            }
            CloseHandle(handle);
        }
    }
    None
}
