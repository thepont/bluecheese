# CodeQL queries

BlueCheese owns the smell definitions. This directory contains CodeQL query
packs for running those rules in GitHub code scanning and focused query tests.

## Current coverage

The current CodeQL port covers **Long Parameter List** in Rust, Java, JavaScript,
and TypeScript (TypeScript uses CodeQL's JavaScript extractor). The Java and
JavaScript/TypeScript versions use the same method threshold as BlueCheese;
constructors have the Java-specific threshold. The Rust version reports free
functions, trait declarations, and inherent methods, while excluding trait
implementations.

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
