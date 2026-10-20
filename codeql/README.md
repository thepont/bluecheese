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

**Comments** is ported for JavaScript/TypeScript. It reports non-documentation
comments with more than five words or a `TODO` marker; Java and Rust comments
are not included.

**Switch Statements** is also ported for Java switch statements and expressions,
JavaScript/TypeScript switch statements, and Rust match expressions. The Rust
query currently reports every match expression, including forwarding
matches that BlueCheese filters. Java's type-code `if`/`else if` ladders are not
yet represented in the CodeQL rules.

**Primitive Obsession** is ported for Java classes, TypeScript classes, and
Rust structs: a type is reported when it has at least four explicitly typed
primitive fields. As in BlueCheese, untyped JavaScript fields are not inferred.

**Message Chains** is ported for getter-style calls in Java,
JavaScript/TypeScript, and Rust. Java recognizes `get*` and `is*` calls;
JavaScript/TypeScript also recognizes
`has*`. The Rust port uses explicit method-call syntax and does not infer
chains through overloaded operators.

**Speculative Generality** has a Java query for abstract classes with no more
than one resolved source type reference. It currently omits Java interfaces,
Rust traits, and TypeScript interfaces; the repository heuristic counts
name-based references in a narrower package/file scope.

**Data Clumps** has a Java query for methods in one class that repeat at least
two parameter names with matching types. Repeated call-site argument groups
and the Rust/JavaScript/TypeScript variants are not included yet.

**Data Class** has Java, JavaScript/TypeScript, and Rust queries for types that
expose public instance fields. These queries do not yet detect accessor-only
data classes.

**Inappropriate Intimacy** has a Java query for direct field reads through a
different source-defined class. The JavaScript/TypeScript and Rust heuristics
are not represented yet.

**Middle Man** has a Java query for classes where at least half of the methods
are single-statement delegations through a field to another source-defined
class. The Rust and JavaScript/TypeScript variants are not included yet.

**Refused Bequest** has a Java query for explicit `@Override` methods that
replace inherited behavior with an empty `void` body or an
`UnsupportedOperation` exception. It does not yet cover implicit refusal
through unimplemented parent methods, JavaScript/TypeScript, or Rust trait
stubs.

**Large Class** has a Java query for classes with at least eight instance
fields. The source detector's weighted method complexity and tight class
cohesion branch is not included in this query.

**Parallel Inheritance Hierarchies** has a Java query for same-file subclasses
under separate source-defined parents that have matching three-character name
prefixes. CodeQL resolves parent types project-wide, while this query's
name-pairing is a simpler approximation of the source detector's greedy
matching heuristic.

**Shotgun Surgery** has a Java query for methods called by at least two
distinct methods from at least two other classes. Rust and JavaScript/TypeScript
are not included yet.

**Divergent Change** has a Java query for classes whose switch statements or
expressions occur across at least three methods. Its repeated-action and
low-cohesion branches are not included yet.

**Temporary Field** has a Java query for uninitialized instance fields checked
against `null` and accessed inside that `if` branch. It does not yet cover the
primitive-field workflow and constructor-frequency heuristics.

**Alternative Classes with Different Interfaces** has a Java query for same-file
classes with at least two methods whose statement and expression node-class
counts match. It ignores identifier and literal values, but does not preserve
the exact AST ordering and nesting used by the source detector.

**Duplicate Code** has a Java query for same-class methods with matching
statement and expression node-class counts and at least three statements. It
does not preserve AST ordering or nesting, and does not cover cross-language
duplicates.

**Incomplete Library Class** has a Java query for same-file static helper
methods that take the same imported type, call a method on that parameter, and
have matching statement and expression node-class counts. It does not cover the
Rust, JavaScript, or TypeScript workflows in the source detector.

Several other ports above cover only a subset of the source detector's
heuristics or languages. A CodeQL database contains one language, so rules
that compare code across languages need a separate multi-database design.
Rules that depend on Git history also need history data to be supplied to the
analysis.

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
