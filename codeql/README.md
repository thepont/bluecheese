# CodeQL queries

BlueCheese owns the smell definitions. This directory contains CodeQL query
packs for running those rules in GitHub code scanning and focused query tests.

## Current coverage

The current CodeQL port covers **Long Parameter List** and **Long Method** in
Rust, Java, JavaScript, and TypeScript (TypeScript uses CodeQL's JavaScript
extractor). It also covers **Feature Envy** and **Lazy Class** in Java, where
BlueCheese currently supports those rules. Long Parameter List uses the
BlueCheese parameter limits.
The Rust version reports free functions, trait declarations, and inherent
methods, while excluding trait implementations. Long Method uses the same
configured size and complexity thresholds. Its CodeQL metrics are AST-based:
Rust's statement count mirrors the existing adapter; Java and JavaScript use
their language libraries' complexity metrics, so metric-level parity on edge
cases remains to be checked.

**Switch Statements** is also ported for Java switch statements and expressions,
JavaScript/TypeScript switch statements, and Rust match expressions. The Rust
query currently reports every match expression, including forwarding
matches that BlueCheese filters. Java's type-code `if`/`else if` ladders are not
yet represented in the CodeQL rules.

The other BlueCheese detectors have not yet been ported. A CodeQL database
contains one language, so rules that compare code across languages need a
separate multi-database design. Rules that depend on Git history also need
history data to be supplied to the analysis.

## Run query tests

Use the CodeQL bundle that matches the repository's CodeQL query libraries.
From the repository root, run:

```sh
codeql test run \
  codeql/rust-smells-tests \
  codeql/java-smells-tests \
  codeql/javascript-smells-tests
```

The GitHub Actions workflow runs these tests before analyzing Rust, Java, and
JavaScript/TypeScript with all three custom query packs.
