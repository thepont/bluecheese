# Deliberately mixed-language examples

Run from the repository root:

```sh
cargo run -- -f tests/fixtures/multilingual -s shotgun
cargo run -- -f tests/fixtures/multilingual -s dup-code
```

The first command reports the TypeScript `Policy.check`, called by JavaScript
`ClientA` and TypeScript `ClientB`. This is a static coupling signal; a shared
policy is not necessarily misplaced.

The second reports four structurally matching `adjustment` functions across
Java, Rust, JavaScript and TypeScript. Names and declaration syntax differ;
the three operations remain the same. These are deliberate clone candidates,
not independently discovered public-project defects. Numeric overflow and
runtime semantics still differ across the languages.
