# METRICS — System Monitoring FFI Library

A high-performance, cross-platform system metrics extraction library written in Rust with native C-ABI bindings. Designed to be embedded in any language that supports FFI calls — Python, Node.js, Bun, Go, C/C++, PHP, Ruby, and more.

---

## Why This Library?

Most monitoring solutions are either too high-level, require language-specific agents, or lack cross-platform consistency. **METRICS** provides a single `.so`/`.dll`/`.dylib` binary that communicates directly via the C ABI. It maintains its own internal state to provide accurate, real-time deltas for CPU usage and Network traffic.

---

## Supported Platforms

| Platform | Status  | Binary             |
| -------- | ------- | ------------------ |
| Linux    | ✅ Full | `libmetrics.so`    |
| Windows  | ✅ Full | `metrics.dll`       |
| macOS    | ✅ Full | `libmetrics.dylib` |

---

## Exposed FFI Functions

All functions use the C ABI (`extern "C"`).

### `get_library_version`

Returns a **JSON object** containing the library version and name.

### `get_cpu_metrics`

Returns a **JSON array** of CPU metrics per core.

### `get_memory_metrics`

Returns a **JSON object** of system memory metrics.

### `get_disk_metrics`

Returns a **JSON array** of disk/partition metrics.

### `get_network_metrics`

Returns a **JSON array** of network interface statistics.

### `get_uptime`

Returns the system uptime in seconds as a `u64`.

### `get_os_info`

Returns a **JSON object** containing OS information (name, kernel version, OS version, hostname).

### `get_cpu_components`

Returns a **JSON array** of CPU component metrics (temperature sensors).

### `get_processes`

Returns a **JSON array** of all running processes with their metrics.

### `get_extended_processes`

Returns a **JSON array** of all running processes with extended metrics (includes parent PID, command line, and start time).

### `get_process_by_pid(pid: u32)`

Returns a **JSON object** for a specific process by PID, or `null` if not found.

### `get_disk_io`

Returns a **JSON object** containing global disk I/O metrics (aggregated from all processes).

### `get_network_io`

Returns a **JSON object** containing global network I/O metrics.

### `get_battery_info`

Returns a **JSON array** of battery information (status, health, charge).

### `get_load_average`

Returns a **JSON object** containing system load averages (1, 5, and 15 minute averages).

### `get_all_metrics`

Returns a **JSON object** containing all system metrics at once (CPU, Memory, Disks, Networks, Uptime, OS Info, Load Average, Batteries, Components).

### `refresh_metrics(flags: u32)`

Refreshes internal metric caches based on the provided flags. Use this for better performance when you need to call multiple metric functions.

**Flags:**
- `1` (REFRESH_CPU): Refresh CPU metrics
- `2` (REFRESH_MEMORY): Refresh memory metrics
- `4` (REFRESH_DISKS): Refresh disk metrics
- `8` (REFRESH_NETWORKS): Refresh network metrics
- `16` (REFRESH_PROCESSES): Refresh process list
- `32` (REFRESH_COMPONENTS): Refresh component (temperature) data
- `0xFFFFFFFF` (REFRESH_ALL): Refresh all metrics

### `cleanup_metrics`

Cleans up internal resources. Call this when done using the library.

### `free_metrics_string`

Frees the memory allocated by the library for any JSON string returned. **Must be called after processing the JSON to avoid memory leaks.**

---

## JSON Schemas (v0.1.0)

### Library Version

```json
{
  "version": "0.1.0",
  "name": "METRICS"
}
```

### CPU Metrics

```json
[
  {
    "usage_pct": 25.5,
    "brand": "AMD Ryzen 9 5900X",
    "frequency": 3700
  }
]
```

### Memory Metrics

```json
{
  "total_bytes": 17179869184,
  "free_bytes": 8589934592,
  "used_bytes": 8589934592,
  "available_bytes": 9663676416,
  "swap_total_bytes": 2147483648,
  "swap_used_bytes": 536870912
}
```

### Disk Metrics

```json
[
  {
    "name": "/dev/sda1",
    "total_space": 512000000000,
    "available_space": 256000000000,
    "used_space": 256000000000,
    "mount_point": "/",
    "file_system": "ext4"
  }
]
```

### Network Metrics

```json
[
  {
    "interface": "eth0",
    "received_bytes": 1073741824,
    "transmitted_bytes": 536870912,
    "packets_received": 1000000,
    "packets_transmitted": 500000
  }
]
```

### OS Info

```json
{
  "name": "Ubuntu",
  "kernel_version": "6.5.0-generic",
  "os_version": "24.04 LTS",
  "host_name": "my-machine"
}
```

### CPU Components (Temperature Sensors)

```json
[
  {
    "label": "CPU Core",
    "temperature": 45.0,
    "max": 100.0,
    "critical": 105.0
  }
]
```

### Process Metrics

```json
[
  {
    "pid": 1234,
    "name": "chrome",
    "cpu_usage": 12.5,
    "memory_bytes": 2048000000,
    "disk_read_bytes": 1048576,
    "disk_written_bytes": 2097152,
    "status": "Run",
    "user_id": "1000"
  }
]
```

### Extended Process Metrics

```json
[
  {
    "pid": 1234,
    "parent_pid": 1200,
    "name": "chrome",
    "command": "/usr/bin/chrome --some-flag",
    "cpu_usage": 12.5,
    "memory_bytes": 2048000000,
    "disk_read_bytes": 1048576,
    "disk_written_bytes": 2097152,
    "status": "Run",
    "user_id": "1000",
    "start_time": 1700000000
  }
]
```

### Disk I/O Metrics

```json
{
  "read_bytes": 1073741824,
  "written_bytes": 536870912
}
```

### Network I/O Metrics

```json
{
  "rx_bytes": 2147483648,
  "tx_bytes": 1073741824,
  "rx_packets": 2000000,
  "tx_packets": 1000000
}
```

### Battery Info

```json
[
  {
    "state": "Discharging",
    "vendor": "Samsung",
    "model": "Battery 0",
    "cycle_count": 150,
    "health_pct": 92.5,
    "energy_pct": 75.0,
    "energy_full_design_wh": 50.0,
    "energy_full_wh": 46.25,
    "energy_wh": 34.68
  }
]
```

### Load Average

```json
{
  "one_min": 1.25,
  "five_min": 0.95,
  "fifteen_min": 0.75
}
```

### All Metrics

```json
{
  "cpu": [...],
  "memory": {...},
  "disks": [...],
  "networks": [...],
  "uptime": 123456,
  "os_info": {...},
  "load_avg": {...},
  "batteries": [...],
  "components": [...]
}
```

---

## Thread Safety

The METRICS library is **thread-safe**:
- Internal state is protected using `RwLock` (Readers-Writer Lock)
- Multiple threads can read metrics simultaneously
- Write operations (refresh) are serialized
- RwLock poisoning is handled gracefully to prevent panics

### Best Practices for Thread Safety

1. **Use `refresh_metrics()` once, then read multiple metrics:**
   ```c
   // More efficient: refresh once, then get all metrics
   refresh_metrics(REFRESH_ALL);  // Or specific flags
   cpu_json = get_cpu_metrics();
   mem_json = get_memory_metrics();
   disk_json = get_disk_metrics();
   ```

2. **Always call `free_metrics_string()`** after processing each JSON string

3. **Call `cleanup_metrics()`** when done to release resources

---

## Usage Examples

### 1. Python (No dependencies)

Create a file `monitor.py`:

```python
import ctypes
import json

lib = ctypes.CDLL("./target/release/libmetrics.so")

# Set return types
lib.get_library_version.restype = ctypes.c_char_p
lib.get_memory_metrics.restype = ctypes.c_char_p
lib.get_load_average.restype = ctypes.c_char_p
lib.get_cpu_metrics.restype = ctypes.c_char_p

# Get library version
raw_version = lib.get_library_version()
version = json.loads(raw_version.decode())
print(f"METRICS Library Version: {version['version']}")
lib.free_metrics_string(raw_version)

# Refresh all metrics once (more efficient)
lib.refresh_metrics(0xFFFFFFFF)  # REFRESH_ALL

# Get CPU metrics
raw_cpu = lib.get_cpu_metrics()
cpu_data = json.loads(raw_cpu.decode())
print(f"CPU Cores: {len(cpu_data)}")
for cpu in cpu_data:
    print(f"  {cpu['brand']}: {cpu['usage_pct']:.1f}%")
lib.free_metrics_string(raw_cpu)

# Get memory metrics
raw_mem = lib.get_memory_metrics()
mem = json.loads(raw_mem.decode())
print(f"Used RAM: {mem['used_bytes'] / (1024**3):.2f} GiB")
lib.free_metrics_string(raw_mem)

# Get load average
raw_load = lib.get_load_average()
load = json.loads(raw_load.decode())
print(f"Load Average: 1m={load['one_min']}, 5m={load['five_min']}, 15m={load['fifteen_min']}")
lib.free_metrics_string(raw_load)

# Cleanup when done
lib.cleanup_metrics()
```

### 2. Bun (Native FFI)

```typescript
import { dlopen, FFIType, CString } from "bun:ffi";

const lib = dlopen("./target/release/libmetrics.so", {
  get_library_version: { returns: FFIType.ptr, args: [] },
  get_cpu_metrics: { returns: FFIType.ptr, args: [] },
  get_memory_metrics: { returns: FFIType.ptr, args: [] },
  get_load_average: { returns: FFIType.ptr, args: [] },
  refresh_metrics: { returns: FFIType.void, args: [FFIType.u32] },
  free_metrics_string: { returns: FFIType.void, args: [FFIType.ptr] },
});

// Refresh all metrics first (more efficient)
lib.symbols.refresh_metrics(0xFFFFFFFF);

const versionPtr = lib.symbols.get_library_version();
console.log("Version:", JSON.parse(new CString(versionPtr)));
lib.symbols.free_metrics_string(versionPtr);

const cpuPtr = lib.symbols.get_cpu_metrics();
console.log("CPU Metrics:", JSON.parse(new CString(cpuPtr)));
lib.symbols.free_metrics_string(cpuPtr);

const loadPtr = lib.symbols.get_load_average();
console.log("Load Average:", JSON.parse(new CString(loadPtr)));
lib.symbols.free_metrics_string(loadPtr);
```

---

## Building

### Prerequisites

- Rust toolchain (1.75+)

### Commands

```bash
# Build optimized release binary
cargo build --release
```

The compiled library will be in `target/release/`.

---

## Performance Tips

1. **Use `refresh_metrics()` strategically:**
   - Call once before getting multiple metrics
   - Use specific flags to refresh only what you need

2. **Cache static data:**
   - OS info (`get_os_info()`) is automatically cached internally
   - For high-frequency polling, consider caching results in your application

3. **Use `get_all_metrics()` for single-call snapshots:**
   - More efficient when you need all metrics at once

---

## License

MIT License.

