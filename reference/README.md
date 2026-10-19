# Reference: the StaticNose Java tool

The Rust detectors in this repository are ports of the Java detectors in
[`refactoring-assistant/StaticNose`](https://github.com/refactoring-assistant/StaticNose)
(the tool described in Yash Mahesh Burshe's MS thesis, *Detecting Fowler's Code
Smells using Static Analysis*, Northeastern University, 2026).

The Java tool is the **reference implementation**. Every ported detector is held
to its output by `tests/parity.rs`.

## Building the reference

Requires Docker (the tool needs JDK 25; the Maven image provides it):

```sh
git clone https://github.com/refactoring-assistant/StaticNose /tmp/StaticNose
cd /tmp/StaticNose
docker run --rm -v "$PWD:/app" -w /app maven:3.9-eclipse-temurin-25 \
  mvn -B -DskipTests package
# -> target/StaticNose-1.0.5.jar
```

Produce the golden report for the SmellBench corpus:

```sh
docker run --rm -v /tmp/StaticNose:/app -w /app maven:3.9-eclipse-temurin-25 \
  java -jar target/StaticNose-1.0.5.jar \
  -f /app/datasets/smellbench-dataset -a -r json
# -> datasets/smellbench-dataset/staticnose-report/report-<timestamp>.json
```

## Shotgun Surgery port

`bluecheese -f <source> -s shotgun` implements StaticNose's default static
coupling rule for Java: at least two distinct calling methods in at least two
external classes; interface-contract methods are excluded. It reports the
callee's declaration and CM, CC and Fan-Out metrics. This is a source snapshot
analysis, with no Git-history input and no Rust call-resolution support.

Reference inspected and built: StaticNose
`de3c2ef15c2c9fc6973eb78280e77e1ea30b5be5`,
`src/main/java/edu/northeastern/smells/ShotgunSurgeryDetector.java`.
The Java adapter owns source-only call resolution. It handles project types,
imports, fields, scoped locals, typed overloads, inheritance and nested named
classes; ambiguous targets are omitted. It is not Spoon or a complete Java
type checker: classpath interfaces, generic inference, arrays/varargs, static
imports and anonymous classes are not fully resolved. Default thresholds are
ported; the Java tool's configurable thresholds are not exposed here.

All 38 SmellBench findings and metrics match the saved and freshly regenerated
reference report. That does not imply full-project equivalence or diagnostic
accuracy. See [the public-example notebook](shotgun-surgery-examples.md) for
the broader comparison, known disagreements and false-positive controls.

## JavaScript and TypeScript

The Rust CLI also parses JavaScript (`.js`, `.jsx`, `.mjs`, `.cjs`), TypeScript
(`.ts`, `.mts`, `.cts`) and TSX (`.tsx`). For example:

```sh
bluecheese -f ./src -s shotgun
```

One scan can include Java, Rust, JavaScript and TypeScript files. The shotgun
adapter resolves project-local relative ES imports across JS/TS, including
named aliases, default imports and namespace imports. It reads class methods,
named object-literal methods, named functions and assigned arrow functions.
Receivers can come from `new`, TypeScript annotations, field initializers or
direct constructor field initialization. Interface-contract methods are
excluded. Calling classes, object owners and source modules count as separate
owners; multiple functions in one module alone do not meet the two-owner rule.

These are characterisation tests, not parity against a JS/TS reference tool.
Long Method and Long Parameter List also use the syntax adapters. The other
existing language-specific detectors retain their documented scope.

This resolver does not run source code or load npm packages. It does not
resolve arbitrary `require()`, dynamic imports, re-export chains, tsconfig
path aliases, full TypeScript overload/generic inference or runtime mutation.
Unknown receivers and ambiguous symbols are omitted. The heuristic still
measures static coupling, not whether a shared responsibility is misplaced.

## Debug probes

`java-probes/` contains two small probes used to reverse-engineer the exact
semantics of Spoon's metrics. They are not part of the shipped tool; they exist
so that the goldens can be regenerated and cross-checked when the port needs to
answer "what does Spoon actually do here?".

- `Probe.java` — dumps per-method `declLine`, `bodyLine`, LLOC and cyclomatic
  complexity as TSV, matching the shape of `examples/dump_metrics.rs`.
  Pass a second argument `comments` to force `setCommentEnabled(true)`;
  pass nothing to use Spoon's default (**comments are enabled by default**).
- `ProbeStatements.java` — dumps, for a file (optionally filtered to one method
  name), the class name of every `CtStatement` Spoon counts for LLOC, plus its
  LLOC and complexity.

Copy them into `src/main/java/edu/northeastern/`, rebuild, and run them from the
shaded jar:

```sh
java -cp target/StaticNose-1.0.5.jar edu.northeastern.Probe /path/to/project
java -cp target/StaticNose-1.0.5.jar edu.northeastern.ProbeStatements \
  /path/to/File.java someMethodName
```

## Semantics discovered while porting

These are non-obvious behaviours of Spoon 11.2.2-beta-10 that the port must
reproduce. Each was confirmed with the probes above, not assumed.

- **Comments are enabled by default.** `AbstractDetector` never calls
  `setCommentEnabled`, and Spoon's default includes comments in the model.
  `CtComment` is a `CtStatement`, so comments count towards LLOC.
- **LLOC counts expressions as statements.** `CtInvocation`,
  `CtConstructorCall`, `CtAssignment` (including `+=`) and `CtUnaryOperator`
  (`!x`, `i++`) are all `CtStatement`s. Nested invocations count individually,
  so `a.b().c()` is two statements.
- **There is no expression-statement wrapper**, so the *inner* expression is
  counted rather than an enclosing node.
- **`CtCatch` is not a `CtStatement`**, so `catch` blocks do not add to LLOC.
- **For-each loops create an implicit `CtLocalVariable`** for the loop variable,
  which counts towards LLOC; try-with-resources resources do too.
- **`CtModel.getAllTypes()` returns top-level types only.** Methods of nested
  types are never returned, so nested types never produce findings.
- **Annotation lines are excluded from a declaration's position**, so a method
  annotated with `@Override` on the previous line reports the line of its first
  modifier or return type.
- **The CLI exposes 19 smells.** `DEAD_CODE`, `DUPLICATE_CODE` and
  `SPECULATIVE_GENERALITY` are commented out in `CodeSmell.java`.
