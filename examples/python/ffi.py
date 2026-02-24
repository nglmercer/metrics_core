"""
FFI bindings for METRICS library.
Handles library loading and function bindings using ctypes.
"""

import ctypes
import json
import os
import sys
from typing import Optional, TypeVar, Callable

try:
    from .types import (
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
        RefreshFlags,
        dict_to_all_metrics,
        dict_to_battery_info,
    )
except (ImportError, ValueError):
    from types import (
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
        RefreshFlags,
        dict_to_all_metrics,
        dict_to_battery_info,
    )


T = TypeVar('T')


def get_lib_extension() -> str:
    """Get the platform-specific library extension."""
    if sys.platform == "win32":
        return "dll"
    elif sys.platform == "darwin":
        return "dylib"
    else:
        return "so"


def get_lib_name() -> str:
    """Get the platform-specific library name."""
    ext = get_lib_extension()
    if sys.platform == "win32":
        return f"metrics.{ext}"
    return f"libmetrics.{ext}"


def find_library() -> str:
    """Search for the native library in common locations."""
    lib_name = get_lib_name()
    search_paths = [
        # Relative to project root
        os.path.join("target", "release"),
        os.path.join("target", "debug"),
        # Relative to this file
        os.path.join(os.path.dirname(__file__), "..", "..", "target", "release"),
        os.path.join(os.path.dirname(__file__), "..", "..", "target", "debug"),
        # Current directory
        ".",
    ]
    
    for path in search_paths:
        full_path = os.path.abspath(os.path.join(path, lib_name))
        if os.path.exists(full_path):
            return full_path
            
    # Default fallback
    return os.path.abspath(os.path.join("target", "release", lib_name))


DEFAULT_LIB_PATH = find_library()


class MetricsLibraryError(Exception):
    """Exception raised for library loading or operation errors."""
    pass


class MetricsLibrary:
    """
    Wrapper class for the METRICS native library.
    Provides type-safe methods for calling FFI functions.
    """

    def __init__(self, lib_path: str = DEFAULT_LIB_PATH):
        """
        Initialize the library wrapper.
        
        Args:
            lib_path: Path to the native library.
        
        Raises:
            MetricsLibraryError: If the library cannot be loaded.
        """
        self._path = lib_path
        self._lib: Optional[ctypes.CDLL] = None
        self._load_library()

    def _load_library(self) -> None:
        """Load the native library and configure function signatures."""
        try:
            self._lib = ctypes.CDLL(self._path)
        except OSError as e:
            raise MetricsLibraryError(
                f"Failed to load library at {self._path}. "
                f"Make sure to run 'cargo build --release' first. Error: {e}"
            )
        
        self._configure_functions()

    def _configure_functions(self) -> None:
        """Configure return and argument types for FFI functions."""
        if not self._lib:
            return

        # String-returning functions (return pointer)
        self._lib.get_library_version.restype = ctypes.c_void_p
        self._lib.get_all_metrics.restype = ctypes.c_void_p
        self._lib.get_cpu_metrics.restype = ctypes.c_void_p
        self._lib.get_memory_metrics.restype = ctypes.c_void_p
        self._lib.get_disk_metrics.restype = ctypes.c_void_p
        self._lib.get_network_metrics.restype = ctypes.c_void_p
        self._lib.get_load_average.restype = ctypes.c_void_p
        self._lib.get_os_info.restype = ctypes.c_void_p
        self._lib.get_cpu_components.restype = ctypes.c_void_p
        self._lib.get_processes.restype = ctypes.c_void_p
        self._lib.get_extended_processes.restype = ctypes.c_void_p
        self._lib.get_process_by_pid.restype = ctypes.c_void_p
        self._lib.get_process_by_pid.argtypes = [ctypes.c_uint32]
        self._lib.get_disk_io.restype = ctypes.c_void_p
        self._lib.get_network_io.restype = ctypes.c_void_p
        self._lib.get_battery_info.restype = ctypes.c_void_p

        # Numeric-returning functions
        self._lib.get_uptime.restype = ctypes.c_uint64

        # Functions with arguments
        self._lib.free_metrics_string.argtypes = [ctypes.c_void_p]
        self._lib.refresh_metrics.argtypes = [ctypes.c_uint32]

    @property
    def library_path(self) -> str:
        """Get the library path."""
        return self._path

    def _ptr_to_json(self, ptr: int) -> Optional[dict]:
        """
        Convert a pointer to a JSON object and free the string.
        
        Args:
            ptr: Pointer to a C string.
        
        Returns:
            Parsed JSON object or None if pointer is null.
        """
        if not ptr:
            return None
        
        try:
            json_str = ctypes.string_at(ptr).decode('utf-8')
            self._lib.free_metrics_string(ptr)
            return json.loads(json_str)
        except Exception:
            if ptr:
                self._lib.free_metrics_string(ptr)
            return None

    def _ptr_to_list(self, ptr: int) -> Optional[list]:
        """Convert pointer to a JSON list."""
        return self._ptr_to_json(ptr)

    def get_library_version(self) -> LibraryVersion:
        """Get the library version."""
        ptr = self._lib.get_library_version()
        data = self._ptr_to_json(ptr)
        if data:
            return LibraryVersion(version=data['version'], name=data['name'])
        return LibraryVersion(version="unknown", name="METRICS")

    def get_all_metrics(self) -> Optional[AllMetrics]:
        """Get all metrics at once."""
        ptr = self._lib.get_all_metrics()
        data = self._ptr_to_json(ptr)
        if data:
            return dict_to_all_metrics(data)
        return None

    def get_cpu_metrics(self) -> Optional[list]:
        """Get CPU metrics."""
        ptr = self._lib.get_cpu_metrics()
        return self._ptr_to_list(ptr)

    def get_memory_metrics(self) -> Optional[dict]:
        """Get memory metrics."""
        ptr = self._lib.get_memory_metrics()
        return self._ptr_to_json(ptr)

    def get_disk_metrics(self) -> Optional[list]:
        """Get disk metrics."""
        ptr = self._lib.get_disk_metrics()
        return self._ptr_to_list(ptr)

    def get_network_metrics(self) -> Optional[list]:
        """Get network metrics."""
        ptr = self._lib.get_network_metrics()
        return self._ptr_to_list(ptr)

    def get_uptime(self) -> int:
        """Get system uptime in seconds."""
        return self._lib.get_uptime()

    def get_load_average(self) -> Optional[dict]:
        """Get load average."""
        ptr = self._lib.get_load_average()
        return self._ptr_to_json(ptr)

    def get_os_info(self) -> Optional[dict]:
        """Get OS information."""
        ptr = self._lib.get_os_info()
        return self._ptr_to_json(ptr)

    def get_cpu_components(self) -> Optional[list]:
        """Get CPU components (temperature sensors)."""
        ptr = self._lib.get_cpu_components()
        return self._ptr_to_list(ptr)

    def get_processes(self) -> Optional[list]:
        """Get process list."""
        ptr = self._lib.get_processes()
        return self._ptr_to_list(ptr)

    def get_extended_processes(self) -> Optional[list]:
        """Get extended process list with parent PID and command line."""
        ptr = self._lib.get_extended_processes()
        return self._ptr_to_list(ptr)

    def get_process_by_pid(self, pid: int) -> Optional[dict]:
        """Get process by PID."""
        ptr = self._lib.get_process_by_pid(pid)
        return self._ptr_to_json(ptr)

    def get_disk_io(self) -> Optional[dict]:
        """Get disk I/O metrics."""
        ptr = self._lib.get_disk_io()
        return self._ptr_to_json(ptr)

    def get_network_io(self) -> Optional[dict]:
        """Get network I/O metrics."""
        ptr = self._lib.get_network_io()
        return self._ptr_to_json(ptr)

    def get_battery_info(self) -> Optional[list]:
        """Get battery information."""
        ptr = self._lib.get_battery_info()
        return self._ptr_to_list(ptr)

    def refresh_metrics(self, flags: int = RefreshFlags.ALL) -> None:
        """Refresh internal metric caches."""
        self._lib.refresh_metrics(flags)

    def cleanup_metrics(self) -> None:
        """Cleanup internal resources."""
        self._lib.cleanup_metrics()


def create_metrics_library(lib_path: Optional[str] = None) -> MetricsLibrary:
    """
    Create a new MetricsLibrary instance.
    
    Args:
        lib_path: Optional path to the library.
    
    Returns:
        MetricsLibrary instance.
    """
    return MetricsLibrary(lib_path or DEFAULT_LIB_PATH)