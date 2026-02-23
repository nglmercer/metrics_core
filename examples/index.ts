import { dlopen, FFIType, CString } from "bun:ffi";

// --- Types ---
interface CpuMetrics {
  usage_pct: number;
  brand: string;
  frequency: number;
}

interface MemoryMetrics {
  total_bytes: number;
  free_bytes: number;
  used_bytes: number;
  available_bytes: number;
}

interface DiskMetrics {
  name: string;
  total_space: number;
  available_space: number;
  mount_point: string;
}

interface NetworkMetrics {
  interface: string;
  received_bytes: number;
  transmitted_bytes: number;
}

// --- Library Loading ---
const libPath = `./target/release/libmetrics.${
  process.platform === "win32" ? "dll" : process.platform === "darwin" ? "dylib" : "so"
}`;

let lib: any;
try {
  lib = dlopen(libPath, {
    get_cpu_metrics: { returns: FFIType.ptr, args: [] },
    get_memory_metrics: { returns: FFIType.ptr, args: [] },
    get_network_metrics: { returns: FFIType.ptr, args: [] },
    get_disk_metrics: { returns: FFIType.ptr, args: [] },
    get_uptime: { returns: FFIType.u64, args: [] },
    free_metrics_string: { returns: FFIType.void, args: [FFIType.ptr] },
  });
} catch (e) {
  console.error(`\x1b[31mFailed to load library at ${libPath}\x1b[0m`);
  console.error(`Make sure to run 'cargo build --release' first.`);
  process.exit(1);
}

// --- UI Helpers ---
// Use binary prefixes (KiB, MiB, GiB) to be precise and avoid confusion
const formatBytes = (bytes: number) => {
  const units = ['B', 'KiB', 'MiB', 'GiB', 'TiB', 'PiB'];
  let size = bytes;
  let unitIndex = 0;
  while (size >= 1024 && unitIndex < units.length - 1) {
    size /= 1024;
    unitIndex++;
  }
  return `${size.toFixed(2)} ${units[unitIndex]}`;
};

const colors = {
  reset: "\x1b[0m",
  bright: "\x1b[1m",
  cyan: "\x1b[36m",
  green: "\x1b[32m",
  yellow: "\x1b[33m",
  magenta: "\x1b[35m",
  red: "\x1b[31m",
  dim: "\x1b[2m",
};

const clearScreen = () => process.stdout.write("\x1Bc");

// --- Main Loop ---
console.log(`${colors.bright}${colors.green}Starting METRICS Pro...${colors.reset}`);

// Pre-refresh to get initial CPU readings
lib.symbols.get_cpu_metrics();

while (true) {
  clearScreen();
  console.log(`${colors.bright}${colors.cyan}=== SYSTEM MONITOR PRO ===${colors.reset}`);
  
  const uptimeSeconds = Number(lib.symbols.get_uptime());
  const hours = Math.floor(uptimeSeconds / 3600);
  const minutes = Math.floor((uptimeSeconds % 3600) / 60);
  const seconds = uptimeSeconds % 60;
  
  console.log(`${colors.yellow}Uptime: ${hours}h ${minutes}m ${seconds}s${colors.reset}\n`);

  // CPU
  const cpuPtr = lib.symbols.get_cpu_metrics();
  if (cpuPtr) {
    const jsonStr = new CString(cpuPtr).toString();
    const cpus = JSON.parse(jsonStr) as CpuMetrics[];
    lib.symbols.free_metrics_string(cpuPtr);
    
    console.log(`${colors.bright}CPU Statistics:${colors.reset}`);
    const avgUsage = cpus.reduce((acc, cpu) => acc + cpu.usage_pct, 0) / cpus.length;
    const barWidth = 30;
    const filled = Math.round((avgUsage / 100) * barWidth);
    const bar = "█".repeat(filled) + "░".repeat(barWidth - filled);
    
    console.log(`  Usage: [${colors.green}${bar}${colors.reset}] ${avgUsage.toFixed(1)}%`);
    console.log(`  Model: ${colors.dim}${cpus[0]?.brand || 'Unknown'}${colors.reset}`);
    console.log(`  Cores: ${cpus.length} @ ${cpus[0]?.frequency || 0}MHz\n`);
  }

  // Memory
  const memPtr = lib.symbols.get_memory_metrics();
  if (memPtr) {
    const mem = JSON.parse(new CString(memPtr).toString()) as MemoryMetrics;
    lib.symbols.free_metrics_string(memPtr);

    const usedPct = (mem.used_bytes / mem.total_bytes) * 100;
    const barWidth = 30;
    const filled = Math.round((usedPct / 100) * barWidth);
    const bar = "█".repeat(filled) + "░".repeat(barWidth - filled);

    console.log(`${colors.bright}Memory Usage:${colors.reset}`);
    console.log(`  RAM:   [${colors.yellow}${bar}${colors.reset}] ${usedPct.toFixed(1)}%`);
    console.log(`  Total: ${formatBytes(mem.total_bytes)}`);
    console.log(`  Used:  ${formatBytes(mem.used_bytes)}`);
    console.log(`  Avail: ${formatBytes(mem.available_bytes)}\n`);
  }

  // Storage
  const diskPtr = lib.symbols.get_disk_metrics();
  if (diskPtr) {
    const disks = JSON.parse(new CString(diskPtr).toString()) as DiskMetrics[];
    lib.symbols.free_metrics_string(diskPtr);

    console.log(`${colors.bright}Storage Devices:${colors.reset}`);
    // Deduplicate mount points to avoid clutter
    const seenMounts = new Set();
    for (const disk of disks) {
      if (disk.total_space === 0 || seenMounts.has(disk.mount_point)) continue;
      seenMounts.add(disk.mount_point);
      
      const usage = ((disk.total_space - disk.available_space) / disk.total_space) * 100;
      const barWidth = 15;
      const filled = Math.round((usage / 100) * barWidth);
      const bar = "█".repeat(filled) + "░".repeat(barWidth - filled);
      
      console.log(`  ${disk.mount_point.padEnd(10)} [${colors.cyan}${bar}${colors.reset}] ${usage.toFixed(1)}% of ${formatBytes(disk.total_space)}`);
    }
    console.log("");
  }

  // Network
  const netPtr = lib.symbols.get_network_metrics();
  if (netPtr) {
    const networks = JSON.parse(new CString(netPtr).toString()) as NetworkMetrics[];
    lib.symbols.free_metrics_string(netPtr);

    console.log(`${colors.bright}Network Traffic:${colors.reset}`);
    for (const net of networks) {
      if (net.received_bytes === 0 && net.transmitted_bytes === 0) continue;
      console.log(`  ${colors.magenta}${net.interface.padEnd(12)}${colors.reset} ↓ ${formatBytes(net.received_bytes).padEnd(12)} ↑ ${formatBytes(net.transmitted_bytes)}`);
    }
  }

  console.log(`\n${colors.dim}Refreshing every 2s... (Press CTRL+C to exit)${colors.reset}`);
  await new Promise(resolve => setTimeout(resolve, 2000));
}
