/**
 * Type definitions for METRICS library
 * These types match the Rust structs defined in src/types.rs
 */

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

export interface ExtendedProcessMetrics {
  pid: number;
  parent_pid: number | null;
  name: string;
  command: string | null;
  cpu_usage: number;
  memory_bytes: number;
  disk_read_bytes: number;
  disk_written_bytes: number;
  status: string;
  user_id: string | null;
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

export interface LibraryVersion {
  version: string;
  name: string;
}

/**
 * Refresh flags for the refresh_metrics function
 */
export const RefreshFlags = {
  CPU: 1,
  MEMORY: 2,
  DISKS: 4,
  NETWORKS: 8,
  PROCESSES: 16,
  COMPONENTS: 32,
  ALL: 0xffffffff,
} as const;

export type RefreshFlag = (typeof RefreshFlags)[keyof typeof RefreshFlags];
