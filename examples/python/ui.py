"""
UI Helpers for METRICS monitor.
Provides formatting, colors, and display utilities.
"""

import os
from typing import Optional


class Colors:
    """ANSI color codes for terminal output."""
    RESET = "\033[0m"
    BRIGHT = "\033[1m"
    DIM = "\033[2m"
    CYAN = "\033[36m"
    GREEN = "\033[32m"
    YELLOW = "\033[33m"
    MAGENTA = "\033[35m"
    RED = "\033[31m"
    BLUE = "\033[34m"
    WHITE = "\033[37m"

    # Combined styles
    BOLD_CYAN = "\033[1;36m"
    BOLD_GREEN = "\033[1;32m"
    BOLD_YELLOW = "\033[1;33m"
    BOLD_RED = "\033[1;31m"


def format_bytes(bytes_value: int, decimals: int = 2) -> str:
    """
    Format bytes to human-readable string.
    
    Args:
        bytes_value: Number of bytes.
        decimals: Number of decimal places (default: 2).
    
    Returns:
        Formatted string like "1.50 GiB".
    """
    if bytes_value == 0:
        return "0 B"
    
    units = ["B", "KiB", "MiB", "GiB", "TiB", "PiB"]
    k = 1024
    i = 0
    
    while bytes_value >= k and i < len(units) - 1:
        bytes_value /= k
        i += 1
    
    return f"{bytes_value:.{decimals}f} {units[i]}"


def format_uptime(seconds: int) -> str:
    """
    Format seconds to human-readable uptime string.
    
    Args:
        seconds: Uptime in seconds.
    
    Returns:
        Formatted string like "2h 30m 15s".
    """
    hours, remainder = divmod(seconds, 3600)
    minutes, secs = divmod(remainder, 60)
    return f"{hours}h {minutes}m {secs}s"


def create_progress_bar(
    percent: float,
    width: int = 25,
    filled_char: str = "█",
    empty_char: str = "░"
) -> str:
    """
    Create a progress bar.
    
    Args:
        percent: Percentage (0-100).
        width: Width of the bar in characters (default: 25).
        filled_char: Character for filled portion (default: "█").
        empty_char: Character for empty portion (default: "░").
    
    Returns:
        Progress bar string.
    """
    filled = int((percent / 100) * width)
    empty = width - filled
    return filled_char * filled + empty_char * empty


def create_colored_progress_bar(
    percent: float,
    color: str = Colors.GREEN,
    width: int = 25
) -> str:
    """
    Create a colored progress bar.
    
    Args:
        percent: Percentage (0-100).
        color: ANSI color code for the filled portion.
        width: Width of the bar.
    
    Returns:
        Colored progress bar string.
    """
    bar = create_progress_bar(percent, width)
    return f"{color}{bar}{Colors.RESET}"


def clear_screen() -> None:
    """Clear the terminal screen."""
    os.system('cls' if os.name == 'nt' else 'clear')


def pad_right(text: str, length: int, char: str = " ") -> str:
    """
    Pad a string to a specific length on the right.
    
    Args:
        text: String to pad.
        length: Target length.
        char: Padding character (default: space).
    
    Returns:
        Padded string.
    """
    return text.ljust(length, char)


def pad_left(text: str, length: int, char: str = " ") -> str:
    """
    Pad a string to a specific length on the left.
    
    Args:
        text: String to pad.
        length: Target length.
        char: Padding character (default: space).
    
    Returns:
        Padded string.
    """
    return text.rjust(length, char)


def format_percent(value: float, decimals: int = 1) -> str:
    """
    Format a percentage with fixed decimals.
    
    Args:
        value: Percentage value.
        decimals: Number of decimal places.
    
    Returns:
        Formatted percentage string.
    """
    return f"{value:.{decimals}f}%"


def create_header(
    text: str,
    color: str = Colors.CYAN,
    bright: bool = True
) -> str:
    """
    Create a header/title line.
    
    Args:
        text: Header text.
        color: ANSI color code.
        bright: Whether to use bright style.
    
    Returns:
        Formatted header string.
    """
    prefix = Colors.BRIGHT if bright else ""
    return f"{prefix}{color}{text}{Colors.RESET}"


def create_labeled_line(
    label: str,
    value: str,
    color: str = Colors.RESET,
    dim: bool = False
) -> str:
    """
    Create a labeled value line.
    
    Args:
        label: Label text.
        value: Value to display.
        color: ANSI color code for the value.
        dim: Whether to dim the label.
    
    Returns:
        Formatted line string.
    """
    prefix = Colors.DIM if dim else Colors.BRIGHT
    return f"{prefix}{label}{Colors.RESET} {color}{value}{Colors.RESET}"


def format_temperature(temp: float, show_unit: bool = True) -> str:
    """
    Format a temperature value.
    
    Args:
        temp: Temperature in Celsius.
        show_unit: Whether to show the unit (default: True).
    
    Returns:
        Formatted temperature string.
    """
    unit = "°C" if show_unit else ""
    return f"{temp:.1f}{unit}"


def get_color_for_percent(percent: float) -> str:
    """
    Get a color based on a percentage value.
    
    Args:
        percent: Percentage value (0-100).
    
    Returns:
        ANSI color code.
    """
    if percent < 50:
        return Colors.GREEN
    elif percent < 75:
        return Colors.YELLOW
    return Colors.RED


def format_number(num: int) -> str:
    """
    Format a number with thousands separators.
    
    Args:
        num: Number to format.
    
    Returns:
        Formatted number string.
    """
    return f"{num:,}"


def truncate(text: str, max_length: int, suffix: str = "...") -> str:
    """
    Truncate a string if it exceeds a maximum length.
    
    Args:
        text: String to truncate.
        max_length: Maximum length.
        suffix: Suffix to add when truncated (default: "...").
    
    Returns:
        Truncated string.
    """
    if len(text) <= max_length:
        return text
    return text[:max_length - len(suffix)] + suffix


def create_separator(
    char: str = "─",
    length: int = 60,
    color: Optional[str] = None
) -> str:
    """
    Create a separator line.
    
    Args:
        char: Character to use (default: "─").
        length: Length of the line (default: 60).
        color: Optional color for the line.
    
    Returns:
        Separator string.
    """
    line = char * length
    if color:
        return f"{color}{line}{Colors.RESET}"
    return line


def colorize(text: str, color: str) -> str:
    """
    Apply color to text.
    
    Args:
        text: Text to colorize.
        color: ANSI color code.
    
    Returns:
        Colorized text string.
    """
    return f"{color}{text}{Colors.RESET}"
