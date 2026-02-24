/**
 * METRICS System Monitor - Bun FFI Example
 * Main entry point that uses the modularized FFI bindings
 */

import { MetricsLibrary } from "./ffi.js";
import type { AllMetrics, BatteryInfo } from "./types.js";
import {
  colors,
  formatBytes,
  formatUptime,
  createColoredProgressBar,
  clearScreen,
  padRight,
  createHeader,
  createLabeledLine,
  getColorForPercent,
} from "./ui.js";

/**
 * Refresh interval in milliseconds
 */
const REFRESH_INTERVAL = 2000;

/**
 * Main monitor class
 */
class SystemMonitor {
  private lib: MetricsLibrary;
  private version: string;
  private running: boolean = true;

  constructor() {
    this.lib = new MetricsLibrary();
    const versionInfo = this.lib.getLibraryVersion();
    this.version = versionInfo.version;
  }

  /**
   * Start the monitoring loop
   */
  async start(): Promise<void> {
    console.log(`${colors.cyan}Starting METRICS Monitor v${this.version}${colors.reset}`);
    
    // Handle graceful shutdown
    process.on("SIGINT", () => this.stop());
    process.on("SIGTERM", () => this.stop());

    while (this.running) {
      try {
        this.render();
        await this.sleep(REFRESH_INTERVAL);
      } catch (e) {
        console.error(`${colors.red}Error: ${e}${colors.reset}`);
        await this.sleep(REFRESH_INTERVAL);
      }
    }
  }

  /**
   * Stop the monitor
   */
  stop(): void {
    this.running = false;
    this.lib.cleanupMetrics();
    console.log(`\n${colors.yellow}Exiting...${colors.reset}`);
    process.exit(0);
  }

  /**
   * Render the current system state
   */
  private render(): void {
    clearScreen();
    this.renderHeader();
    
    const data = this.lib.getAllMetrics();
    if (!data) {
      console.log(`${colors.red}Failed to fetch metrics${colors.reset}`);
      return;
    }

    this.renderOsInfo(data);
    this.renderCpu(data);
    this.renderMemory(data);
    this.renderBattery();
    this.renderStorage(data);
    this.renderNetwork(data);
    this.renderFooter();
  }

  /**
   * Render the header
   */
  private renderHeader(): void {
    console.log(
      createHeader(`=== METRICS PRO v${this.version} (BUN FFI) ===`, { color: colors.cyan })
    );
  }

  /**
   * Render OS info and uptime
   */
  private renderOsInfo(data: AllMetrics): void {
    const { os_info, load_avg, uptime } = data;
    console.log(
      `${colors.dim}${os_info.name} ${os_info.os_version} | ${os_info.host_name}${colors.reset}`
    );
    console.log(
      `${colors.yellow}Uptime: ${formatUptime(uptime)} | ` +
      `Load: ${load_avg.one_min.toFixed(2)} ${load_avg.five_min.toFixed(2)} ${load_avg.fifteen_min.toFixed(2)}${colors.reset}\n`
    );
  }

  /**
   * Render CPU metrics
   */
  private renderCpu(data: AllMetrics): void {
    const { cpu } = data;
    const avgCpu = cpu.reduce((acc, c) => acc + c.usage_pct, 0) / cpu.length;
    const color = getColorForPercent(avgCpu);
    
    console.log(
      `${colors.bright}CPU usage:${colors.reset} [${createColoredProgressBar(avgCpu, color)}] ${avgCpu.toFixed(1)}%`
    );
    console.log(
      `${colors.dim}  ${cpu[0]?.brand || "N/A"} (${cpu.length} cores @ ${cpu[0]?.frequency}MHz)${colors.reset}\n`
    );
  }

  /**
   * Render memory metrics
   */
  private renderMemory(data: AllMetrics): void {
    const { memory } = data;
    const ramPct = (memory.used_bytes / memory.total_bytes) * 100;
    const color = getColorForPercent(ramPct);
    
    console.log(
      `${colors.bright}RAM usage:${colors.reset} [${createColoredProgressBar(ramPct, color)}] ${ramPct.toFixed(1)}%`
    );
    console.log(
      `  ${formatBytes(memory.used_bytes)} / ${formatBytes(memory.total_bytes)}\n`
    );
  }

  /**
   * Render battery information
   */
  private renderBattery(): void {
    const batteries = this.lib.getBatteryInfo();
    if (!batteries || batteries.length === 0) return;

    console.log(`${colors.bright}Battery:${colors.reset}`);
    for (const b of batteries) {
      console.log(
        `  ${b.state}: ${b.energy_pct.toFixed(1)}% (Health: ${b.health_pct.toFixed(1)}%)`
      );
    }
    console.log("");
  }

  /**
   * Render storage information
   */
  private renderStorage(data: AllMetrics): void {
    console.log(`${colors.bright}Storage:${colors.reset}`);
    const seen = new Set<string>();
    
    for (const d of data.disks) {
      const id = `${d.name}_${d.total_space}`;
      if (d.total_space === 0 || seen.has(id)) continue;
      seen.add(id);
      
      const used = ((d.total_space - d.available_space) / d.total_space) * 100;
      const mountPoint = padRight(d.mount_point, 12);
      console.log(
        `  ${mountPoint} ${used.toFixed(1)}% of ${formatBytes(d.total_space)} (${d.file_system})`
      );
    }
    console.log("");
  }

  /**
   * Render network information
   */
  private renderNetwork(data: AllMetrics): void {
    console.log(`${colors.bright}Network Traffic:${colors.reset}`);
    
    for (const n of data.networks) {
      if (n.received_bytes === 0 && n.transmitted_bytes === 0) continue;
      
      const iface = `${colors.magenta}${padRight(n.interface, 10)}${colors.reset}`;
      console.log(
        `  ${iface} ↓ ${padRight(formatBytes(n.received_bytes), 10)} ↑ ${formatBytes(n.transmitted_bytes)}`
      );
    }
  }

  /**
   * Render footer
   */
  private renderFooter(): void {
    console.log(`\n${colors.dim}Refreshing in ${REFRESH_INTERVAL / 1000}s...${colors.reset}`);
  }

  /**
   * Sleep for a given number of milliseconds
   */
  private sleep(ms: number): Promise<void> {
    return new Promise((resolve) => setTimeout(resolve, ms));
  }
  /**
   * get all metrics
   */
  public getAllMetrics() {
    return this.lib.getAllMetrics();
  }
}

/**
 * Main entry point
 */
async function main() {
  try {
    const monitor = new SystemMonitor();
    await monitor.start();
  } catch (e) {
    console.error(`${colors.red}Fatal error: ${e}${colors.reset}`);
    process.exit(1);
  }
}

// Run the monitor
main();

export { SystemMonitor };