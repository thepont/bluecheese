# Source evidence for disconnected membership declarations

The earlier incoming-coupling prerequisite missed the user's tenant example:
separate declarations repeat a catalogue, and adding a tenant requires editing
every copy even though none calls another. The source detector now recognises
this pattern without Git history.

## Implemented behaviour

`bluecheese -f PROJECT -s shotgun --source-only` groups declarations by name,
explicit domain and complete static membership. It reports one finding per
declaration, including every matching file and line. At least two distinct
members and two files are needed to establish a parallel membership set.

Supported declarations include simple Java/Rust/TypeScript enums and named
Java array initializers, Rust const/static arrays, and JavaScript/TypeScript
literal arrays (including TypeScript `as const`). Explicit string-valued
TypeScript enums use their encoded values. Member order is immaterial; string
contents and case remain significant. Catalogue identifiers ignore case and
separators; a trailing plural `s` on list names allows `Tenant`, `TENANTS` and
`tenants` to be compared. This is a naming convention, not linguistic analysis.

The application default tries this check first. When it finds evidence, it
returns source candidates without evaluating history. Otherwise it requires
the existing bounded Git body-co-change analysis. The two paths are alternatives,
not an exhaustive union. `--reference` retains the original incoming-coupling
metric and the existing public `bluecheese::detect` function remains unchanged.
`--reference` and `--source-only` are mutually exclusive.

## Legitimate reasons and conservative boundaries

- Separate domains may independently have the same states. Explicit Java
  packages, Rust modules and TypeScript namespaces keep these apart.
- Frozen protocol/schema versions may intentionally preserve copies. Explicit
  version namespaces keep these apart; directory names alone do not establish
  independence.
- Generated copies have one authoring authority. Leading actual comments with
  `@generated` or both `generated` and `do not edit` identify generated files;
  those declarations do not count as separate manual authorities. A marker in
  a string does not establish provenance.
- Consumers that reference an existing enum's members do not redeclare its
  membership. Those references do not count as literal copies.

Matching identity and membership are evidence for review, not semantic proof.
Same-named independent concepts without explicit domain boundaries can still
produce candidates. Different names, partial overlaps and already-drifted copies
can be missed. The detector does not infer a shared business concept from
arbitrary repeated strings. Dynamic lists, spreads, sparse arrays, destructured
bindings, enum payloads/discriminants and strings requiring escape/raw-string
decoding are outside this initial check. Parse errors are explicit errors.

## A second pattern: one enum, many handling sites

The user also identified a single enum whose new members require edits to
multiple consumers. This is distinct from duplicate declarations and is **not
implemented by this check**. Detection needs resolved enum references and a map
of switch/match/conditional handlers, including default/wildcard behaviour.
That map can expose change obligations from source alone. Exhaustive matches
are useful compiler enforcement and may implement legitimately different
behaviours; repeated policy/translation across handlers is stronger smell
evidence. Git co-change can validate actual maintenance impact when source
cannot establish that common responsibility.

The existing history fallback still starts from shared callees. It does not yet
discover arbitrary disconnected co-change or enum-consumer change obligations.

## Verification

The initial regression suite was observed red: four failing tests and six
passing controls. The expanded suite has 13 passing source-evidence tests,
covering Java/Rust/JS/TS, mixed languages, default execution without Git,
generated provenance, explicit domains/versions, encoded values, malformed
source and mutually exclusive CLI modes. Sparse arrays were also observed
failing before their rejection was implemented.

Focused source, history, CLI and mixed-language suites pass. Strict all-target
Clippy and repository formatting pass. The full `--no-fail-fast` suite reports
282 passing tests, one failure, zero ignored. The failure is the repository
self-check: `incomplete_library_class.rs` has data-clump findings, also reproduced
from an archive of the pre-change committed tree (`f27a566`). A scan of all 22
application rules additionally identifies existing duplicate-code and long-
parameter findings in that same file. No exclusions or test assertions were
weakened. This source feature introduces no remaining application findings.

Evidence logs are local under `/tmp/bluecheese-source-*.log`; application results
are under `/tmp/bluecheese-source-application/`. Existing public-project counts
in the public-example notebook remain reference-metric results; they do not
measure precision or recall of the new declaration check.

## Follow-up: repeated member bundles

The source evidence path now also checks repeated representations of one
resolved type. JavaScript/TypeScript property reads and zero-argument getter
calls, including mixed JS/TS imports, and Java zero-argument getter calls use
the existing adapter resolvers. Unresolved targets are omitted; types are not
joined by matching names. Java direct field reads, Rust member bundles,
destructuring, computed keys, JSX output and exhaustive enum branching are not
covered by this initial bundle check.

Reads must participate in an explicit representation: JS/TS object/array
expressions or template strings, or Java array initializers/constructor calls.
Separate expressions and differently named receiver instances are not pooled.
The detector requires a common set of at least three distinct members across
three independent functions/methods, constituting at least 60% of each matched
projection's members. Partial overlap is supported. Similarity does not chain
transitively: a group needs a real common set. Repeated projections inside one
consumer count once. Generated consumers and own-type implementations do not
count as independent sites. Assignments and updates are not read evidence.

These defaults prevent repeated reads of one field or a pair from triggering,
even when those reads cover 100% of a small type or appear in ten consumers.
Ordinary method calls such as `start`, `stop` and `reset` are not getters. A
large collection of unrelated reads without a representation is also excluded.

`BundleRules::new(min_members, min_consumers, min_overlap_percent)` and
`shotgun::source_evidence_with` expose validated library configuration. At least
three members, two consumers, and an overlap percentage in 1..=100 are required.
The overlap percentage measures the common set against each projection, not
against a type's possibly incomplete declared member surface. The CLI uses the
defaults in both default and `--source-only` modes; `--reference` is unchanged.

Intersection closure is bounded to 4,096 candidate common sets per resolved
type. Exceeding that budget returns an explicit incomplete-analysis error,
never a partial or empty clean report.

Sixteen integration tests cover overlapping partial representations, Java
getters, mixed-language imports/aliases, local JS objects, tiny complete types,
single reads/pairs, repetition, unrelated types/instances, commands, writes,
generated consumers, configuration boundaries, non-transitive overlap, default
CLI behaviour without Git and the analysis budget. The initial suite was
observed red (three positives missing, seven controls passing); the overlap and
write regressions were also observed failing before implementation fixes.

This is our source heuristic, not a rule reproduced from HIST. In
[HIST, §2.2.2](https://fpalomba.github.io/pdf/Journals/J1.pdf), Shotgun Surgery is
detected using historical co-change association rules. The paper discusses
frequent accessor use under Feature Envy; it does not specify these bundle
thresholds. Passing the controls establishes behaviour on those examples, not
a public-project false-positive rate. Legitimately separate representations
sharing three or more members can still be reported as review candidates.
