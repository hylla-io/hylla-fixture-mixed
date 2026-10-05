import { toFahrenheit } from "./convert";

export function formatTemp(celsius: number): string {
  return `${celsius.toFixed(1)} C (${toFahrenheit(celsius).toFixed(1)} F)`;
}
