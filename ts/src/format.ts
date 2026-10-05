import { toFahrenheit } from "./convert";

export function formatTemp(celsius: number): string {
  return `${celsius.toFixed(0)} C (${toFahrenheit(celsius).toFixed(0)} F)`;
}
