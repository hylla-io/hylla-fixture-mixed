export function toFahrenheit(celsius: number): number {
  return (celsius * 9) / 5 + 32;
}

/** Reads text like "21.5C" or "70F" and returns degrees Celsius. */
export function parseTemp(text: string): number {
  const trimmed = text.trim();
  const unit = trimmed.slice(-1).toUpperCase();
  const value = Number(trimmed.slice(0, -1));
  if (unit === "C") {
    return value;
  }
  if (unit === "F") {
    return ((value - 32) * 5) / 9;
  }
  throw new Error(`unknown unit in ${text}`);
}
