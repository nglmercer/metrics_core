import { dlopen, FFIType, CString } from "bun:ffi";

// --- Types ---
interface AllMetrics {
  cpu: CpuMetrics[];
  memory: MemoryMetrics;
  disks: DiskMetrics[];
  networks: NetworkMetrics[];
  uptime: number;
  os_info: OsInfo;
  load_avg: LoadAverage;
}

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
  swap_total_bytes: number;
  swap_used_bytes: number;
}

interface LoadAverage {
  one_min: number;
  five_min: number;
  fifteen_min: number;
}

interface DiskMetrics {
  name: string;
  total_space: number;
  available_space: number;
  mount_point: string;
  file_system: string;
}

interface NetworkMetrics {
  interface: string;
  received_bytes: number;
  transmitted_bytes: number;
}

interface OsInfo {
  name: string;
  kernel_version: string;
  os_version: string;
  host_name: string;
}

interface BatteryInfo {
  state: string;
  vendor: string | null;
  model: string | null;
  energy_pct: number;
  health_pct: number;
}

// --- Library Loading ---
const libPath = `./target/release/libmetrics.${
  process.platform === "win32" ? "dll" : process.platform === "darwin" ? "dylib" : "so"
}`;

let lib: any;
try {
  lib = dlopen(libPath, {
    get_all_metrics: { returns: FFIType.ptr, args: [] },
    get_battery_info: { returns: FFIType.ptr, args: [] },
    get_library_version: { returns: FFIType.ptr, args: [] },
    free_metrics_string: { returns: FFIType.void, args: [FFIType.ptr] },
  });
} catch (e) {
  console.error(`\x1b[31mFailed to load library at ${libPath}\x1b[0m`);
  console.error(e);
  console.error(`Make sure to run 'cargo build --release' first.`);
  process.exit(1);
}

// --- UI Helpers ---
const formatBytes = (bytes: number) => {
  const units = ['B', 'KiB', 'MiB', 'GiB', 'TiB'];
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
const versionPtr = lib.symbols.get_library_version();
const version = versionPtr ? JSON.parse(new CString(versionPtr).toString()) : { version: "unknown" };
lib.symbols.free_metrics_string(versionPtr);

while (true) {
  clearScreen();
  console.log(`${colors.bright}${colors.cyan}=== METRICS PRO v${version.version} (BUN FFI) ===${colors.reset}`);
  
  const ptr = lib.symbols.get_all_metrics();
  if (!ptr) {
    console.log("Failed to fetch metrics.");
    await new Promise(r => setTimeout(r, 2000));
    continue;
  }
  
  const data = JSON.parse(new CString(ptr).toString()) as AllMetrics;
  lib.symbols.free_metrics_string(ptr);

  const uptimeSeconds = data.uptime;
  const h = Math.floor(uptimeSeconds / 3600);
  const m = Math.floor((uptimeSeconds % 3600) / 60);
  const s = uptimeSeconds % 60;
  
  // OS & Load
  console.log(`${colors.dim}${data.os_info.name} ${data.os_info.os_version} | ${data.os_info.host_name}${colors.reset}`);
  console.log(`${colors.yellow}Uptime: ${h}h ${m}m ${s}s | Load: ${data.load_avg.one_min.toFixed(2)} ${data.load_avg.five_min.toFixed(2)} ${data.load_avg.fifteen_min.toFixed(2)}${colors.reset}\n`);

  // CPU
  const avgCpu = data.cpu.reduce((acc, c) => acc + c.usage_pct, 0) / data.cpu.length;
  const cpuBar = "█".repeat(Math.round(avgCpu / 4)) + "░".repeat(25 - Math.round(avgCpu / 4));
  console.log(`${colors.bright}CPU usage:${colors.reset} [${colors.green}${cpuBar}${colors.reset}] ${avgCpu.toFixed(1)}%`);
  console.log(`${colors.dim}  ${data.cpu[0]?.brand || 'N/A'} (${data.cpu.length} cores @ ${data.cpu[0]?.frequency}MHz)${colors.reset}\n`);

  // RAM
  const ramPct = (data.memory.used_bytes / data.memory.total_bytes) * 100;
  const ramBar = "█".repeat(Math.round(ramPct / 4)) + "░".repeat(25 - Math.round(ramPct / 4));
  console.log(`${colors.bright}RAM usage:${colors.reset} [${colors.yellow}${ramBar}${colors.reset}] ${ramPct.toFixed(1)}%`);
  console.log(`  ${formatBytes(data.memory.used_bytes)} / ${formatBytes(data.memory.total_bytes)}\n`);

  // Battery
  const batPtr = lib.symbols.get_battery_info();
  if (batPtr) {
    const batteries = JSON.parse(new CString(batPtr).toString()) as BatteryInfo[];
    lib.symbols.free_metrics_string(batPtr);
    if (batteries.length > 0) {
      console.log(`${colors.bright}Battery:${colors.reset}`);
      for (const b of batteries) {
        console.log(`  ${b.state}: ${b.energy_pct.toFixed(1)}% (Health: ${b.health_pct.toFixed(1)}%)`);
      }
      console.log("");
    }
  }

  // Storage
  console.log(`${colors.bright}Storage:${colors.reset}`);
  const seen = new Set();
  for (const d of data.disks) {
    const id = `${d.name}_${d.total_space}`;
    if (d.total_space === 0 || seen.has(id)) continue;
    seen.add(id);
    const used = ((d.total_space - d.available_space) / d.total_space) * 100;
    console.log(`  ${d.mount_point.padEnd(12)} ${used.toFixed(1)}% of ${formatBytes(d.total_space)} (${d.file_system})`);
  }
  console.log("");

  // Network
  console.log(`${colors.bright}Network Traffic:${colors.reset}`);
  for (const n of data.networks) {
    if (n.received_bytes === 0 && n.transmitted_bytes === 0) continue;
    console.log(`  ${colors.magenta}${n.interface.padEnd(10)}${colors.reset} ↓ ${formatBytes(n.received_bytes).padEnd(10)} ↑ ${formatBytes(n.transmitted_bytes)}`);
  }

  console.log(`\n${colors.dim}Refreshing in 2s...${colors.reset}`);
  await new Promise(r => setTimeout(r, 2000));
}
