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
lib.get_all_metrics.restype = ctypes.c_char_p
lib.get_battery_info.restype = ctypes.c_char_p
lib.get_library_version.restype = ctypes.c_char_p
lib.free_metrics_string.argtypes = [ctypes.c_char_p]

# --- UI Helpers ---
def format_bytes(b):
    for unit in ['B', 'KiB', 'MiB', 'GiB', 'TiB']:
        if b < 1024:
            return f"{b:.2f} {unit}"
        b /= 1024
    return f"{b:.2f} PiB"

def clear():
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
    version_raw = lib.get_library_version()
    version = json.loads(version_raw.decode())['version']
    lib.free_metrics_string(version_raw)

    while True:
        clear()
        print(f"{Colors.CYAN}=== SYSTEM MONITOR PRO v{version} (PYTHON FFI) ==={Colors.RESET}")
        
        raw_ptr = lib.get_all_metrics()
        if not raw_ptr:
            print("Error: Could not fetch metrics")
            time.sleep(2)
            continue
            
        data = json.loads(raw_ptr.decode())
        lib.free_metrics_string(raw_ptr)

        uptime = data['uptime']
        hours, rem = divmod(uptime, 3600)
        minutes, seconds = divmod(rem, 60)
        
        # OS & Load
        os_info = data['os_info']
        load = data['load_avg']
        print(f"{Colors.DIM}{os_info['name']} {os_info['os_version']} | {os_info['host_name']}{Colors.RESET}")
        print(f"{Colors.YELLOW}Uptime: {hours}h {minutes}m {seconds}s | Load: {load['one_min']:.2f} {load['five_min']:.2f} {load['fifteen_min']:.2f}{Colors.RESET}\n")

        # CPU Statistics
        cpus = data['cpu']
        avg_cpu = sum(c['usage_pct'] for c in cpus) / len(cpus)
        bar_width = 30
        filled = int((avg_cpu / 100) * bar_width)
        bar = "█" * filled + "░" * (bar_width - filled)
        print(f"{Colors.BOLD}CPU Usage:{Colors.RESET} [{Colors.GREEN}{bar}{Colors.RESET}] {avg_cpu:.1f}%")
        print(f"{Colors.DIM}  {cpus[0]['brand']} ({len(cpus)} cores @ {cpus[0]['frequency']}MHz){Colors.RESET}\n")

        # Memory Usage
        mem = data['memory']
        used_pct = (mem['used_bytes'] / mem['total_bytes']) * 100
        filled = int((used_pct / 100) * bar_width)
        bar = "█" * filled + "░" * (bar_width - filled)
        print(f"{Colors.BOLD}RAM Usage:{Colors.RESET} [{Colors.YELLOW}{bar}{Colors.RESET}] {used_pct:.1f}%")
        print(f"  {format_bytes(mem['used_bytes'])} / {format_bytes(mem['total_bytes'])}\n")

        # Battery
        bat_raw = lib.get_battery_info()
        if bat_raw:
            batteries = json.loads(bat_raw.decode())
            lib.free_metrics_string(bat_raw)
            if batteries:
                print(f"{Colors.BOLD}Battery Status:{Colors.RESET}")
                for b in batteries:
                    print(f"  {b['state']}: {b['energy_pct']:.1f}% (Health: {b['health_pct']:.1f}%)")
                print()

        # Storage
        print(f"{Colors.BOLD}Storage Devices:{Colors.RESET}")
        seen_disks = set()
        for d in data['disks']:
            disk_id = f"{d['name']}_{d['total_space']}"
            if d['total_space'] == 0 or disk_id in seen_disks:
                continue
            seen_disks.add(disk_id)
            usage_pct = ((d['total_space'] - d['available_space']) / d['total_space']) * 100
            print(f"  {d['mount_point']:<12} {usage_pct:>5.1f}% used of {format_bytes(d['total_space'])} ({d['file_system']})")
        print()

        # Network Traffic
        print(f"{Colors.BOLD}Network Traffic:{Colors.RESET}")
        for n in data['networks']:
            if n['received_bytes'] == 0 and n['transmitted_bytes'] == 0:
                continue
            print(f"  {Colors.MAGENTA}{n['interface']:<10}{Colors.RESET} ↓ {format_bytes(n['received_bytes']):<10} ↑ {format_bytes(n['transmitted_bytes'])}")

        print(f"\n{Colors.DIM}Refreshing... (Ctrl+C to exit){Colors.RESET}")
        time.sleep(2)
        
except KeyboardInterrupt:
    print(f"\n{Colors.YELLOW}Exiting...{Colors.RESET}")
    sys.exit(0)
except Exception as e:
    print(f"\n{Colors.RED}Runtime Error: {e}{Colors.RESET}")
    sys.exit(1)
