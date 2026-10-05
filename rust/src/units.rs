/// Converts Celsius to Fahrenheit.
pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

/// Converts kilometres to miles.
pub fn km_to_miles(km: f64) -> f64 {
    km * 0.621371
}
