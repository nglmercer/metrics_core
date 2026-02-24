/**
 * FFI bindings for METRICS library
 * Handles library loading and function bindings using Bun's FFI
 */

import { dlopen, FFIType, CString } from "bun:ffi";
import type {
  AllMetrics,
  BatteryInfo,
  LibraryVersion,
  CpuMetrics,
  MemoryMetrics,
  DiskMetrics,
  NetworkMetrics,
  OsInfo,
  LoadAverage,
  ComponentMetrics,
  ProcessMetrics,
  ExtendedProcessMetrics,
  DiskIoMetrics,
  NetworkIoMetrics,
  RefreshFlag,
} from "./types";

/**
 * Get the platform-specific library extension
 */
function getLibExtension(): string {
  switch (process.platform) {
    case "win32":
      return "dll";
    case "darwin":
      return "dylib";
    default:
      return "so";
  }
}

/**
 * Get the platform-specific library name
 */
function getLibName(): string {
  const ext = getLibExtension();
  return process.platform === "win32" ? `metrics.${ext}` : `libmetrics.${ext}`;
}

/**
 * Default library path
 */
export const DEFAULT_LIB_PATH = `./target/release/${getLibName()}`;

/**
 * FFI function definitions
 */
const FFI_DEFS = {
  get_library_version: { returns: FFIType.ptr, args: [] },
  get_all_metrics: { returns: FFIType.ptr, args: [] },
  get_cpu_metrics: { returns: FFIType.ptr, args: [] },
  get_memory_metrics: { returns: FFIType.ptr, args: [] },
  get_disk_metrics: { returns: FFIType.ptr, args: [] },
  get_network_metrics: { returns: FFIType.ptr, args: [] },
  get_uptime: { returns: FFIType.u64, args: [] },
  get_load_average: { returns: FFIType.ptr, args: [] },
  get_os_info: { returns: FFIType.ptr, args: [] },
  get_cpu_components: { returns: FFIType.ptr, args: [] },
  get_processes: { returns: FFIType.ptr, args: [] },
  get_extended_processes: { returns: FFIType.ptr, args: [] },
  get_process_by_pid: { returns: FFIType.ptr, args: [FFIType.u32] },
  get_disk_io: { returns: FFIType.ptr, args: [] },
  get_network_io: { returns: FFIType.ptr, args: [] },
  get_battery_info: { returns: FFIType.ptr, args: [] },
  refresh_metrics: { returns: FFIType.void, args: [FFIType.u32] },
  cleanup_metrics: { returns: FFIType.void, args: [] },
  free_metrics_string: { returns: FFIType.void, args: [FFIType.ptr] },
};

/**
 * Library interface type
 */
interface MetricsLib {
  symbols: {
    get_library_version: () => unknown;
    get_all_metrics: () => unknown;
    get_cpu_metrics: () => unknown;
    get_memory_metrics: () => unknown;
    get_disk_metrics: () => unknown;
    get_network_metrics: () => unknown;
    get_uptime: () => unknown;
    get_load_average: () => unknown;
    get_os_info: () => unknown;
    get_cpu_components: () => unknown;
    get_processes: () => unknown;
    get_extended_processes: () => unknown;
    get_process_by_pid: (pid: number) => unknown;
    get_disk_io: () => unknown;
    get_network_io: () => unknown;
    get_battery_info: () => unknown;
    refresh_metrics: (flags: number) => void;
    cleanup_metrics: () => void;
    free_metrics_string: (ptr: unknown) => void;
  };
}

/**
 * MetricsLibrary class - wraps the native library with type-safe methods
 */
export class MetricsLibrary {
  private lib: MetricsLib;
  private path: string;

  constructor(libPath: string = DEFAULT_LIB_PATH) {
    this.path = libPath;
    try {
      this.lib = dlopen(libPath, FFI_DEFS) as MetricsLib;
    } catch (e) {
      throw new Error(
        `Failed to load library at ${libPath}. Make sure to run 'cargo build --release' first.`
      );
    }
  }

  /**
   * Get the library path
   */
  get libraryPath(): string {
    return this.path;
  }

  /**
   * Convert a pointer to a JSON object and free the string
   */
  private ptrToJson<T>(ptrVal: unknown): T | null {
    if (!ptrVal) return null;
    // Bun's CString accepts the pointer from FFI
    const str = new CString(ptrVal as never).toString();
    this.lib.symbols.free_metrics_string(ptrVal);
    return JSON.parse(str) as T;
  }

  /**
   * Get the library version
   */
  getLibraryVersion(): LibraryVersion {
    const ptrVal = this.lib.symbols.get_library_version();
    return this.ptrToJson<LibraryVersion>(ptrVal) ?? { version: "unknown", name: "METRICS" };
  }

  /**
   * Get all metrics at once
   */
  getAllMetrics(): AllMetrics | null {
    const ptrVal = this.lib.symbols.get_all_metrics();
    return this.ptrToJson<AllMetrics>(ptrVal);
  }

  /**
   * Get CPU metrics
   */
  getCpuMetrics(): CpuMetrics[] | null {
    const ptrVal = this.lib.symbols.get_cpu_metrics();
    return this.ptrToJson<CpuMetrics[]>(ptrVal);
  }

  /**
   * Get memory metrics
   */
  getMemoryMetrics(): MemoryMetrics | null {
    const ptrVal = this.lib.symbols.get_memory_metrics();
    return this.ptrToJson<MemoryMetrics>(ptrVal);
  }

  /**
   * Get disk metrics
   */
  getDiskMetrics(): DiskMetrics[] | null {
    const ptrVal = this.lib.symbols.get_disk_metrics();
    return this.ptrToJson<DiskMetrics[]>(ptrVal);
  }

  /**
   * Get network metrics
   */
  getNetworkMetrics(): NetworkMetrics[] | null {
    const ptrVal = this.lib.symbols.get_network_metrics();
    return this.ptrToJson<NetworkMetrics[]>(ptrVal);
  }

  /**
   * Get system uptime in seconds
   */
  getUptime(): number {
    const val = this.lib.symbols.get_uptime();
    return Number(val);
  }

  /**
   * Get load average
   */
  getLoadAverage(): LoadAverage | null {
    const ptrVal = this.lib.symbols.get_load_average();
    return this.ptrToJson<LoadAverage>(ptrVal);
  }

  /**
   * Get OS information
   */
  getOsInfo(): OsInfo | null {
    const ptrVal = this.lib.symbols.get_os_info();
    return this.ptrToJson<OsInfo>(ptrVal);
  }

  /**
   * Get CPU components (temperature sensors)
   */
  getCpuComponents(): ComponentMetrics[] | null {
    const ptrVal = this.lib.symbols.get_cpu_components();
    return this.ptrToJson<ComponentMetrics[]>(ptrVal);
  }

  /**
   * Get process list
   */
  getProcesses(): ProcessMetrics[] | null {
    const ptrVal = this.lib.symbols.get_processes();
    return this.ptrToJson<ProcessMetrics[]>(ptrVal);
  }

  /**
   * Get extended process list with parent PID and command line
   */
  getExtendedProcesses(): ExtendedProcessMetrics[] | null {
    const ptrVal = this.lib.symbols.get_extended_processes();
    return this.ptrToJson<ExtendedProcessMetrics[]>(ptrVal);
  }

  /**
   * Get process by PID
   */
  getProcessByPid(pid: number): ProcessMetrics | null {
    const ptrVal = this.lib.symbols.get_process_by_pid(pid);
    return this.ptrToJson<ProcessMetrics>(ptrVal);
  }

  /**
   * Get disk I/O metrics
   */
  getDiskIo(): DiskIoMetrics | null {
    const ptrVal = this.lib.symbols.get_disk_io();
    return this.ptrToJson<DiskIoMetrics>(ptrVal);
  }

  /**
   * Get network I/O metrics
   */
  getNetworkIo(): NetworkIoMetrics | null {
    const ptrVal = this.lib.symbols.get_network_io();
    return this.ptrToJson<NetworkIoMetrics>(ptrVal);
  }

  /**
   * Get battery information
   */
  getBatteryInfo(): BatteryInfo[] | null {
    const ptrVal = this.lib.symbols.get_battery_info();
    return this.ptrToJson<BatteryInfo[]>(ptrVal);
  }

  /**
   * Refresh internal metric caches
   */
  refreshMetrics(flags: RefreshFlag): void {
    this.lib.symbols.refresh_metrics(flags);
  }

  /**
   * Cleanup internal resources
   */
  cleanupMetrics(): void {
    this.lib.symbols.cleanup_metrics();
  }
}

/**
 * Create a new MetricsLibrary instance
 */
export function createMetricsLibrary(libPath?: string): MetricsLibrary {
  return new MetricsLibrary(libPath);
}

/**
 * Default export for convenience
 */
export default MetricsLibrary;