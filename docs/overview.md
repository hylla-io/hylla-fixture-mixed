# Overview

Each language owns one small piece. Code names in backticks are resolved through the language
servers.

## Rust

[units.rs](../rust/src/units.rs) converts units. `celsius_to_fahrenheit` converts Celsius to
Fahrenheit by multiplying by 9/5 and adding 32. `describe_temp` prints both scales.
`count_label` writes its number with `itoa::Buffer`, from the one third-party crate.

```rs
let f = celsius_to_fahrenheit(21.5);
let line = describe_temp(f);
```

## TypeScript

`formatTemp` in [format.ts](../ts/src/format.ts) prints a temperature with one decimal place, for
example 21.5 C.

## Python

The statistics helpers are described in [stats](stats.md#mean).

```py
kept = drop_outliers([1.0, 2.0, 50.0], 10.0)
```

## Go

`temp.ToFahrenheit` converts Celsius to Fahrenheit. `temp.Describe` prints one line.

## Standard library

`std::cmp::max`, `statistics.median` and `Math.round` are outside this repo.
