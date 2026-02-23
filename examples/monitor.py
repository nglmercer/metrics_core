import ctypes
import json
import os
import time
import sys

# --- Library Configuration ---
lib_name = "libmetrics.so"
if sys.platform == "win32":
    lib_name = "metrics.dll"
elif sys.platform == "darwin":
    lib_name = "libmetrics.dylib"

lib_path = os.path.abspath(os.path.join("target", "release", lib_name))

try:
    lib = ctypes.CDLL(lib_path)
except OSError:
    print(f"\033[91mError: Library not found at {lib_path}\033[0m")
    print("Please run 'cargo build --release' first.")
    sys.exit(1)

# Define return types for FFI functions
lib.get_cpu_metrics.restype = ctypes.c_char_p
lib.get_memory_metrics.restype = ctypes.c_char_p
lib.get_disk_metrics.restype = ctypes.c_char_p
lib.get_network_metrics.restype = ctypes.c_char_p
lib.get_os_info.restype = ctypes.c_char_p
lib.get_cpu_components.restype = ctypes.c_char_p
lib.get_uptime.restype = ctypes.c_uint64

# --- UI Helpers ---
def format_bytes(b):
    """Formats bytes into human-readable binary prefixes (KiB, GiB, etc.)"""
    for unit in ['B', 'KiB', 'MiB', 'GiB', 'TiB']:
        if b < 1024:
            return f"{b:.2f} {unit}"
        b /= 1024
    return f"{b:.2f} PiB"

def clear():
    """Clears the terminal screen"""
    os.system('cls' if os.name == 'nt' else 'clear')

class Colors:
    CYAN = "\033[1;36m"
    YELLOW = "\033[93m"
    GREEN = "\033[92m"
    MAGENTA = "\033[95m"
    RED = "\033[91m"
    DIM = "\033[2m"
    BOLD = "\033[1m"
    RESET = "\033[0m"

# --- Main Application Loop ---
try:
    # Initial call to initialize CPU history in the Rust library
    lib.get_cpu_metrics()
    
    while True:
        clear()
        print(f"{Colors.CYAN}=== SYSTEM MONITOR PRO (PYTHON FFI) ==={Colors.RESET}")
        
        uptime = lib.get_uptime()
        hours, rem = divmod(uptime, 3600)
        minutes, seconds = divmod(rem, 60)
        
        # OS info
        os_raw = lib.get_os_info()
        if os_raw:
            os_data = json.loads(os_raw.decode())
            print(f"{Colors.DIM}{os_data['name']} {os_data['os_version']} | {os_data['host_name']} ({os_data['kernel_version']}){Colors.RESET}")

        print(f"{Colors.YELLOW}System Uptime: {hours}h {minutes}m {seconds}s{Colors.RESET}\n")

        # CPU Statistics
        cpu_raw = lib.get_cpu_metrics()
        if cpu_raw:
            cpu_data = json.loads(cpu_raw.decode())
            avg_cpu = sum(c['usage_pct'] for c in cpu_data) / len(cpu_data)
            bar_width = 30
            filled = int((avg_cpu / 100) * bar_width)
            bar = "█" * filled + "░" * (bar_width - filled)
            print(f"{Colors.BOLD}CPU Usage:{Colors.RESET} [{Colors.GREEN}{bar}{Colors.RESET}] {avg_cpu:.1f}%")
            
            # CPU Temperature
            sensors_raw = lib.get_cpu_components()
            if sensors_raw:
                sensors = json.loads(sensors_raw.decode())
                # Define common CPU-related temperature labels
                cpu_labels = ["cpu", "package", "k10temp", "coretemp", "tctl", "tdie"]
                temps = [s for s in sensors if any(label in s['label'].lower() for label in cpu_labels)]
                if temps:
                    temp_str = ", ".join([f"{t['temperature']}°C" for t in temps])
                    print(f"{Colors.DIM}  Temp: {temp_str}{Colors.RESET}")
            
            print(f"{Colors.DIM}  {cpu_data[0]['brand']} ({len(cpu_data)} cores @ {cpu_data[0]['frequency']}MHz){Colors.RESET}\n")

        # Memory Usage
        mem_raw = lib.get_memory_metrics()
        if mem_raw:
            mem = json.loads(mem_raw.decode())
            used_pct = (mem['used_bytes'] / mem['total_bytes']) * 100
            bar_width = 30
            filled = int((used_pct / 100) * bar_width)
            bar = "█" * filled + "░" * (bar_width - filled)
            print(f"{Colors.BOLD}RAM Usage:{Colors.RESET} [{Colors.YELLOW}{bar}{Colors.RESET}] {used_pct:.1f}%")
            print(f"  {format_bytes(mem['used_bytes'])} / {format_bytes(mem['total_bytes'])}\n")

        # Storage Devices (Deduplicated)
        disk_raw = lib.get_disk_metrics()
        if disk_raw:
            print(f"{Colors.BOLD}Storage Devices:{Colors.RESET}")
            disks = json.loads(disk_raw.decode())
            seen_disks = set()
            for d in disks:
                # Deduplicate by disk name and size to handle subvolumes/mounts
                disk_id = f"{d['name']}_{d['total_space']}"
                if d['total_space'] == 0 or disk_id in seen_disks:
                    continue
                seen_disks.add(disk_id)
                usage_pct = ((d['total_space'] - d['available_space']) / d['total_space']) * 100
                print(f"  {d['mount_point']:<12} {usage_pct:>5.1f}% used of {format_bytes(d['total_space'])}")
            print()

        # Network Traffic
        net_raw = lib.get_network_metrics()
        if net_raw:
            print(f"{Colors.BOLD}Network Traffic:{Colors.RESET}")
            networks = json.loads(net_raw.decode())
            for n in networks:
                if n['received_bytes'] == 0 and n['transmitted_bytes'] == 0:
                    continue
                print(f"  {Colors.MAGENTA}{n['interface']:<10}{Colors.RESET} "
                      f"RX: {format_bytes(n['received_bytes']):<10} "
                      f"TX: {format_bytes(n['transmitted_bytes'])}")

        print(f"\n{Colors.DIM}Refreshing every 2s... (Press Ctrl+C to exit){Colors.RESET}")
        time.sleep(2)
        
except KeyboardInterrupt:
    print(f"\n{Colors.YELLOW}Exiting Monitor...{Colors.RESET}")
    sys.exit(0)
except Exception as e:
    print(f"\n{Colors.RED}Runtime Error: {e}{Colors.RESET}")
    sys.exit(1)
