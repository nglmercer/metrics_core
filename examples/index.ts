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
lib.symbols.get_cpu_metrics(); // Dummy call to init CPU history

while (true) {
  clearScreen();
  console.log(`${colors.bright}${colors.cyan}=== METRICS PRO (BUN FFI) ===${colors.reset}`);
  
  const uptimeSeconds = Number(lib.symbols.get_uptime());
  const h = Math.floor(uptimeSeconds / 3600);
  const m = Math.floor((uptimeSeconds % 3600) / 60);
  const s = uptimeSeconds % 60;
  console.log(`${colors.yellow}Uptime: ${h}h ${m}m ${s}s${colors.reset}\n`);

  // CPU
  const cpuPtr = lib.symbols.get_cpu_metrics();
  if (cpuPtr) {
    const cpus = JSON.parse(new CString(cpuPtr).toString()) as CpuMetrics[];
    lib.symbols.free_metrics_string(cpuPtr);
    const avg = cpus.reduce((acc, c) => acc + c.usage_pct, 0) / cpus.length;
    const bar = "█".repeat(Math.round(avg / 4)) + "░".repeat(25 - Math.round(avg / 4));
    console.log(`${colors.bright}CPU usage:${colors.reset} [${colors.green}${bar}${colors.reset}] ${avg.toFixed(1)}%`);
    console.log(`${colors.dim}  ${cpus[0]?.brand || 'N/A'} (${cpus.length} cores)${colors.reset}\n`);
  }

  // RAM
  const memPtr = lib.symbols.get_memory_metrics();
  if (memPtr) {
    const mem = JSON.parse(new CString(memPtr).toString()) as MemoryMetrics;
    lib.symbols.free_metrics_string(memPtr);
    const pct = (mem.used_bytes / mem.total_bytes) * 100;
    const bar = "█".repeat(Math.round(pct / 4)) + "░".repeat(25 - Math.round(pct / 4));
    console.log(`${colors.bright}RAM usage:${colors.reset} [${colors.yellow}${bar}${colors.reset}] ${pct.toFixed(1)}%`);
    console.log(`  ${formatBytes(mem.used_bytes)} / ${formatBytes(mem.total_bytes)}\n`);
  }

  // Storage (Deduplicated)
  const diskPtr = lib.symbols.get_disk_metrics();
  if (diskPtr) {
    const disks = JSON.parse(new CString(diskPtr).toString()) as DiskMetrics[];
    lib.symbols.free_metrics_string(diskPtr);
    console.log(`${colors.bright}Storage:${colors.reset}`);
    const seen = new Set();
    for (const d of disks) {
      const id = `${d.name}_${d.total_space}`;
      if (d.total_space === 0 || seen.has(id)) continue;
      seen.add(id);
      const used = ((d.total_space - d.available_space) / d.total_space) * 100;
      console.log(`  ${d.mount_point.padEnd(12)} ${used.toFixed(1)}% of ${formatBytes(d.total_space)}`);
    }
    console.log("");
  }

  // Network
  const netPtr = lib.symbols.get_network_metrics();
  if (netPtr) {
    const nets = JSON.parse(new CString(netPtr).toString()) as NetworkMetrics[];
    lib.symbols.free_metrics_string(netPtr);
    console.log(`${colors.bright}Network Traffic:${colors.reset}`);
    for (const n of nets) {
      if (n.received_bytes === 0 && n.transmitted_bytes === 0) continue;
      console.log(`  ${colors.magenta}${n.interface.padEnd(10)}${colors.reset} ↓ ${formatBytes(n.received_bytes).padEnd(10)} ↑ ${formatBytes(n.transmitted_bytes)}`);
    }
  }

  console.log(`\n${colors.dim}Refreshing in 2s...${colors.reset}`);
  await new Promise(r => setTimeout(r, 2000));
}
