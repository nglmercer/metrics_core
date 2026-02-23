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

### `free_metrics_string`

Frees the memory allocated by the library for any JSON string returned. **Must be called after processing the JSON to avoid memory leaks.**

---

## JSON Schema (v0.1.0)

### Memory Metrics

```json
{
  "total_bytes": 17179869184,
  "free_bytes": 8589934592,
  "used_bytes": 8589934592,
  "available_bytes": 9663676416
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

# Call and parse
raw_json = lib.get_memory_metrics()
data = json.loads(raw_json.decode())
print(f"Used RAM: {data['used_bytes'] / (1024**3):.2f} GiB")

# Always free the string!
lib.free_metrics_string(raw_json)
```

### 2. Bun (Native FFI)

```typescript
import { dlopen, FFIType, CString } from "bun:ffi";

const lib = dlopen("./target/release/libmetrics.so", {
  get_cpu_metrics: { returns: FFIType.ptr, args: [] },
  free_metrics_string: { returns: FFIType.void, args: [FFIType.ptr] },
});

const ptr = lib.symbols.get_cpu_metrics();
console.log(JSON.parse(new CString(ptr)));
lib.symbols.free_metrics_string(ptr);
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
