/**
 * METRICS System Monitor - Bun FFI Example
 * 
 * This file is kept for backward compatibility.
 * For the modularized version, see the examples/bun/ directory.
 * 
 * Usage:
 *   bun run examples/index.ts          # Run this file (legacy)
 *   bun run examples/bun/index.ts      # Run modularized version
 */

// Re-export from the modularized version
export { MetricsLibrary, createMetricsLibrary } from "./bun/ffi.js";
export type {
  AllMetrics,
  BatteryInfo,
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
  LibraryVersion,
  RefreshFlag,
} from "./bun/types.js";
export { RefreshFlags } from "./bun/types.js";
export {
  colors,
  formatBytes,
  formatUptime,
  createProgressBar,
  createColoredProgressBar,
  clearScreen,
} from "./bun/ui.js";
export { SystemMonitor } from "./bun/index.js";

// Run the monitor if this is the main entry point
import { SystemMonitor } from "./bun/index.js";

async function main() {
  const monitor = new SystemMonitor();
  await monitor.start();
}

main();
