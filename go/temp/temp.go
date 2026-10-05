// Package temp converts temperatures.
package temp

import "fmt"

// ToFahrenheit converts Celsius to Fahrenheit.
func ToFahrenheit(c float64) float64 {
	return c*9/5 + 32
}

// Describe prints a Celsius temperature in both scales.
func Describe(c float64) string {
	return fmt.Sprintf("%.1f C = %.1f F", c, ToFahrenheit(c))
}
