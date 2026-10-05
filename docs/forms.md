# Link forms

One instance of each link form the doc-link pass reads. This file never changes after the first
commit.

## Reference links

The [output rules][out] use a full reference link, the [Glossary][] a collapsed one, and the
[notes] a shortcut one.

[out]: design.md#output
[glossary]: glossary.md
[NOTES]: notes.md

## Code locations

The parser is [lines 6-17 of convert.ts](../ts/src/convert.ts#L6-L17). The Go converter is
`../go/temp/temp.go:7-9`. The Python code is [the mixed_stats folder](../python/mixed_stats/).

## Code names

`parseTemp()` reads a temperature. `units::celsius_to_fahrenheit` is the Rust converter.

## Blocks

```
celsius_to_fahrenheit(21.5)
```

```pseudo
formatTemp(21.5)
```

## Outside

[Another fixture](../../hylla-fixture-docs/README.md) sits outside this repository.
