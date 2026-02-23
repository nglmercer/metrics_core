# METRICS — System Monitoring FFI Library

A high-performance, cross-platform system metrics extraction library written in Rust with native C-ABI bindings. Designed to be embedded in any language that supports FFI calls — Python, Node.js, Go, C/C++, PHP, Ruby, and more.

---

## Why This Library?

Most monitoring solutions either:
- Are too high-level (don't give you raw data)
- Require language-specific agents (Python scripts, Node modules)
- Lack cross-platform support

**METRICS** gives you a single `.so`/`.dll`/`.dylib` binary that speaks raw C to your application. Zero dependencies. Zero interpreters. Just fast, direct memory access to system data.

---

## Supported Platforms

| Platform | Status | Binary |
|----------|--------|--------|
| Linux    | ✅ Full | `libmetrics.so` |
| Windows  | ✅ Full | `metrics.dll` |
| macOS    | ✅ Full | `libmetrics.dylib` |

---

## Exposed FFI Functions

All functions use the C ABI (`extern "C"`) and are name-mangled for easy discovery.

### `get_vps_metrics`

```c
char* get_vps_metrics();
```

Returns a **JSON string** containing all system metrics. The caller **must** free the returned string using `free_metrics_string()` to prevent memory leaks.

**Return value:** JSON string (caller frees with `free_metrics_string`) or `NULL` on error.

---

### `free_metrics_string`

```c
void free_metrics_string(char* s);
```

Frees the memory allocated by `get_vps_metrics()`. Always call this after processing the JSON.

**Parameters:**
- `s` — Pointer to the string returned by `get_vps_metrics()`

---

## JSON Output Schema

The returned JSON follows this structure:

```json
{
  "cpus": [
    {
      "usage_pct": 15.5,
      "brand": "AMD EPYC 7642 48-Core Processor",
      "frequency": 2299
    }
  ],
  "memory": {
    "total_kb": 16777216,
    "free_kb": 8192000,
    "used_kb": 7500000,
    "available_kb": 9200000
  },
  "disks": [
    {
      "name": "/dev/sda",
      "total_space": 500000000000,
      "available_space": 200000000000,
      "mount_point": "/"
    }
  ],
  "networks": [
    {
      "interface": "eth0",
      "received_bytes": 1234567890,
      "transmitted_bytes": 987654321
    }
  ],
  "uptime_sec": 3600000
}
```

### Field Definitions

| Field | Type | Description |
|-------|------|-------------|
| `cpus[].usage_pct` | f32 | CPU usage percentage (0-100) |
| `cpus[].brand` | string | CPU model name |
| `cpus[].frequency` | u64 | CPU frequency in MHz |
| `memory.total_kb` | u64 | Total memory in KB |
| `memory.free_kb` | u64 | Free memory in KB |
| `memory.used_kb` | u64 | Used memory in KB |
| `memory.available_kb` | u64 | Available memory in KB |
| `disks[].name` | string | Disk device name |
| `disks[].total_space` | u64 | Total disk space in bytes |
| `disks[].available_space` | u64 | Available disk space in bytes |
| `disks[].mount_point` | string | Mount point path |
| `networks[].interface` | string | Network interface name |
| `networks[].received_bytes` | u64 | Total bytes received |
| `networks[].transmitted_bytes` | u64 | Total bytes transmitted |
| `uptime_sec` | u64 | System uptime in seconds |

---

## Building

### Prerequisites

- Rust toolchain (1.70+)
- Cargo

### Build Commands

```bash
# Debug build
cargo build

# Release build (optimized, smaller binary)
cargo build --release

# Build as a dynamic library (.so/.dll/.dylib)
cargo build --release --lib
```

The compiled library will be in:
- Debug: `target/debug/libmetrics.so` (Linux), `target/debug/metrics.dll` (Windows)
- Release: `target/release/libmetrics.so` (Linux), `target/release/metrics.dll` (Windows)

---

## Usage Examples

### Python (ctypes)

```python
import ctypes
import json
import os

# Load the shared library
lib = ctypes.CDLL("./libmetrics.so")

# Define return type
lib.get_vps_metrics.restype = ctypes.c_char_p

# Call the function
json_str = lib.get_vps_metrics()
if json_str:
    metrics = json.loads(json_str.decode("utf-8"))
    
    print(f"CPU Usage: {metrics['cpus'][0]['usage_pct']}%")
    print(f"Memory Available: {metrics['memory']['available_kb']} KB")
    print(f"Uptime: {metrics['uptime_sec']} seconds")
    
    # CRITICAL: Free the memory!
    lib.free_metrics_string(json_str)
else:
    print("Failed to get metrics")
```

### Node.js (ffi-napi)

```javascript
const ffi = require('ffi-napi');
const ref = require('ref-napi');

// Define types
const charPtr = ref.refType(ref.types.char);

// Load library
const metrics = ffi.Library('./libmetrics', {
  get_vps_metrics: [charPtr, []],
  free_metrics_string: ['void', [charPtr]]
});

// Get metrics
const jsonPtr = metrics.get_vps_metrics();
const jsonStr = jsonPtr.readCString();

const metricsData = JSON.parse(jsonStr);

console.log(`CPU Usage: ${metricsData.cpus[0].usage_pct}%`);
console.log(`Memory Available: ${metricsData.memory.available_kb} KB`);

// Free memory
metrics.free_metrics_string(jsonPtr);
```

### Go (cgo)

```go
package main

/*
#cgo LDFLAGS: -L. -lmetrics
#include <stdlib.h>
#include <string.h>

extern char* get_vps_metrics();
extern void free_metrics_string(char* s);
*/
import "C"
import (
	"encoding/json"
	"fmt"
	"unsafe"
)

func main() {
	// Get metrics
	cStr := C.get_vps_metrics()
	if cStr == nil {
		panic("Failed to get metrics")
	}

	// Convert to Go string
	jsonStr := C.GoString(cStr)

	// Parse JSON
	var metrics map[string]interface{}
	json.Unmarshal([]byte(jsonStr), &metrics)

	// Print some data
	cpus := metrics["cpus"].([]interface{})
	cpu := cpus[0].(map[string]interface{})
	fmt.Printf("CPU Usage: %v%%\n", cpu["usage_pct"])
	fmt.Printf("Memory Available: %v KB\n", metrics["memory"].(map[string]interface{})["available_kb"])

	// Free memory
	C.free_metrics_string(cStr)
}
```

### C / C++

```c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Function declarations
char* get_vps_metrics();
void free_metrics_string(char* s);

int main() {
    // Get metrics
    char* json_str = get_vps_metrics();
    if (json_str == NULL) {
        fprintf(stderr, "Failed to get metrics\n");
        return 1;
    }

    // Process JSON (use your favorite JSON library)
    printf("Raw JSON: %s\n", json_str);

    // IMPORTANT: Free the memory!
    free_metrics_string(json_str);

    return 0;
}
```

---

## Integration with VPS Panels

This library is designed to be the **core metrics engine** for VPS management panels. The typical integration pattern is:

1. **Build** the library for your target OS
2. **Distribute** the binary alongside your panel agent
3. **Call** `get_vps_metrics()` on a configurable interval (e.g., every 5 seconds)
4. **Parse** the JSON and store/display the metrics
5. **Free** the string to prevent memory leaks

### Example: Updating a Prometheus exporter

```python
import ctypes
import json
import time
from prometheus_client import Gauge, start_http_server

# Define Prometheus metrics
cpu_gauge = Gauge('vps_cpu_percent', 'CPU usage percentage')
mem_gauge = Gauge('vps_memory_available_kb', 'Available memory in KB')
disk_gauge = Gauge('vps_disk_available_bytes', 'Available disk space in bytes')

# Load library
lib = ctypes.CDLL("./libmetrics.so")
lib.get_vps_metrics.restype = ctypes.c_char_p

def collect_metrics():
    json_str = lib.get_vps_metrics()
    if json_str:
        data = json.loads(json_str.decode())
        
        # Update Prometheus gauges
        cpu_gauge.set(data['cpus'][0]['usage_pct'])
        mem_gauge.set(data['memory']['available_kb'])
        disk_gauge.set(data['disks'][0]['available_space'])
        
        lib.free_metrics_string(json_str)

# Start Prometheus server
start_http_server(9090)

# Collect metrics every 5 seconds
while True:
    collect_metrics()
    time.sleep(5)
```

---

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `sysinfo` | 0.30+ | Cross-platform system information (CPU, memory, disks, networks) |
| `serde` | 1.0+ | Serialization framework |
| `serde_json` | 1.0+ | JSON output |
| `libc` | 0.2+ | C type definitions for FFI |

---

## License

MIT License — free to use, modify, and distribute.

---

## Contributing

Contributions are welcome! Please ensure:
1. New platform implementations follow the existing pattern in `src/platform/`
2. The JSON schema remains backward-compatible
3. All FFI functions are marked with `#[no_mangle]` and `extern "C"`
