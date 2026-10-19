# Test fixtures

## `smellbench/`

The SmellBench corpus (163 Java files), copied from the `StaticNose` repository
(`datasets/smellbench-dataset/main`). Each case is a small program that either
exhibits a smell (`*BE*`) or is the refactored variant (`*GE*`).

## `rust/`

A hand-written Rust fixture for the Rust adapter. There is no reference
implementation for Rust smells, so `tests/rust_adapter.rs` uses these as
characterisation tests rather than parity tests: expectations are reasoned by
hand from the source, and only lock in behaviour.

## `smelly/`

A growing corpus of deliberately smelly fixtures, one concern per file, frozen
once written. It holds both languages: the extension is what picks the adapter,
and a corpus that exercises the partition on genuinely bad code is worth more
than one that does not.

The Java half is held to the Java tool's own report by `tests/parity.rs`. The
Rust half cannot be: there is no reference implementation for Rust smells, so
`tests/smelly_rust.rs` is characterisation, and it says so. Where the two halves
disagree about a shape, they disagree on purpose, and each test names the
difference — a long `fn new` is reported where a long Java constructor is not,
and a function in a nested module is invisible for the same reason a method in a
nested Java type is.

Not every file here is smelly. `Clean.java` and `clean.rs` are controls, and
several others are traps: code that looks reportable and must not be.

## `refactored/`

The after halves of the refactoring pairs whose before halves are in `smelly/`,
sharing each file's name so the pairs read as pairs. Two of the three silence the
detector; the third, Replace Method with Method Object, does not, and both halves
of that claim sit in the corpus rather than in a comment.

## `golden/smellbench.json`

The report produced by the reference Java tool over `smellbench/`, with the
project path stripped so entries look like `main/java/...`. 515 findings across
the 19 smells the CLI exposes.

## `golden/smelly.json` and `golden/refactored.json`

The same capture over `smelly/` and `refactored/`. Both carry every smell the
Java CLI exposes, not only Long Method, so a port that lands later already has
its evidence.

Capture from a *copy* of the corpus, never from the tree in place: the tool
writes its report into the folder it was given, and it runs as root under
Docker, which leaves a `staticnose-report/` directory the repository cannot
delete on its own. The copy also keeps the path normalisation honest: the tool
reports paths relative to the *parent* of the folder it was given, so stripping
the leading component is what makes them look like `EnumLong.java` here.

```sh
rm -rf /tmp/spoon-in && mkdir -p /tmp/spoon-in
cp -r tests/fixtures/smelly tests/fixtures/refactored /tmp/spoon-in/
docker run --rm -v /tmp/StaticNose:/app -v /tmp/spoon-in:/in \
  -w /app maven:3.9-eclipse-temurin-25 \
  java -jar target/StaticNose-1.0.5.jar -f /in/smelly -a -r json
# then, for each finding: filePath = relativeFilePath, with the leading
# `smelly/` or `refactored/` component removed.
```

The Rust fixtures in `smelly/` are ignored by that run, as they are by the Java
tool itself. Their half of the corpus is checked by `tests/smelly_rust.rs`, which
is characterisation rather than parity, because there is no Rust reference.

Regenerate it with the commands in `../../reference/README.md`, then normalise
the paths (split each `filePath`/`relativeFilePath` on `smellbench-dataset/` and
keep the tail).

## How the parity tests use these

`tests/parity.rs` runs a ported detector over `smellbench/` and compares its
findings to the golden entries for that smell, as a set of
`(relativeFilePath, className, sorted lineNumbers)`. A port is done only when
that difference is empty.
