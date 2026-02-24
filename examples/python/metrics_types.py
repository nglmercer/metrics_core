"""
Type definitions for METRICS library.
These types match the Rust structs defined in src/types.rs.
"""

from dataclasses import dataclass
from typing import Optional, List
from enum import IntFlag


@dataclass
class CpuMetrics:
    """CPU metrics for a single core."""
    usage_pct: float
    brand: str
    frequency: int


@dataclass
class MemoryMetrics:
    """System memory metrics."""
    total_bytes: int
    free_bytes: int
    used_bytes: int
    available_bytes: int
    swap_total_bytes: int
    swap_used_bytes: int


@dataclass
class DiskMetrics:
    """Disk/storage metrics."""
    name: str
    total_space: int
    available_space: int
    used_space: int
    mount_point: str
    file_system: str


@dataclass
class NetworkMetrics:
    """Network interface metrics."""
    interface: str
    received_bytes: int
    transmitted_bytes: int
    packets_received: int
    packets_transmitted: int


@dataclass
class OsInfo:
    """Operating system information."""
    name: str
    kernel_version: str
    os_version: str
    host_name: str


@dataclass
class ComponentMetrics:
    """Temperature sensor/component metrics."""
    label: str
    temperature: float
    max: float
    critical: Optional[float]


@dataclass
class ProcessMetrics:
    """Process metrics."""
    pid: int
    name: str
    cpu_usage: float
    memory_bytes: int
    disk_read_bytes: int
    disk_written_bytes: int
    status: str
    user_id: Optional[str]


@dataclass
class ExtendedProcessMetrics:
    """Extended process metrics with additional details."""
    pid: int
    parent_pid: Optional[int]
    name: str
    command: Optional[str]
    cpu_usage: float
    memory_bytes: int
    disk_read_bytes: int
    disk_written_bytes: int
    status: str
    user_id: Optional[str]
    start_time: int


@dataclass
class DiskIoMetrics:
    """Global disk I/O metrics."""
    read_bytes: int
    written_bytes: int


@dataclass
class NetworkIoMetrics:
    """Global network I/O metrics."""
    rx_bytes: int
    tx_bytes: int
    rx_packets: int
    tx_packets: int


@dataclass
class BatteryInfo:
    """Battery information."""
    state: str
    vendor: Optional[str]
    model: Optional[str]
    cycle_count: Optional[int]
    health_pct: float
    energy_pct: float
    energy_full_design_wh: float
    energy_full_wh: float
    energy_wh: float


@dataclass
class LoadAverage:
    """System load average."""
    one_min: float
    five_min: float
    fifteen_min: float


@dataclass
class AllMetrics:
    """All system metrics combined."""
    cpu: List[CpuMetrics]
    memory: MemoryMetrics
    disks: List[DiskMetrics]
    networks: List[NetworkMetrics]
    uptime: int
    os_info: OsInfo
    load_avg: LoadAverage
    batteries: List[BatteryInfo]
    components: List[ComponentMetrics]


@dataclass
class LibraryVersion:
    """Library version information."""
    version: str
    name: str


class RefreshFlags(IntFlag):
    """Refresh flags for the refresh_metrics function."""
    CPU = 1
    MEMORY = 2
    DISKS = 4
    NETWORKS = 8
    PROCESSES = 16
    COMPONENTS = 32
    ALL = 0xffffffff


def dict_to_cpu_metrics(data: dict) -> CpuMetrics:
    """Convert dictionary to CpuMetrics."""
    return CpuMetrics(
        usage_pct=data['usage_pct'],
        brand=data['brand'],
        frequency=data['frequency']
    )


def dict_to_memory_metrics(data: dict) -> MemoryMetrics:
    """Convert dictionary to MemoryMetrics."""
    return MemoryMetrics(
        total_bytes=data['total_bytes'],
        free_bytes=data['free_bytes'],
        used_bytes=data['used_bytes'],
        available_bytes=data['available_bytes'],
        swap_total_bytes=data['swap_total_bytes'],
        swap_used_bytes=data['swap_used_bytes']
    )


def dict_to_disk_metrics(data: dict) -> DiskMetrics:
    """Convert dictionary to DiskMetrics."""
    return DiskMetrics(
        name=data['name'],
        total_space=data['total_space'],
        available_space=data['available_space'],
        used_space=data['used_space'],
        mount_point=data['mount_point'],
        file_system=data['file_system']
    )


def dict_to_network_metrics(data: dict) -> NetworkMetrics:
    """Convert dictionary to NetworkMetrics."""
    return NetworkMetrics(
        interface=data['interface'],
        received_bytes=data['received_bytes'],
        transmitted_bytes=data['transmitted_bytes'],
        packets_received=data['packets_received'],
        packets_transmitted=data['packets_transmitted']
    )


def dict_to_os_info(data: dict) -> OsInfo:
    """Convert dictionary to OsInfo."""
    return OsInfo(
        name=data['name'],
        kernel_version=data['kernel_version'],
        os_version=data['os_version'],
        host_name=data['host_name']
    )


def dict_to_load_average(data: dict) -> LoadAverage:
    """Convert dictionary to LoadAverage."""
    return LoadAverage(
        one_min=data['one_min'],
        five_min=data['five_min'],
        fifteen_min=data['fifteen_min']
    )


def dict_to_battery_info(data: dict) -> BatteryInfo:
    """Convert dictionary to BatteryInfo."""
    return BatteryInfo(
        state=data['state'],
        vendor=data.get('vendor'),
        model=data.get('model'),
        cycle_count=data.get('cycle_count'),
        health_pct=data['health_pct'],
        energy_pct=data['energy_pct'],
        energy_full_design_wh=data['energy_full_design_wh'],
        energy_full_wh=data['energy_full_wh'],
        energy_wh=data['energy_wh']
    )


def dict_to_all_metrics(data: dict) -> AllMetrics:
    """Convert dictionary to AllMetrics."""
    return AllMetrics(
        cpu=[dict_to_cpu_metrics(c) for c in data['cpu']],
        memory=dict_to_memory_metrics(data['memory']),
        disks=[dict_to_disk_metrics(d) for d in data['disks']],
        networks=[dict_to_network_metrics(n) for n in data['networks']],
        uptime=data['uptime'],
        os_info=dict_to_os_info(data['os_info']),
        load_avg=dict_to_load_average(data['load_avg']),
        batteries=[dict_to_battery_info(b) for b in data['batteries']],
        components=[
            ComponentMetrics(
                label=c['label'],
                temperature=c['temperature'],
                max=c['max'],
                critical=c.get('critical')
            ) for c in data['components']
        ]
    )
