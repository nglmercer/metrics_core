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

| Platform | Status  | Binary             |
| -------- | ------- | ------------------ |
| Linux    | ✅ Full | `libmetrics.so`    |
| Windows  | ✅ Full | `metrics.dll`      |
| macOS    | ✅ Full | `libmetrics.dylib` |

---

## Exposed FFI Functions

All functions use the C ABI (`extern "C"`) and are name-mangled for easy discovery.

### `get_cpu_metrics`

```c
char* get_cpu_metrics();
```

Returns a **JSON array** of CPU metrics.

### `get_memory_metrics`

```c
char* get_memory_metrics();
```

Returns a **JSON object** of memory metrics.

### `get_disk_metrics`

```c
char* get_disk_metrics();
```

Returns a **JSON array** of disk metrics.

### `get_network_metrics`

```c
char* get_network_metrics();
```

Returns a **JSON array** of network interface metrics.

### `get_uptime`

```c
uint64_t get_uptime();
```

Returns the system uptime in seconds. (Does not return a pointer, no need to free).

### `free_metrics_string`

```c
void free_metrics_string(char* s);
```

Frees the memory allocated by any function returning a `char*`. Always call this after processing the JSON.

---

## JSON Output Schemas

### CPU Metrics (`get_cpu_metrics`)

```json
[
  {
    "usage_pct": 15.5,
    "brand": "AMD EPYC 7642 48-Core Processor",
    "frequency": 2299
  }
]
```

### Memory Metrics (`get_memory_metrics`)

```json
{
  "total_kb": 16777216,
  "free_kb": 8192000,
  "used_kb": 7500000,
  "available_kb": 9200000
}
```

### Disk Metrics (`get_disk_metrics`)

```json
[
  {
    "name": "/dev/sda",
    "total_space": 500000000000,
    "available_space": 200000000000,
    "mount_point": "/"
  }
]
```

### Network Metrics (`get_network_metrics`)

```json
[
  {
    "interface": "eth0",
    "received_bytes": 1234567890,
    "transmitted_bytes": 987654321
  }
]
```

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

lib = ctypes.CDLL("./libmetrics.so")
lib.get_cpu_metrics.restype = ctypes.c_char_p
lib.get_memory_metrics.restype = ctypes.c_char_p

# Access specific components
cpu_json = lib.get_cpu_metrics()
print(json.loads(cpu_json.decode()))
lib.free_metrics_string(cpu_json)

# Access non-string return types
uptime = lib.get_uptime()
print(f"Uptime: {uptime}s")
```

### Node.js (ffi-napi)

```javascript
const ffi = require("ffi-napi");
const metrics = ffi.Library("./libmetrics", {
  get_cpu_metrics: ["string", []],
  get_uptime: ["uint64", []],
  free_metrics_string: ["void", ["string"]],
});

console.log(JSON.parse(metrics.get_cpu_metrics()));
console.log(`Uptime: ${metrics.get_uptime()}s`);
```

### Go (cgo)

```go
/*
#cgo LDFLAGS: -L. -lmetrics
extern char* get_cpu_metrics();
extern long long get_uptime();
extern void free_metrics_string(char* s);
*/
import "C"
import "fmt"

func main() {
    cStr := C.get_cpu_metrics()
    fmt.Println(C.GoString(cStr))
    C.free_metrics_string(cStr)
    fmt.Printf("Uptime: %d\n", C.get_uptime())
}
```

### Bun (FFI)

```typescript
import { dlopen, FFIType, ptr } from "bun:ffi";

const path = `libmetrics.${process.platform === "win32" ? "dll" : process.platform === "darwin" ? "dylib" : "so"}`;

const lib = dlopen(path, {
  get_cpu_metrics: { returns: FFIType.cstring, args: [] },
  get_memory_metrics: { returns: FFIType.cstring, args: [] },
  get_uptime: { returns: FFIType.u64, args: [] },
});

console.log("CPU:", JSON.parse(lib.symbols.get_cpu_metrics()!));
console.log("Memory:", JSON.parse(lib.symbols.get_memory_metrics()!));
console.log("Uptime:", lib.symbols.get_uptime(), "s");
```

### C / C++

```c
#include <stdio.h>
#include <stdint.h>

char* get_cpu_metrics();
uint64_t get_uptime();
void free_metrics_string(char* s);

int main() {
    char* json = get_cpu_metrics();
    if (json) {
        printf("CPU: %s\n", json);
        free_metrics_string(json);
    }
    printf("Uptime: %lu s\n", get_uptime());
    return 0;
}
```

---

## Integration with VPS Panels

This library is designed to be the **core metrics engine** for VPS management panels. The component-based design allows you to fetch only the data you need, reducing overhead.

### Example: Updating a Prometheus exporter (Python)

```python
import ctypes
import json
import time

lib = ctypes.CDLL("./libmetrics.so")
lib.get_cpu_metrics.restype = ctypes.c_char_p

def collect_metrics():
    # Only fetch CPU if that's all we need
    cpu_json = lib.get_cpu_metrics()
    if cpu_json:
        data = json.loads(cpu_json.decode())
        # Update your gauges here...
        lib.free_metrics_string(cpu_json)

while True:
    collect_metrics()
    time.sleep(5)
```

---

## Dependencies

| Crate        | Version | Purpose                                                          |
| ------------ | ------- | ---------------------------------------------------------------- |
| `sysinfo`    | 0.30+   | Cross-platform system information (CPU, memory, disks, networks) |
| `serde`      | 1.0+    | Serialization framework                                          |
| `serde_json` | 1.0+    | JSON output                                                      |
| `libc`       | 0.2+    | C type definitions for FFI                                       |

---

## License

MIT License — free to use, modify, and distribute.

---

## Contributing

Contributions are welcome! Please ensure:

1. New platform implementations follow the existing pattern in `src/platform/`
2. The JSON schema remains backward-compatible
3. All FFI functions are marked with `#[no_mangle]` and `extern "C"`
