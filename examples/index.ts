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