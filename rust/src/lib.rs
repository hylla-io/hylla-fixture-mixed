//! Unit conversions for the mixed fixture.

mod distance;
mod units;

pub use distance::km_to_miles;
pub use units::{celsius_to_fahrenheit, fahrenheit_to_celsius};

/// Prints a Celsius temperature in both scales.
pub fn describe_temp(c: f64) -> String {
    format!("{c:.1} C = {:.1} F", celsius_to_fahrenheit(c))
}

/// Labels a count of readings, as in "3 readings".
pub fn count_label(n: u32) -> String {
    let mut digits = itoa::Buffer::new();
    format!("{} readings", digits.format(n))
}
