/**
 * UI Helpers for METRICS monitor
 * Provides formatting, colors, and display utilities
 */

/**
 * ANSI color codes for terminal output
 */
export const colors = {
  reset: "\x1b[0m",
  bright: "\x1b[1m",
  dim: "\x1b[2m",
  cyan: "\x1b[36m",
  green: "\x1b[32m",
  yellow: "\x1b[33m",
  magenta: "\x1b[35m",
  red: "\x1b[31m",
  blue: "\x1b[34m",
  white: "\x1b[37m",
} as const;

/**
 * Format bytes to human-readable string
 * @param bytes - Number of bytes
 * @param decimals - Number of decimal places (default: 2)
 * @returns Formatted string like "1.50 GiB"
 */
export function formatBytes(bytes: number, decimals: number = 2): string {
  if (bytes === 0) return "0 B";
  
  const units = ["B", "KiB", "MiB", "GiB", "TiB", "PiB"];
  const k = 1024;
  const dm = decimals < 0 ? 0 : decimals;
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  
  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(dm))} ${units[i]}`;
}

/**
 * Format seconds to human-readable uptime string
 * @param seconds - Uptime in seconds
 * @returns Formatted string like "2h 30m 15s"
 */
export function formatUptime(seconds: number): string {
  const h = Math.floor(seconds / 3600);
  const m = Math.floor((seconds % 3600) / 60);
  const s = seconds % 60;
  return `${h}h ${m}m ${s}s`;
}

/**
 * Create a progress bar
 * @param percent - Percentage (0-100)
 * @param width - Width of the bar in characters (default: 25)
 * @param filledChar - Character for filled portion (default: "█")
 * @param emptyChar - Character for empty portion (default: "░")
 * @returns Progress bar string
 */
export function createProgressBar(
  percent: number,
  width: number = 25,
  filledChar: string = "█",
  emptyChar: string = "░"
): string {
  const filled = Math.round((percent / 100) * width);
  const empty = width - filled;
  return filledChar.repeat(filled) + emptyChar.repeat(empty);
}

/**
 * Clear the terminal screen
 */
export function clearScreen(): void {
  process.stdout.write("\x1Bc");
}

/**
 * Create a colored progress bar
 * @param percent - Percentage (0-100)
 * @param color - ANSI color code for the filled portion
 * @param width - Width of the bar
 * @returns Colored progress bar string
 */
export function createColoredProgressBar(
  percent: number,
  color: string = colors.green,
  width: number = 25
): string {
  const bar = createProgressBar(percent, width);
  return `${color}${bar}${colors.reset}`;
}

/**
 * Pad a string to a specific length
 * @param str - String to pad
 * @param length - Target length
 * @param char - Padding character (default: space)
 * @returns Padded string
 */
export function padRight(str: string, length: number, char: string = " "): string {
  return str.padEnd(length, char);
}

/**
 * Format a percentage with fixed decimals
 * @param value - Percentage value
 * @param decimals - Number of decimal places
 * @returns Formatted percentage string
 */
export function formatPercent(value: number, decimals: number = 1): string {
  return `${value.toFixed(decimals)}%`;
}

/**
 * Create a header/title line
 * @param text - Header text
 * @param style - Style object with color and formatting
 * @returns Formatted header string
 */
export function createHeader(
  text: string,
  style: { color?: string; bright?: boolean } = {}
): string {
  const { color = colors.cyan, bright = true } = style;
  const prefix = bright ? colors.bright : "";
  return `${prefix}${color}${text}${colors.reset}`;
}

/**
 * Create a labeled value line
 * @param label - Label text
 * @param value - Value to display
 * @param options - Display options
 * @returns Formatted line string
 */
export function createLabeledLine(
  label: string,
  value: string,
  options: { color?: string; dim?: boolean } = {}
): string {
  const { color = colors.reset, dim = false } = options;
  const prefix = dim ? colors.dim : colors.bright;
  return `${prefix}${label}${colors.reset} ${color}${value}${colors.reset}`;
}

/**
 * Format a temperature value
 * @param temp - Temperature in Celsius
 * @param showUnit - Whether to show the unit (default: true)
 * @returns Formatted temperature string
 */
export function formatTemperature(temp: number, showUnit: boolean = true): string {
  const unit = showUnit ? "°C" : "";
  return `${temp.toFixed(1)}${unit}`;
}

/**
 * Get a color based on a percentage value
 * @param percent - Percentage value (0-100)
 * @returns ANSI color code
 */
export function getColorForPercent(percent: number): string {
  if (percent < 50) return colors.green;
  if (percent < 75) return colors.yellow;
  return colors.red;
}

/**
 * Format a number with thousands separators
 * @param num - Number to format
 * @returns Formatted number string
 */
export function formatNumber(num: number): string {
  return num.toLocaleString();
}

/**
 * Truncate a string if it exceeds a maximum length
 * @param str - String to truncate
 * @param maxLength - Maximum length
 * @param suffix - Suffix to add when truncated (default: "...")
 * @returns Truncated string
 */
export function truncate(str: string, maxLength: number, suffix: string = "..."): string {
  if (str.length <= maxLength) return str;
  return str.slice(0, maxLength - suffix.length) + suffix;
}

/**
 * Create a separator line
 * @param char - Character to use (default: "─")
 * @param length - Length of the line (default: 60)
 * @param color - Optional color for the line
 * @returns Separator string
 */
export function createSeparator(
  char: string = "─",
  length: number = 60,
  color?: string
): string {
  const line = char.repeat(length);
  return color ? `${color}${line}${colors.reset}` : line;
}