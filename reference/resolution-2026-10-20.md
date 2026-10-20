# Application cleanup and reporting corrections — 2026-10-20

Implementation through commit 6416381, starting from 6f50539.

## Result

The normal application scan reports **zero findings across all 22 detectors**
on 48 shipped Rust source/example files. The original exclusions remain
`tests/` and `reference/`, which contain deliberate smell fixtures and reference
probes. No finding suppressions, extra exclusions or threshold increases were added.

**262 tests pass, with zero ignored or filtered tests.** This includes the
repository self-scan. Strict all-target Clippy and repository-wide rustfmt pass.
All 66 source-only fixture reports (22 detectors across three corpora) match
the starting revision byte for byte.

## Proven reporting defects fixed

| Defect | Correction and evidence |
| --- | --- |
| Data Clumps combined incompatible explicit parameter types | Compare declared types before grouping; absent annotations remain compatible and Rust lifetimes do not create artificial differences. Regression cases failed before the fix. |
| Receiver wrappers lost type information | Preserve receiver types through parentheses, Rust references and TypeScript non-null wrappers; unknown receivers remain unresolved. |
| Data Class confused field contents with modifiers | Read actual accessibility/static syntax and JavaScript private-property nodes. Public fields containing strings such as `"private"` remain visible; private/static fields do not become public instance data. |
| Rust coupling invented name-only relationships | Require receiver type evidence for member calls, distinguish free from associated functions, and respect lexical typed/local bindings. Tests cover unknown types, shadowing, generic calls and valid calls. |
| Switch Statements treated pure enum strategy selection as branch algorithms | Recognise exhaustive forwarding over an unambiguous project-declared unit enum. Guards, wildcards, incomplete/payload enums, changed arguments and branch computations remain reportable. |
| Shared helpers were called Shotgun Surgery solely because they had callers | Default application reporting now requires Git body co-change evidence; source-only coupling remains an explicit comparison mode. |

Behaviour fixes and structural refactors were committed separately. Existing
assertions were preserved during refactoring; interface callers were migrated.

## Structural cleanup

SourceNode now binds syntax to the source and adapter needed to interpret it.
Type names, methods, constructors, metrics and accessor queries use that bound
input instead of repeatedly passing file/node pairs. Analysis contexts own
lookup depth, owner identity, field sets and related decisions.

Shared declaration modifiers, field enumeration, accessor classification,
module lookup guards, library-extension grouping and duplicate fingerprint
collection have one implementation. Adapter queries own language-specific
field visibility and class/value-type syntax. Rust call layouts are data for a
common name parser. Git reader/process objects own repository selection,
polling and failure cleanup. History changes own revision comparisons; a
small change-query port keeps caller policy in the coupling model.

CLI inventory reporting is a named operation. Test and grouping rationale is
documented on the declarations it explains, and a redundant debug comment was
removed. The Comments rule and its threshold were not weakened.

## Application contract

Normal application analysis uses `Project::detect`. The existing
`bluecheese::detect(files, label)` API retains source-only reference heuristics.

`-s shotgun` now requires a local repository with a committed HEAD. A candidate
must meet the existing static caller/class thresholds and have an existing
callee body change together with existing currently-resolved callers in at
least two other source files. Reports include supporting commit IDs.

The window is at most 128 non-merge commits. Signature migrations, additions,
deletions, comments and formatting do not establish a body co-change. AST leaf
tokens preserve operations and string literal contents. Declaration identities
and source offsets distinguish methods even when they share a line.

Git commands share a 60-second deadline and a 16 MiB per-command output
limit. Timeout handling kills and reaps a blocked child. Git object IDs and
repository paths are validated at the boundary. Missing history, command
failures and parse errors return errors instead of clean reports. Source from
Git is parsed, never checked out, compiled or executed by the detector.

Commands:

```sh
cargo run --locked -- -f . -s shotgun -x tests/ -x reference/
cargo run --locked -- -f . -s shotgun --source-only -x tests/ -x reference/
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo fmt --all -- --check
```

The self-scan now invokes the same application API as the CLI. Its empty-result
assertion remains intact. A separate source-only corpus control retains its
assertion that every detector reaches deliberate fixtures. History tests use
controlled local Git repositories, including Java, Rust and mixed JS/TS cases.

## Remaining limits

Zero findings means no candidates under these rules and this scan scope; it is
not a proof that every possible design defect has been eliminated.

History filtering starts from currently resolved shared callees. It still
misses scattered duplicated policy with no shared callee, and does not
reconstruct the complete historical call graph. Co-change is evidence for
review, not proof of a misplaced responsibility. Rust resolution remains
bounded source analysis rather than compiler-backed import/trait inference.
JavaScript and TypeScript share a resolver; Java and Rust do not acquire
cross-language call edges from matching names. Cross-language structural
clone tests continue to pass.

The historical public-project results in
`reference/shotgun-surgery-examples.md` concern the source-only heuristic. They
were not rerun as a validation set for the new history-backed mode; no public
precision/recall or HIST reproduction claim is made here.

## Visible source-only coupling measurements

The explicit source-only run still produces 10 coupling candidates.
They were not suppressed or removed from that measurement. They have no
qualifying body co-change evidence in the application scan's history window.

| Source location | Owner | Measurement |
| --- | --- | --- |
| src/lang/coupling.rs:28 | MethodLocation | Shotgun Surgery [CM=3, CC=3, Fan-Out=0] |
| src/lang/ecmascript/calls.rs:606 | calls | Shotgun Surgery [CM=2, CC=2, Fan-Out=0] |
| src/lang/java/calls.rs:531 | calls | Shotgun Surgery [CM=5, CC=3, Fan-Out=0] |
| src/lang/java/calls.rs:567 | calls | Shotgun Surgery [CM=2, CC=2, Fan-Out=0] |
| src/lang/java/calls.rs:605 | calls | Shotgun Surgery [CM=2, CC=2, Fan-Out=0] |
| src/lang/mod.rs:52 | SourceFile | Shotgun Surgery [CM=6, CC=5, Fan-Out=0] |
| src/lang/mod.rs:56 | SourceFile | Shotgun Surgery [CM=15, CC=10, Fan-Out=0] |
| src/lang/mod.rs:65 | SourceFile | Shotgun Surgery [CM=13, CC=11, Fan-Out=0] |
| src/lang/structure.rs:287 | structure | Shotgun Surgery [CM=3, CC=2, Fan-Out=1] |
| src/lang/structure.rs:304 | structure | Shotgun Surgery [CM=3, CC=2, Fan-Out=0] |
