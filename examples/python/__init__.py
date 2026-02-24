"""
METRICS Python FFI Package
Provides modularized FFI bindings for the METRICS system monitoring library.
"""

try:
    from .ffi import (
        MetricsLibrary,
        MetricsLibraryError,
        create_metrics_library,
        DEFAULT_LIB_PATH,
        get_lib_extension,
        get_lib_name,
    )

    from .metrics_types import (
        CpuMetrics,
        MemoryMetrics,
        DiskMetrics,
        NetworkMetrics,
        OsInfo,
        ComponentMetrics,
        ProcessMetrics,
        ExtendedProcessMetrics,
        DiskIoMetrics,
        NetworkIoMetrics,
        BatteryInfo,
        LoadAverage,
        AllMetrics,
        LibraryVersion,
        RefreshFlags,
        dict_to_all_metrics,
    )

    from .ui import (
        Colors,
        format_bytes,
        format_uptime,
        create_progress_bar,
        create_colored_progress_bar,
        clear_screen,
        pad_right,
        pad_left,
        format_percent,
        create_header,
        create_labeled_line,
        format_temperature,
        get_color_for_percent,
        format_number,
        truncate,
        create_separator,
        colorize,
    )

    from .monitor import SystemMonitor, main
except (ImportError, ValueError):
    from ffi import (
        MetricsLibrary,
        MetricsLibraryError,
        create_metrics_library,
        DEFAULT_LIB_PATH,
        get_lib_extension,
        get_lib_name,
    )

    from metrics_types import (
        CpuMetrics,
        MemoryMetrics,
        DiskMetrics,
        NetworkMetrics,
        OsInfo,
        ComponentMetrics,
        ProcessMetrics,
        ExtendedProcessMetrics,
        DiskIoMetrics,
        NetworkIoMetrics,
        BatteryInfo,
        LoadAverage,
        AllMetrics,
        LibraryVersion,
        RefreshFlags,
        dict_to_all_metrics,
    )

    from ui import (
        Colors,
        format_bytes,
        format_uptime,
        create_progress_bar,
        create_colored_progress_bar,
        clear_screen,
        pad_right,
        pad_left,
        format_percent,
        create_header,
        create_labeled_line,
        format_temperature,
        get_color_for_percent,
        format_number,
        truncate,
        create_separator,
        colorize,
    )

    from monitor import SystemMonitor, main

__all__ = [
    # FFI
    "MetricsLibrary",
    "MetricsLibraryError",
    "create_metrics_library",
    "DEFAULT_LIB_PATH",
    "get_lib_extension",
    "get_lib_name",
    # Types
    "CpuMetrics",
    "MemoryMetrics",
    "DiskMetrics",
    "NetworkMetrics",
    "OsInfo",
    "ComponentMetrics",
    "ProcessMetrics",
    "ExtendedProcessMetrics",
    "DiskIoMetrics",
    "NetworkIoMetrics",
    "BatteryInfo",
    "LoadAverage",
    "AllMetrics",
    "LibraryVersion",
    "RefreshFlags",
    "dict_to_all_metrics",
    # UI
    "Colors",
    "format_bytes",
    "format_uptime",
    "create_progress_bar",
    "create_colored_progress_bar",
    "clear_screen",
    "pad_right",
    "pad_left",
    "format_percent",
    "create_header",
    "create_labeled_line",
    "format_temperature",
    "get_color_for_percent",
    "format_number",
    "truncate",
    "create_separator",
    "colorize",
    # Monitor
    "SystemMonitor",
    "main",
]

__version__ = "0.1.0"
