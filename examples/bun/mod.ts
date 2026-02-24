/**
 * Node.js FFI Bindings for METRICS Library
 * 
 * This module provides a high-level TypeScript interface to the METRICS library
 * using the ffi-napi package.
 * 
 * Usage:
 *     import { Metrics } from './ffi/mod';
 *     
 *     const m = new Metrics();
 *     console.log(m.getAllMetrics());
 *     m.cleanup();
 */

// ============================================================
// TypeScript Types
// ============================================================

export interface CpuMetrics {
  usage_pct: number;
  brand: string;
  frequency: number;
}

export interface MemoryMetrics {
  total_bytes: number;
  free_bytes: number;
  used_bytes: number;
  available_bytes: number;
  swap_total_bytes: number;
  swap_used_bytes: number;
}

export interface DiskMetrics {
  name: string;
  total_space: number;
  available_space: number;
  used_space: number;
  mount_point: string;
  file_system: string;
}

export interface NetworkMetrics {
  interface: string;
  received_bytes: number;
  transmitted_bytes: number;
  packets_received: number;
  packets_transmitted: number;
}

export interface OsInfo {
  name: string;
  kernel_version: string;
  os_version: string;
  host_name: string;
}

export interface ComponentMetrics {
  label: string;
  temperature: number;
  max: number;
  critical: number | null;
}

export interface ProcessMetrics {
  pid: number;
  name: string;
  cpu_usage: number;
  memory_bytes: number;
  disk_read_bytes: number;
  disk_written_bytes: number;
  status: string;
  user_id: string | null;
}

export interface ExtendedProcessMetrics extends ProcessMetrics {
  parent_pid: number | null;
  command: string | null;
  start_time: number;
}

export interface DiskIoMetrics {
  read_bytes: number;
  written_bytes: number;
}

export interface NetworkIoMetrics {
  rx_bytes: number;
  tx_bytes: number;
  rx_packets: number;
  tx_packets: number;
}

export interface BatteryInfo {
  state: string;
  vendor: string | null;
  model: string | null;
  cycle_count: number | null;
  health_pct: number;
  energy_pct: number;
  energy_full_design_wh: number;
  energy_full_wh: number;
  energy_wh: number;
}

export interface LoadAverage {
  one_min: number;
  five_min: number;
  fifteen_min: number;
}

export interface LibraryVersion {
  version: string;
  name: string;
}

export interface AllMetrics {
  cpu: CpuMetrics[];
  memory: MemoryMetrics;
  disks: DiskMetrics[];
  networks: NetworkMetrics[];
  uptime: number;
  os_info: OsInfo;
  load_avg: LoadAverage;
  batteries: BatteryInfo[];
  components: ComponentMetrics[];
}

// ============================================================
// Refresh Flags
// ============================================================

export const RefreshFlags = {
  CPU: 1,
  MEMORY: 2,
  DISKS: 4,
  NETWORKS: 8,
  PROCESSES: 16,
  COMPONENTS: 32,
  ALL: 0xFFFFFFFF,
} as const;

// ============================================================
// Library Loader
// ============================================================

function getLibraryPath(): string {
  const ext = process.platform === 'win32' ? 'dll' 
            : process.platform === 'darwin' ? 'dylib' 
            : 'so';
  const paths = [
    'target/release/libmetrics',
    'target/debug/libmetrics',
    './libmetrics',
    '../target/release/libmetrics',
  ];
  
  for (const path of paths) {
    const fullPath = `${path}.${ext}`;
    const fs = require('fs');
    if (fs.existsSync(fullPath)) {
      return fullPath;
    }
  }
  
  return `target/release/libmetrics.${ext}`;
}

// ============================================================
// Metrics Class (Node.js with ffi-napi)
// ============================================================

export class Metrics {
  private lib: any = null;
  private loaded = false;
  
  constructor() {
    this.load();
  }
  
  private load(): void {
    if (this.loaded) return;
    
    try {
      const ffi = require('ffi-napi');
      const ref = require('ref-napi');
      const libPath = getLibraryPath();
      
      // Define the library interface
      this.lib = ffi.Library(libPath, {
        get_all_metrics: ['pointer', []],
        get_battery_info: ['pointer', []],
        get_library_version: ['pointer', []],
        get_cpu_metrics: ['pointer', []],
        get_memory_metrics: ['pointer', []],
        get_disk_metrics: ['pointer', []],
        get_network_metrics: ['pointer', []],
        get_os_info: ['pointer', []],
        get_cpu_components: ['pointer', []],
        get_processes: ['pointer', []],
        get_extended_processes: ['pointer', []],
        get_process_by_pid: ['pointer', ['uint32']],
        get_disk_io: ['pointer', []],
        get_network_io: ['pointer', []],
        get_load_average: ['pointer', []],
        get_uptime: ['uint64', []],
        refresh_metrics: ['void', ['uint32']],
        cleanup_metrics: ['void', []],
        free_metrics_string: ['void', ['pointer']],
      });
      
      this.loaded = true;
    } catch (error) {
      console.error(`Failed to load METRICS library: ${error}`);
      throw new Error(`Library not found. Make sure to run 'cargo build --release' first.`);
    }
  }
  
  private ptrToJson<T>(ptr: any): T | null {
    if (!ptr || ptr.isNull()) return null;
    
    try {
      const ref = require('ref-napi');
      const result = ptr.readCString();
      this.lib.free_metrics_string(ptr);
      return JSON.parse(result) as T;
    } catch {
      return null;
    }
  }
  
  // ============================================================
  // Core Metrics API
  // ============================================================
  
  getAllMetrics(): AllMetrics {
    const ptr = this.lib.get_all_metrics();
    return this.ptrToJson<AllMetrics>(ptr) ?? this.emptyAllMetrics();
  }
  
  getCpuMetrics(): CpuMetrics[] {
    const ptr = this.lib.get_cpu_metrics();
    return this.ptrToJson<CpuMetrics[]>(ptr) ?? [];
  }
  
  getMemoryMetrics(): MemoryMetrics {
    const ptr = this.lib.get_memory_metrics();
    return this.ptrToJson<MemoryMetrics>(ptr) ?? this.emptyMemoryMetrics();
  }
  
  getDiskMetrics(): DiskMetrics[] {
    const ptr = this.lib.get_disk_metrics();
    return this.ptrToJson<DiskMetrics[]>(ptr) ?? [];
  }
  
  getNetworkMetrics(): NetworkMetrics[] {
    const ptr = this.lib.get_network_metrics();
    return this.ptrToJson<NetworkMetrics[]>(ptr) ?? [];
  }
  
  getOsInfo(): OsInfo {
    const ptr = this.lib.get_os_info();
    return this.ptrToJson<OsInfo>(ptr) ?? this.emptyOsInfo();
  }
  
  getCpuComponents(): ComponentMetrics[] {
    const ptr = this.lib.get_cpu_components();
    return this.ptrToJson<ComponentMetrics[]>(ptr) ?? [];
  }
  
  getProcesses(): ProcessMetrics[] {
    const ptr = this.lib.get_processes();
    return this.ptrToJson<ProcessMetrics[]>(ptr) ?? [];
  }
  
  getExtendedProcesses(): ExtendedProcessMetrics[] {
    const ptr = this.lib.get_extended_processes();
    return this.ptrToJson<ExtendedProcessMetrics[]>(ptr) ?? [];
  }
  
  getProcessByPid(pid: number): ProcessMetrics | null {
    const ptr = this.lib.get_process_by_pid(pid);
    return this.ptrToJson<ProcessMetrics>(ptr);
  }
  
  getDiskIo(): DiskIoMetrics {
    const ptr = this.lib.get_disk_io();
    return this.ptrToJson<DiskIoMetrics>(ptr) ?? { read_bytes: 0, written_bytes: 0 };
  }
  
  getNetworkIo(): NetworkIoMetrics {
    const ptr = this.lib.get_network_io();
    return this.ptrToJson<NetworkIoMetrics>(ptr) ?? { rx_bytes: 0, tx_bytes: 0, rx_packets: 0, tx_packets: 0 };
  }
  
  getBatteryInfo(): BatteryInfo[] {
    const ptr = this.lib.get_battery_info();
    return this.ptrToJson<BatteryInfo[]>(ptr) ?? [];
  }
  
  getLoadAverage(): LoadAverage {
    const ptr = this.lib.get_load_average();
    return this.ptrToJson<LoadAverage>(ptr) ?? { one_min: 0, five_min: 0, fifteen_min: 0 };
  }
  
  getUptime(): number {
    return Number(this.lib.get_uptime());
  }
  
  getLibraryVersion(): LibraryVersion {
    const ptr = this.lib.get_library_version();
    return this.ptrToJson<LibraryVersion>(ptr) ?? { version: 'unknown', name: 'METRICS' };
  }
  
  // ============================================================
  // Control API
  // ============================================================
  
  refreshMetrics(flags: number = RefreshFlags.ALL): void {
    this.lib.refresh_metrics(flags);
  }
  
  cleanup(): void {
    this.lib?.cleanup_metrics();
    this.loaded = false;
  }
  
  // ============================================================
  // Utility Methods
  // ============================================================
  
  static formatBytes(bytes: number): string {
    const units = ['B', 'KiB', 'MiB', 'GiB', 'TiB'];
    let size = bytes;
    let unitIndex = 0;
    while (size >= 1024 && unitIndex < units.length - 1) {
      size /= 1024;
      unitIndex++;
    }
    return `${size.toFixed(2)} ${units[unitIndex]}`;
  }
  
  static formatUptime(seconds: number): string {
    const h = Math.floor(seconds / 3600);
    const m = Math.floor((seconds % 3600) / 60);
    const s = seconds % 60;
    return `${h}h ${m}m ${s}s`;
  }
  
  // ============================================================
  // Private Helpers
  // ============================================================
  
  private emptyAllMetrics(): AllMetrics {
    return {
      cpu: [],
      memory: this.emptyMemoryMetrics(),
      disks: [],
      networks: [],
      uptime: 0,
      os_info: this.emptyOsInfo(),
      load_avg: { one_min: 0, five_min: 0, fifteen_min: 0 },
      batteries: [],
      components: [],
    };
  }
  
  private emptyMemoryMetrics(): MemoryMetrics {
    return {
      total_bytes: 0,
      free_bytes: 0,
      used_bytes: 0,
      available_bytes: 0,
      swap_total_bytes: 0,
      swap_used_bytes: 0,
    };
  }
  
  private emptyOsInfo(): OsInfo {
    return {
      name: '',
      kernel_version: '',
      os_version: '',
      host_name: '',
    };
  }
}

// ============================================================
// Convenience Exports
// ============================================================

let _defaultMetrics: Metrics | null = null;

export function getMetrics(): Metrics {
  if (!_defaultMetrics) {
    _defaultMetrics = new Metrics();
  }
  return _defaultMetrics;
}

export function getAllMetrics(): AllMetrics {
  return getMetrics().getAllMetrics();
}

export function cleanup(): void {
  if (_defaultMetrics) {
    _defaultMetrics.cleanup();
    _defaultMetrics = null;
  }
}
