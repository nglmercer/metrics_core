"""
Python FFI Bindings for METRICS Library

This module provides a high-level Python interface to the METRICS library.
It handles library loading, memory management, and type conversions automatically.

Usage:
    from metrics import Metrics
    
    m = Metrics()
    print(m.get_all_metrics())
    m.cleanup()
"""

import ctypes
import json
import os
import sys
from typing import Optional, Dict, Any, List

class MetricsError(Exception):
    """Custom exception for METRICS errors"""
    pass

class LibraryNotFoundError(MetricsError):
    """Raised when the library cannot be found"""
    pass

class Metrics:
    """
    High-level Python wrapper for the METRICS FFI library.
    
    Automatically handles:
    - Library loading
    - Memory management (freeing strings)
    - Cross-platform library naming
    
    Example:
        >>> m = Metrics()
        >>> cpu = m.get_cpu_metrics()
        >>> print(cpu)
        [{'usage_pct': 25.5, 'brand': 'AMD Ryzen 9', 'frequency': 3700}]
        >>> m.cleanup()
    """
    
    # Platform-specific library names
    LIBRARY_NAMES = {
        'linux': 'libmetrics.so',
        'win32': 'metrics.dll',
        'darwin': 'libmetrics.dylib',
    }
    
    # Default search paths
    DEFAULT_SEARCH_PATHS = [
        'target/release',
        'target/debug',
        '.',
        '..',
    ]
    
    def __init__(self, library_path: Optional[str] = None):
        """
        Initialize the Metrics wrapper.
        
        Args:
            library_path: Optional custom path to the library.
                        If not provided, searches in default locations.
        
        Raises:
            LibraryNotFoundError: If library cannot be found.
        """
        self._lib = self._load_library(library_path)
        self._setup_function_signatures()
    
    def _load_library(self, custom_path: Optional[str]) -> ctypes.CDLL:
        """Load the shared library"""
        if custom_path:
            if os.path.exists(custom_path):
                return ctypes.CDLL(custom_path)
            raise LibraryNotFoundError(f"Library not found at: {custom_path}")
        
        # Try to find library in search paths
        lib_name = self.LIBRARY_NAMES.get(sys.platform)
        if not lib_name:
            raise LibraryNotFoundError(f"Unsupported platform: {sys.platform}")
        
        for search_path in self.DEFAULT_SEARCH_PATHS:
            full_path = os.path.join(search_path, lib_name)
            if os.path.exists(full_path):
                return ctypes.CDLL(os.path.abspath(full_path))
        
        raise LibraryNotFoundError(
            f"Library '{lib_name}' not found. "
            f"Please run 'cargo build --release' first."
        )
    
    def _setup_function_signatures(self):
        """Configure C function signatures"""
        # String return types
        void_p = ctypes.c_void_p
        self._lib.get_all_metrics.restype = void_p
        self._lib.get_battery_info.restype = void_p
        self._lib.get_library_version.restype = void_p
        self._lib.get_cpu_metrics.restype = void_p
        self._lib.get_memory_metrics.restype = void_p
        self._lib.get_disk_metrics.restype = void_p
        self._lib.get_network_metrics.restype = void_p
        self._lib.get_os_info.restype = void_p
        self._lib.get_cpu_components.restype = void_p
        self._lib.get_processes.restype = void_p
        self._lib.get_extended_processes.restype = void_p
        self._lib.get_process_by_pid.restype = void_p
        self._lib.get_disk_io.restype = void_p
        self._lib.get_network_io.restype = void_p
        self._lib.get_load_average.restype = void_p
        
        # Numeric return types
        self._lib.get_uptime.restype = ctypes.c_uint64
        
        # Argument types
        self._lib.free_metrics_string.argtypes = [void_p]
        self._lib.get_process_by_pid.argtypes = [ctypes.c_uint32]
        self._lib.refresh_metrics.argtypes = [ctypes.c_uint32]
    
    def _ptr_to_json(self, ptr: ctypes.c_void_p) -> Optional[Dict[str, Any]]:
        """Convert pointer to JSON, automatically freeing the string"""
        if not ptr:
            return None
        try:
            result = ctypes.string_at(ptr).decode('utf-8')
            return json.loads(result)
        finally:
            self._lib.free_metrics_string(ptr)
    
    def _ptr_to_json_list(self, ptr: ctypes.c_void_p) -> Optional[List[Dict[str, Any]]]:
        """Convert pointer to JSON list, automatically freeing the string"""
        return self._ptr_to_json(ptr)
    
    # ============================================================
    # Core Metrics API
    # ============================================================
    
    def get_all_metrics(self) -> Dict[str, Any]:
        """
        Get all system metrics at once.
        
        Returns:
            Dict containing: cpu, memory, disks, networks, uptime,
                          os_info, load_avg, batteries, components
        """
        ptr = self._lib.get_all_metrics()
        return self._ptr_to_json(ptr) or {}
    
    def get_cpu_metrics(self) -> List[Dict[str, Any]]:
        """Get CPU metrics per core"""
        ptr = self._lib.get_cpu_metrics()
        return self._ptr_to_json_list(ptr) or []
    
    def get_memory_metrics(self) -> Dict[str, Any]:
        """Get memory metrics"""
        ptr = self._lib.get_memory_metrics()
        return self._ptr_to_json(ptr) or {}
    
    def get_disk_metrics(self) -> List[Dict[str, Any]]:
        """Get disk/partition metrics"""
        ptr = self._lib.get_disk_metrics()
        return self._ptr_to_json_list(ptr) or []
    
    def get_network_metrics(self) -> List[Dict[str, Any]]:
        """Get network interface metrics"""
        ptr = self._lib.get_network_metrics()
        return self._ptr_to_json_list(ptr) or []
    
    def get_os_info(self) -> Dict[str, Any]:
        """Get OS information"""
        ptr = self._lib.get_os_info()
        return self._ptr_to_json(ptr) or {}
    
    def get_cpu_components(self) -> List[Dict[str, Any]]:
        """Get CPU temperature sensors"""
        ptr = self._lib.get_cpu_components()
        return self._ptr_to_json_list(ptr) or []
    
    def get_processes(self) -> List[Dict[str, Any]]:
        """Get all running processes"""
        ptr = self._lib.get_processes()
        return self._ptr_to_json_list(ptr) or []
    
    def get_extended_processes(self) -> List[Dict[str, Any]]:
        """Get all processes with extended info (parent PID, command, start time)"""
        ptr = self._lib.get_extended_processes()
        return self._ptr_to_json_list(ptr) or []
    
    def get_process_by_pid(self, pid: int) -> Optional[Dict[str, Any]]:
        """Get specific process by PID"""
        ptr = self._lib.get_process_by_pid(ctypes.c_uint32(pid))
        return self._ptr_to_json(ptr)
    
    def get_disk_io(self) -> Dict[str, Any]:
        """Get aggregated disk I/O metrics"""
        ptr = self._lib.get_disk_io()
        return self._ptr_to_json(ptr) or {}
    
    def get_network_io(self) -> Dict[str, Any]:
        """Get aggregated network I/O metrics"""
        ptr = self._lib.get_network_io()
        return self._ptr_to_json(ptr) or {}
    
    def get_battery_info(self) -> List[Dict[str, Any]]:
        """Get battery information"""
        ptr = self._lib.get_battery_info()
        return self._ptr_to_json_list(ptr) or []
    
    def get_load_average(self) -> Dict[str, Any]:
        """Get system load averages"""
        ptr = self._lib.get_load_average()
        return self._ptr_to_json(ptr) or {}
    
    def get_uptime(self) -> int:
        """Get system uptime in seconds"""
        return self._lib.get_uptime()
    
    def get_library_version(self) -> Dict[str, Any]:
        """Get library version"""
        ptr = self._lib.get_library_version()
        return self._ptr_to_json(ptr) or {}
    
    # ============================================================
    # Control API
    # ============================================================
    
    def refresh_metrics(self, flags: int = 0xFFFFFFFF) -> None:
        """
        Refresh internal metric caches.
        
        Args:
            flags: Bitmask for what to refresh
                   1  = CPU
                   2  = Memory
                   4  = Disks
                   8  = Networks
                   16 = Processes
                   32 = Components
                   0xFFFFFFFF = All (default)
        
        Example:
            # Refresh only CPU and memory
            m.refresh_metrics(1 | 2)
        """
        self._lib.refresh_metrics(ctypes.c_uint32(flags))
    
    def cleanup(self) -> None:
        """Clean up internal resources. Call when done."""
        self._lib.cleanup_metrics()
    
    # ============================================================
    # Utility Methods
    # ============================================================
    
    @staticmethod
    def format_bytes(b: int) -> str:
        """Format bytes as human-readable string"""
        units = ['B', 'KiB', 'MiB', 'GiB', 'TiB']
        size = float(b)
        for unit in units:
            if size < 1024:
                return f"{size:.2f} {unit}"
            size /= 1024
        return f"{size:.2f} PiB"
    
    @staticmethod
    def format_uptime(seconds: int) -> str:
        """Format uptime seconds as human-readable string"""
        hours, rem = divmod(seconds, 3600)
        minutes, secs = divmod(rem, 60)
        return f"{hours}h {minutes}m {secs}s"


# Convenience function for simple usage
_default_metrics: Optional[Metrics] = None

def get_metrics() -> Metrics:
    """Get a singleton Metrics instance"""
    global _default_metrics
    if _default_metrics is None:
        _default_metrics = Metrics()
    return _default_metrics

def get_all_metrics() -> Dict[str, Any]:
    """Convenience function to get all metrics"""
    return get_metrics().get_all_metrics()

def cleanup() -> None:
    """Convenience function to cleanup resources"""
    global _default_metrics
    if _default_metrics:
        _default_metrics.cleanup()
        _default_metrics = None


# Refresh flags - use with refresh_metrics()
class RefreshFlags:
    """Refresh flags for selective metric updates"""
    CPU = 1
    MEMORY = 2
    DISKS = 4
    NETWORKS = 8
    PROCESSES = 16
    COMPONENTS = 32
    ALL = 0xFFFFFFFF


if __name__ == "__main__":
    # Example usage of the Metrics library
    try:
        print("Initializing METRICS library...")
        m = Metrics()
        
        # Get library version
        version = m.get_library_version()
        print(f"Library: {version.get('name')} v{version.get('version')}")
        
        # Get uptime
        uptime_seconds = m.get_uptime()
        print(f"Uptime: {m.format_uptime(uptime_seconds)}")
        
        # Refresh all metrics
        print("\nRefreshing metrics...")
        m.refresh_metrics(RefreshFlags.ALL)
        
        # OS Info
        os_info = m.get_os_info()
        print(f"OS: {os_info.get('name')} {os_info.get('os_version')} on {os_info.get('host_name')}")
        
        # CPU
        cpu = m.get_cpu_metrics()
        print(f"CPU cores detected: {len(cpu)}")
        if cpu:
            avg_usage = sum(c.get('usage_pct', 0) for c in cpu) / len(cpu)
            print(f"Average CPU Usage: {avg_usage:.1f}%")
        
        # Memory
        mem = m.get_memory_metrics()
        total_gb = mem.get('total_bytes', 0) / (1024**3)
        used_gb = mem.get('used_bytes', 0) / (1024**3)
        print(f"Memory: {used_gb:.2f} GB / {total_gb:.2f} GB used")
        
        # Load Average
        load = m.get_load_average()
        print(f"Load Average: {load.get('one_min')}, {load.get('five_min')}, {load.get('fifteen_min')}")
        
        # Cleanup
        m.cleanup()
        print("\nCleanup successful.")
        
    except MetricsError as e:
        print(f"Error: {e}")
    except KeyboardInterrupt:
        print("\nInterrupted by user.")
