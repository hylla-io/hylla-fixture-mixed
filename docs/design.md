# Design

The design has two halves: how input is read and how output is written.

## Input

Input is a temperature typed by a person or read from a file, one per line.

### Parsing

A temperature is a number followed by the unit letter C or F. Upper case only.

#### Whitespace

Leading and trailing spaces are ignored.

## Output

Output follows [the TypeScript formatter](overview.md#typescript). Rounding rules are in
[rounding](overview.md#rounding), a section that does not exist.
