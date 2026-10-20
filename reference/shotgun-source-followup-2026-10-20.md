# Shotgun Surgery source follow-up: Java metadata and further public examples

## Update: Discord.js duplicate static catalogue fixed

The existing parallel-declaration rule now identifies `MessageTypes` in both
`src/util/Constants.js:580` and `typings/enums.d.ts:89` at the pinned Discord.js
snapshot below. These are two locations of one duplicated catalogue. The earlier
zero-source-findings result in this document describes the implementation before
this fix.

Behaviour commit: `9bf99b7`; behaviour-preserving extraction: `b8356ba`.
The fix recognises top-level `exports.Name = [...]` and
`module.exports.Name = [...]`, numeric TypeScript enums, and explicit `null`
slots in those runtime arrays. Enum/list identities use the same normalisation.
Known numeric encodings must agree: reserved slots and explicit/implicit values
are preserved rather than discarded to obtain a match. Ordinary symbolic name
lists retain their existing membership comparison.

The regression suite contains the actual upstream `MessageTypes` declarations
with attribution and licence. Controls cover shifted values, collapsed gaps,
reordered labels, duplicate array labels, holes/spreads, dynamic enum expressions,
unsafe integer values, signed/radix literals, ordinary instance assignments and
shadowed CommonJS names (including imports and destructuring). No public code is
executed. General helper wrappers such as `createEnum(...)` and computed enum
expressions remain outside this bounded declaration check.

The full suite passes 322 tests with the same pre-existing self-scan test
explicitly filtered. Strict Clippy and formatting pass. All 22 source detectors
return zero findings in the changed production files. This fixes source
recognition of the original duplicate-static pattern; the history rule remains
unchanged.

## Earlier evaluation

The Java metadata gap is fixed. On jEdit's complete pre-refactor snapshot,
source analysis now identifies the focused `VFSBrowser.filesActivated` method.
It produces seven related metadata findings at six distinct method locations:
four repeated read bundles and three repeated file-kind handlers. These are
structural evidence, not seven independently confirmed defects.

This extends the [earlier evaluation](shotgun-cochange-evaluation-2026-10-20.md).
Full per-case findings and provenance are in the
[machine-readable results](shotgun-source-followup-results-2026-10-20.json).
Its original results remain a record of the previous implementation. The new
Discord.js and Phoenix cases were selected independently of detector output.
They are not a representative benchmark or a precision/recall estimate.

## Java metadata: what was actually missing

Repository: [jEdit](https://github.com/jEdit-editor/jEdit).
Before snapshot: `7ea901a294b7ca96e39971c919974fa4d7d4a1a6`, scope `org/`,
253 Java files. The subsequent [VFS refactor](https://github.com/jEdit-editor/jEdit/commit/058dde7f873ecd291a3b409951d40312c58a8c43)
migrates direct metadata accesses to getters and lazy attribute handling.

There were three extraction gaps and one missing evidence pattern:

1. Direct Java field reads were absent; only getters were collected.
2. Reads scattered through conditions, local assignments and UI updates were
   absent unless contained in one array or constructor expression.
3. Importing the outer `VFS` class did not resolve `VFS.DirectoryEntry`.
4. Repeated handling of named file kinds was invisible when consumers only
   read `type` and `path`.

The implementation now resolves instance-field declarations, including their
declaring type, and collects imperative Java reads within each method. Receiver
bindings remain separate, including identically named variables in separate
blocks. Writes, increments, static constants, unresolved fields and own-type
accesses do not become ordinary read bundles.

A separate evidence path recognises equality comparisons against at least two
named `static final` fields of that same declaring type. Consumers must inspect
the same discriminator and share the case values. This uses the existing
three-consumer and 60% overlap defaults, while ordinary bundles retain their
three-member minimum. It is currently a Java class-constant check, not a general
enum/switch analyser, and does not require identical branch outcomes.

The full run returns 41 raw findings. Seven concern `VFS.DirectoryEntry`:

| Evidence | Method location in the parent snapshot | Shared knowledge |
| --- | --- | --- |
| Read bundle | `browser/FileCellRenderer.java:55` | `name`, `path`, `type` (3/4 reads) |
| Read bundle | `browser/VFSDirectoryEntryTable.java:398` | Same three fields |
| Read bundle | `browser/VFSFileChooserDialog.java:327` | Same three fields |
| Read bundle | `io/VFS.java:940` | Same three fields |
| Case handling | `browser/FileCellRenderer.java:168` | `type`: `DIRECTORY`, `FILESYSTEM` |
| Case handling | `browser/VFSBrowser.java:783` (`filesActivated`) | Same two cases |
| Case handling | `io/VFS.java:940` | Same two cases |

Paths above are under `org/gjt/sp/jedit/`. The focused method has only two
ordinary fields; its detection comes from repeated case handling. The other
34 raw findings have not been validated as defects. They include the earlier
editable tab-size preset candidate, which this metadata fix does not resolve.

Regression tests cover direct reads, imperative getters, imported nested types,
repeated kind handling, normal field pairs, a single kind check, writes,
increments, missing members, constants and separate receiver bindings.

## New example: Discord.js repeatedly maintains parallel enum declarations

[Maintainer explanation](https://github.com/discordjs/discord.js/discussions/7380):
v13 and earlier maintained enum declarations in JavaScript, TypeScript typings
and the separate API-types package. The v14 approach centralises the source.
We verified the first two copies directly in the public repository; the third
package was not cloned or counted as independently inspected evidence.

Inspected snapshot: tag `13.17.1`, commit
`1e0603551573e7bab65b9f8c325081507849bdbb`. Source scan: 307 files
(303 JavaScript, four TypeScript), zero findings, no parse failure.

Two distinct commits modify the same `MessageTypes` declarations in
`src/util/Constants.js` and `typings/enums.d.ts`:

| Commit | Coordinated catalogue addition |
| --- | --- |
| [428798374f4c3c2128b844d476627762dd45edf4](https://github.com/discordjs/discord.js/commit/428798374f4c3c2128b844d476627762dd45edf4), 2023-01-13 | Adds `AUTO_MODERATION_ACTION` to the runtime array and the typings enum |
| [0afa405f5a967ed5d82cfeffedc84d444c034924](https://github.com/discordjs/discord.js/commit/0afa405f5a967ed5d82cfeffedc84d444c034924), 2023-02-17 | Adds `ROLE_SUBSCRIPTION_PURCHASE` to both declarations |

These are actual matched additions on multiple occasions, not just two files
listed in a commit. Other files implement the new features; those edits are
not automatically classified as smells. This confirms recurring maintenance
of two representations of the same catalogue without relying on shared callers.

Source detection should be feasible here. Our catalogue extractor currently
misses exported assignments such as `exports.MessageTypes = [...]`, numeric
TypeScript enums and index-preserving `null` gaps. Other catalogues also use
the local `createEnum` wrapper. Recognising these requires preserving numeric
meaning and resolving the wrapper, rather than discarding gaps or blindly
treating arbitrary calls as declarations. No Discord.js-specific fix is
included in the Java metadata change.

## New example: Phoenix consolidates repeated error-code switches

[Issue #1421](https://github.com/kamp-us/phoenix/issues/1421) identifies six
non-exhaustive switches over `FateWireCode`. The original report was agent-filed
and later triaged; the label alone is not our validation. We inspected the
pre-refactor source and the actual consolidation diff.

Before snapshot: `96cba712e1502b3cf4e31e1fe5c3cf933d2b2941`, scope
`apps/web/src/`. Source scan: 131 files (65 TypeScript, 66 TSX), zero findings,
no parse failure. The switches occur in `usernameMessages.ts`,
`DefinitionCard.tsx`, `PanoPostDetail.tsx` (two), `SozlukTermPage.tsx` and
`PanoSubmitPage.tsx`.

[Refactor 487b9cc219dedaaa10b65636920b7864cf8d29cd](https://github.com/kamp-us/phoenix/commit/487b9cc219dedaaa10b65636920b7864cf8d29cd)
replaces these mappings with an exhaustive `Record<FateWireCode, string>`
registry and per-surface overrides. It also lifts a shared submit hook. The
diff deliberately leaves distinct gating flows separate, an important control
against merging superficially similar error handling.

The source opportunity is repeated handling of the same typed scalar catalogue:
`switch(code)` cases, not reads of several object members. Our Java case path
does not yet handle TypeScript switches, string-union aliases or these local
mapping functions. We have verified this one refactor episode, not established
repeated co-change across the Phoenix history, so it is not a historical-recall
case.

## Enforma: threshold-only experiment

An isolated copy of our implementation changed only the default ordinary
member minimum from three to two. The production minimum remains three.

| Input | Production default | Experimental minimum of two |
| --- | --- | --- |
| Original Enforma parent (69 JS/TS files) | 0 | 0 |
| Three ordinary `Pair { x, y }` readers returning `[p.x, p.y]` | 0 | 3 |

Thus lowering the field threshold alone does not recover Enforma, while it
introduces the ordinary-pair false positive. Its repeated `statusText` branches
and `message` fallback are still invisible because the callback receiver is
untyped and the accesses are not collected as a constructed representation.
This is not evidence that two repeated cases are insufficient: `UNAUTHORIZED`
and `UNPROCESSABLE ENTITY` already provide a repeated decision vocabulary. A
conditional-policy extractor could use that vocabulary and repeated outcomes
without flagging ordinary two-field reads. That JS/TS capability is not yet
implemented.

## Android: the protocol does have an underlying object

At October's parent snapshot, the new Java rule reports ten repeated field-read
locations, all inside `FragmentManager`, concerning `Fragment` state such as
`mAdded`, `mDetached`, `mHasMenu`, `mHidden` and `mMenuVisible`. These are clues
to an underlying object and state protocol, not successful identification of
the targeted adapter method. Concentration in the intended manager also weakens
the inference of scattered ownership; those ten locations are not ten verified
smells.

The adapters change visibility through setters, while the manager reads state
and enforces lifecycle transitions. Connecting these roles would require tracing
setter effects, reads and conditions on the same state, including compatibility
wrappers and platform types. It is not inherently impossible to recognise a
source relationship. However, the new deferred-start field and requirement do
not exist in the parent: source alone cannot prove that a future feature must
modify a particular set of sites. Historical recurrence remains independent
evidence for that maintenance obligation.

## Verification and limits

### Forced history follow-up

The current history detector was subsequently run independently of source
analysis on both new snapshots:

| Snapshot | Parsed files | History findings | Execution/parse errors |
| --- | --- | --- | --- |
| Discord.js `13.17.1` | 307 | 0 | None |
| Phoenix before consolidation | 131 | 0 | None |

Discord.js's 128 non-merge-commit window reaches
`4cf05559a2cc6c4dfc409f8aab60256e5b030bca` (2022-07-20). Both verified
`MessageTypes` additions are inside that window. Its failure to identify that
catalogue is therefore not explained by the window. The implementation selects
shared-callee methods and compares method bodies; parallel declarations never
become the responsibility being tracked.

Phoenix's zero result does not establish failure against independently proven
recurrence: we have verified one consolidation episode, not recurring groups.
The repeated scalar-code switches still do not fit the detector's prerequisite.

The history implementation also accepts a nonempty list of supporting commits:
one qualifying co-change suffices. It does not require repeated occasions.
Consequently these runs test the present implementation, not an independent
recurring-co-change method such as the user requested. A Git-confirmed example
and a successful detector retrieval must remain distinct claims.

The behaviour change is commit `37b1dfe`; subsequent refactorings preserve its
tests and source evidence. New positive regressions were observed failing before
their fixes, including the imported nested-type case. A separate binding-scope
regression caught and prevented grouping different local instances.

The full suite has a pre-existing self-scan failure in
`src/detectors/incomplete_library_class.rs`. With that single test explicitly
filtered, 318 tests pass. Relevant source/coupling/history tests and strict
Clippy checks pass. All 22 source detectors also return zero findings in the
changed production files. The final CLI is built from `93343c8`; its SHA-256
and final rerun comparisons are recorded in the accompanying results.

Public source was read and parsed; none of the examined applications was built
or executed. This work does not repair the history rule's caller prerequisite,
its bounded window or the default-mode short circuit. Those limitations in the
earlier report still apply. Broader Java extraction also increases candidate
volume; the raw counts must not be presented as measured precision.
