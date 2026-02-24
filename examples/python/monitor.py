#!/usr/bin/env python3
"""
METRICS System Monitor - Python FFI Example
Main entry point that uses the modularized FFI bindings.
"""

import sys
import time
import os
from typing import Set

# Add current directory to path to support direct execution
# This allows 'import ffi' to work when run as 'python3 monitor.py'
current_dir = os.path.dirname(os.path.abspath(__file__))
if current_dir not in sys.path:
    sys.path.append(current_dir)

try:
    from .ffi import MetricsLibrary, MetricsLibraryError, create_metrics_library
    from .types import AllMetrics, BatteryInfo
    from .ui import (
        Colors,
        clear_screen,
        format_bytes,
        format_uptime,
        create_colored_progress_bar,
        pad_right,
        create_header,
        get_color_for_percent,
    )
except (ImportError, ValueError):
    from ffi import MetricsLibrary, MetricsLibraryError, create_metrics_library
    from types import AllMetrics, BatteryInfo
    from ui import (
        Colors,
        clear_screen,
        format_bytes,
        format_uptime,
        create_colored_progress_bar,
        pad_right,
        create_header,
        get_color_for_percent,
    )


# Refresh interval in seconds
REFRESH_INTERVAL = 2


class SystemMonitor:
    """
    System monitor that displays real-time metrics.
    """

    def __init__(self, lib_path: str = None):
        """
        Initialize the system monitor.
        
        Args:
            lib_path: Optional path to the native library.
        """
        try:
            self.lib = create_metrics_library(lib_path)
        except MetricsLibraryError as e:
            print(f"{Colors.RED}Error: {e}{Colors.RESET}")
            sys.exit(1)
        
        self.version = self.lib.get_library_version().version
        self.running = True

    def start(self) -> None:
        """Start the monitoring loop."""
        print(f"{Colors.CYAN}Starting METRICS Monitor v{self.version}{Colors.RESET}")
        
        try:
            while self.running:
                self._render()
                time.sleep(REFRESH_INTERVAL)
        except KeyboardInterrupt:
            self._stop()

    def _stop(self) -> None:
        """Stop the monitor."""
        self.running = False
        self.lib.cleanup_metrics()
        print(f"\n{Colors.YELLOW}Exiting...{Colors.RESET}")
        sys.exit(0)

    def _render(self) -> None:
        """Render the current system state."""
        clear_screen()
        self._render_header()
        
        data = self.lib.get_all_metrics()
        if not data:
            print(f"{Colors.RED}Failed to fetch metrics{Colors.RESET}")
            return

        self._render_os_info(data)
        self._render_cpu(data)
        self._render_memory(data)
        self._render_battery()
        self._render_storage(data)
        self._render_network(data)
        self._render_footer()

    def _render_header(self) -> None:
        """Render the header."""
        print(create_header(f"=== SYSTEM MONITOR PRO v{self.version} (PYTHON FFI) ==="))

    def _render_os_info(self, data: AllMetrics) -> None:
        """Render OS info and uptime."""
        os_info = data.os_info
        load = data.load_avg
        
        print(
            f"{Colors.DIM}{os_info.name} {os_info.os_version} | {os_info.host_name}{Colors.RESET}"
        )
        print(
            f"{Colors.YELLOW}Uptime: {format_uptime(data.uptime)} | "
            f"Load: {load.one_min:.2f} {load.five_min:.2f} {load.fifteen_min:.2f}{Colors.RESET}\n"
        )

    def _render_cpu(self, data: AllMetrics) -> None:
        """Render CPU metrics."""
        cpus = data.cpu
        avg_cpu = sum(c.usage_pct for c in cpus) / len(cpus) if cpus else 0
        color = get_color_for_percent(avg_cpu)
        
        print(
            f"{Colors.BRIGHT}CPU Usage:{Colors.RESET} "
            f"[{create_colored_progress_bar(avg_cpu, color)}] {avg_cpu:.1f}%"
        )
        if cpus:
            print(
                f"{Colors.DIM}  {cpus[0].brand} ({len(cpus)} cores @ {cpus[0].frequency}MHz){Colors.RESET}\n"
            )

    def _render_memory(self, data: AllMetrics) -> None:
        """Render memory metrics."""
        mem = data.memory
        used_pct = (mem.used_bytes / mem.total_bytes) * 100 if mem.total_bytes else 0
        color = get_color_for_percent(used_pct)
        
        print(
            f"{Colors.BRIGHT}RAM Usage:{Colors.RESET} "
            f"[{create_colored_progress_bar(used_pct, color)}] {used_pct:.1f}%"
        )
        print(f"  {format_bytes(mem.used_bytes)} / {format_bytes(mem.total_bytes)}\n")

    def _render_battery(self) -> None:
        """Render battery information."""
        batteries = self.lib.get_battery_info()
        if not batteries:
            return
        
        print(f"{Colors.BRIGHT}Battery Status:{Colors.RESET}")
        for b in batteries:
            print(
                f"  {b['state']}: {b['energy_pct']:.1f}% "
                f"(Health: {b['health_pct']:.1f}%)"
            )
        print()

    def _render_storage(self, data: AllMetrics) -> None:
        """Render storage information."""
        print(f"{Colors.BRIGHT}Storage Devices:{Colors.RESET}")
        seen_disks: Set[str] = set()
        
        for d in data.disks:
            disk_id = f"{d.name}_{d.total_space}"
            if d.total_space == 0 or disk_id in seen_disks:
                continue
            
            seen_disks.add(disk_id)
            usage_pct = ((d.total_space - d.available_space) / d.total_space) * 100
            mount_point = pad_right(d.mount_point, 12)
            
            print(
                f"  {mount_point} {usage_pct:>5.1f}% used of "
                f"{format_bytes(d.total_space)} ({d.file_system})"
            )
        print()

    def _render_network(self, data: AllMetrics) -> None:
        """Render network information."""
        print(f"{Colors.BRIGHT}Network Traffic:{Colors.RESET}")
        
        for n in data.networks:
            if n.received_bytes == 0 and n.transmitted_bytes == 0:
                continue
            
            iface = f"{Colors.MAGENTA}{pad_right(n.interface, 10)}{Colors.RESET}"
            print(
                f"  {iface} ↓ {pad_right(format_bytes(n.received_bytes), 10)} "
                f"↑ {format_bytes(n.transmitted_bytes)}"
            )

    def _render_footer(self) -> None:
        """Render footer."""
        print(f"\n{Colors.DIM}Refreshing... (Ctrl+C to exit){Colors.RESET}")


def main() -> None:
    """Main entry point."""
    try:
        monitor = SystemMonitor()
        monitor.start()
    except MetricsLibraryError as e:
        print(f"{Colors.RED}Fatal error: {e}{Colors.RESET}")
        sys.exit(1)
    except Exception as e:
        print(f"{Colors.RED}Runtime Error: {e}{Colors.RESET}")
        sys.exit(1)


if __name__ == "__main__":
    main()
