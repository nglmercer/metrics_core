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
| Windows  | ✅ Full | `metrics.dll`      |
| macOS    | ✅ Full | `libmetrics.dylib` |

---

## Exposed FFI Functions

All functions use the C ABI (`extern "C"`).

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

### `free_metrics_string`

Frees the memory allocated by the library for any JSON string returned. **Must be called after processing the JSON to avoid memory leaks.**

---

## JSON Schemas (v0.1.0)

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

---

## Usage Examples

### 1. Python (No dependencies)

Create a file `monitor.py`:

```python
import ctypes
import json

lib = ctypes.CDLL("./target/release/libmetrics.so")
lib.get_memory_metrics.restype = ctypes.c_char_p
lib.get_load_average.restype = ctypes.c_char_p

# Call and parse memory metrics
raw_json = lib.get_memory_metrics()
data = json.loads(raw_json.decode())
print(f"Used RAM: {data['used_bytes'] / (1024**3):.2f} GiB")

# Call and parse load average
raw_load = lib.get_load_average()
load = json.loads(raw_load.decode())
print(f"Load Average: 1m={load['one_min']}, 5m={load['five_min']}, 15m={load['fifteen_min']}")

# Always free the strings!
lib.free_metrics_string(raw_json)
lib.free_metrics_string(raw_load)
```

### 2. Bun (Native FFI)

```typescript
import { dlopen, FFIType, CString } from "bun:ffi";

const lib = dlopen("./target/release/libmetrics.so", {
  get_cpu_metrics: { returns: FFIType.ptr, args: [] },
  get_load_average: { returns: FFIType.ptr, args: [] },
  free_metrics_string: { returns: FFIType.void, args: [FFIType.ptr] },
});

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

## License

MIT License.
