# Test fixtures

## `smellbench/`

The SmellBench corpus (163 Java files), copied from the `StaticNose` repository
(`datasets/smellbench-dataset/main`). Each case is a small program that either
exhibits a smell (`*BE*`) or is the refactored variant (`*GE*`).

## `golden/smellbench.json`

The report produced by the reference Java tool over `smellbench/`, with the
project path stripped so entries look like `main/java/...`. 515 findings across
the 19 smells the CLI exposes.

Regenerate it with the commands in `../../reference/README.md`, then normalise
the paths (split each `filePath`/`relativeFilePath` on `smellbench-dataset/` and
keep the tail).

## How the parity tests use these

`tests/parity.rs` runs a ported detector over `smellbench/` and compares its
findings to the golden entries for that smell, as a set of
`(relativeFilePath, className, sorted lineNumbers)`. A port is done only when
that difference is empty.
