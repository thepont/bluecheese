# Public co-change evaluation, 2026-10-20

The source rule matched **0 of 6 focused examples before their changes**. Forced
history analysis matched the focused method in **2 of 5 retrospective cases**.
The application default matched **0 of those 5 targets**: unrelated source
findings prevented the two successful history detections from being evaluated.

These are exploratory retrieval counts, **not precision or recall**. Three
Android changes belong to one family, and two jEdit cases were selected from
our own history output. Git co-change is confirmed; whether every propagation
represents misplaced responsibility remains a separate judgement. Necessary API
migrations and deliberate compatibility implementations can also co-change.

Machine-readable snapshots, results and scoring fields are saved in
[the results JSON](shotgun-cochange-results-2026-10-20.json). The read-only
[probe](cochange-probe.rs) calls the public source, history, reference and
application APIs. No public repository code was built or executed.

## What was compared

For each episode, source detection runs on its **parent**, before the edits.
History detection runs at the changed commit with its ancestors available.
A hit must identify the focused method at its actual path and declaration
line. Merely reporting another method in the same class is insufficient.
The frontend extraction is evaluated before the refactor only: expecting a
history detector to know that future extraction would be invalid, and scoring
the already centralised after state as a live defect would also be invalid.

| Episode and focused target | Source before: target hit | History after: target hit | Default after: target hit |
| --- | --- | --- | --- |
| Android, [August 2011 fragment fixes](https://github.com/aosp-mirror/platform_frameworks_support/commit/2a4d8518f36346ea25a22a736453ff28f2954165), v13 `FragmentPagerAdapter.instantiateItem` | No | No; reports `addFragment` instead | No; same other method |
| Android, [October deferred starts](https://github.com/aosp-mirror/platform_frameworks_support/commit/abc968f1eba800c34a4008deb43b015da5d23a5f), same target | No | No; returns older August evidence | No |
| Android, [November visible hints](https://github.com/aosp-mirror/platform_frameworks_support/commit/79398eaefea45e61d839cf4e0534f0eafee70a09), same target | No | No; returns older August evidence | No |
| jEdit, [lazy VFS attributes](https://github.com/jEdit-editor/jEdit/commit/058dde7f873ecd291a3b409951d40312c58a8c43), `VFSBrowser.filesActivated` | No | **Yes**, exact episode commit | No; tab-size lists stop history |
| jEdit, [JEditBuffer extraction](https://github.com/jEdit-editor/jEdit/commit/d571dc9c2b689c28ab37dd36a7ea846d0de81279), `JEditTextArea.propertiesChanged` | No | **Yes**, exact episode commit | No; tab-size lists stop history |
| Enforma, [developer-labelled shotgun refactor](https://github.com/Lara-Simonetti/enforma-front/commit/44a108c1ac748a4489a0d3ea57a59187110b40b1), repeated error policy in 12 components | No | Not scored | Not scored |

The history output retrieves the selected **commit ID** in 3/5 episodes, but
locates the selected **method** in only 2/5. The distinction matters: the August
Android commit genuinely appears in the result, attached to a different method.
Repeatedly returning that same August commit in later snapshots is one piece
of evidence, not three independently discovered co-change events.

## Why each case was missed or found

### Android: callbacks and distributed lifecycle decisions

The three Git patches change v13/v4 adapter behaviour alongside
`FragmentManagerImpl` lifecycle behaviour. October, for example, changes
`instantiateItem` in both adapters and adds a deferred-start check to
`moveToState(Fragment,int,int,int)`. November coordinates visible-hint behaviour
in both adapters with fragment state handling. These are actual source diffs,
not just commits that happen to contain several filenames.

The source rule finds no member bundles or repeated catalogues. The concern is
distributed lifecycle/conditional logic, not constructing three-member object,
array or message representations. Deliberate v4/v13 compatibility duplication
is also a legitimate consideration when judging the design.

The static reference run reports no concrete `FragmentPagerAdapter` target.
`ViewPager` invokes `mAdapter.instantiateItem(...)` through the base adapter
type; our bounded resolver does not produce the needed incoming edges to the
concrete override. The history rule first requires shared-callee coupling, so
this callback-shaped relationship never becomes a history candidate.

The history rule **does** identify `FragmentManagerImpl.addFragment` at the
August commit: CM=3, CC=2. This is a useful successful co-change retrieval,
but not the labelled/focused method. The October and November runs return that
same August evidence rather than evidence for the newer focused episodes.

At the paper's exact Android-support revision
`0f6f72e19db852cc253fd2fc05459abdf8d5c3af`, the last 128 non-merge commits reach
back only to 2011-11-18. All three inspected episodes precede that window.
The reference output has 83 coupling findings; source, history and default
outputs each have zero. This is additional evidence of a window limitation,
not a reproduction of HIST's full-history analysis.

### jEdit VFS: public fields and imperative representations

The VFS refactor replaces direct accesses such as `file.type` and `file.path`
with getters and changes file metadata handling across browser code and its
consumers. The parent source is a relevant test for scattered representations.
The current bundle extractor supports Java getters, but **not Java direct
field reads**; it also requires a single constructor/array representation and
does not recognise an imperative UI assembled across statements.

At the exact changed commit, history detects
`VFSBrowser.filesActivated` at line 787, with CM=3, CC=3, and the exact supporting
commit `058dde7f873ecd291a3b409951d40312c58a8c43`. Its signature remains identifiable
while its body and caller bodies change, satisfying the implemented history rule.

This demonstrates historical change propagation. Since the commit explicitly
refactors the VFS API, that propagation alone does not prove an unresolved smell
still exists after the change. This case was selected from our history output;
it cannot serve as an independent recall test.

### jEdit buffer extraction: history finds a migration, source does not

The JEditBuffer extraction changes 17 files, including the text area and its
consumers. History identifies `JEditTextArea.propertiesChanged` at line 4656
with CM=5, CC=3 and supporting commit
`d571dc9c2b689c28ab37dd36a7ea846d0de81279`. The parent source has no finding at
the selected method. A buffer/type migration is broader than the current
member-bundle representation check.

As with the VFS case, this is a detector-selected example of actual co-change,
and a useful **API-migration interpretation control**, not independently proven
design damage. A history detector can correctly identify body co-change while
still over-interpreting its architectural meaning.

### Enforma: repeated policy over only two untyped properties

GitHub commit-message search found the developer-labelled merge
`44a108c1ac748a4489a0d3ea57a59187110b40b1`. The parent is
`4bc7c173b2bd9593ca4cb52264f7a96f626340e2`; the implementation branch tip is
`95a002bc52bbc9e26c0009604123d46e681fff3a`. The diff changes twelve Angular
components and introduces `ErrorMessageMapperPipe` to own repeated status-to-
message decisions. This is an application example, rather than a teaching fixture.

Before the extraction, components repeatedly branch on `error.statusText`,
map the same status strings to the same messages, and fall back to
`error.message`. All 69 JS/TS-family source files parse. Source and default
before-state runs return zero findings. History also returns zero before the
future extraction, which is not scored as failure to predict the future.

This misses for several concrete reasons: the receiver is an untyped callback
argument, the accesses involve only two distinct properties, and the duplicated
knowledge is a **conditional mapping**, not a constructed representation.
Lowering the member floor would endanger the legitimate two-field controls.
The missing capability is recognising repeated decisions, not merely accepting
smaller field bundles.

## A public false-positive control and a default-mode failure

In the jEdit snapshots, the source rule reports two locations in one group:
`tabSizes = {"2", "4", "8"}` in `EditingOptionPane` and `PrintOptionPane`.
Both combo boxes call `setEditable(true)`. These arrays are **suggested presets**,
not exhaustive definitions of the valid values. Adding another valid tab size
does not require changing both arrays, and editing/printing are distinct settings.

This is a reviewed **likely false positive / weak candidate** for the claimed
change obligation. It is not counted as a historical hit. The same group
recurs across snapshots and remains one reviewed control, not many false positives.

The application returns these two findings immediately and never evaluates
history. Forced history detects `filesActivated` in the VFS after snapshot and
`propertiesChanged` in the buffer after snapshot; the default returns neither.
This was also reproduced with the actual CLI, whose findings match the probe's
public-API output. An unrelated source candidate must not prevent an independent
history analysis from finding other responsibilities.

## Published-label alignment and other controls

The [HIST paper](https://fpalomba.github.io/pdf/Journals/J1.pdf) and
[replication package](https://doi.org/10.6084/m9.figshare.1157374) provided the
Android/Tomcat/jEdit leads. Oracle rows are not interchangeable independent smells:
some are method pairs and some are class records. We did not divide hits by CSV rows.

| Extra snapshot/control | Inventory and raw outputs | Interpretation |
| --- | --- | --- |
| Tomcat paper hash `398ca7eedce89f05011104134a09c80208414cef`, `java/` | 1,167 Java; source 0, history 0, reference 1,123, default 0 | The labelled `AsyncStateMachine` type is absent. Excluded from target scoring; this is a label/snapshot alignment problem. |
| Tomcat 7.0.39, `5f6f258107e7e463cce41187e13474f3c894693e`, `java/` | 1,260 Java; source 0, history 0, reference 1,162, default 0 | Later proxy. `isAsync` is an eligible coupling candidate, but its wrapper body has no change in the bounded window. Not an exact HIST reproduction. |
| jEdit August 2005 mirror proxy `280fb40364f5fbf6bae816dccb6cc038f3d43fef`, `org/` | 277 Java; source 2, history 2, reference 449, default 2 | Published mouse-handler target is not located; both source findings are the preset group. Both history findings concern other methods. Original research revision `feb608ea8a349858dc905ede78b4b74810035987` is unavailable from this mirror, so excluded from exact benchmark scoring. |
| Roommate before labelled factory extraction, `604bb07ea52692bd84bdf330c5365e77f45c5d59`, `src/main/java/` | 35 Java; all four modes return 0 | Retained as caller-migration/control evidence. The labelled [refactor](https://github.com/team-propra/roommate/commit/3f1348ec8bd79dfaabd2ecb17d70059722271dc0) does not independently establish an unresolved smell. |

The original jEdit short hash was recovered from the replication CSV rather
than trusting PDF OCR. Its original full hash could not be fetched from the
official mirror. Browser-local `MouseHandler` names also occur more than once,
so matching a simple class name would be ambiguous.

One supplied Tomcat transaction illustrates why we checked Git independently:
the package labels `AsyncStateMachine.isAsync()` modified at
`6c72fd4b24c2c468d8e6d3c8d7208bb37d4feb4c`, but that Git commit does not change
`AsyncStateMachine.java` at all. Its diff changes interface declarations/imports
and an annotation in four processor files. That record cannot establish a
body co-change under our definition. This is an observed disagreement on one
record, not a claim that all published labels are wrong.

The apparent beginning of `isAsync` in the shallow 2013 Tomcat clone is a
shallow boundary, not evidence of its real introduction. Its last-128-commit
window starts at `ac992f249a61d9e28840682c5ec389d967c65fb8`, 2013-01-24.

## What the evidence suggests changing next

1. Run source and history independently and combine their evidence. A weak
   source candidate currently hides useful history results elsewhere.
2. Remove resolved shared callers as a prerequisite for historical candidates.
   Repeated co-change needs its own candidate discovery, including callbacks
   and disconnected decisions; the current rule only validates coupling candidates.
3. Extend source evidence using these real cases: Java direct fields,
   representations assembled across statements, and repeated conditional
   mappings. Keep ordinary single/pair reads as negative controls; do not just
   lower thresholds until these examples pass.
4. Establish whether a literal list defines a closed set or merely offers
   suggestions. The editable tab-size controls expose the distinction.
5. Expand to independently selected projects with aligned snapshots and human
   judgement of the shared responsibility. Do not optimise against detector-
   selected examples and then present that as public-project recall.

No Shotgun detector code or thresholds were changed during this evaluation.

## Reproduction and artefacts

The selected Shotgun, language, history and CLI code is the code at
`7c567572543ee1227b8893d37d75a0aa878748be`. Concurrent commits changed only the
unselected incomplete-library-class detector and its tests while the release
build was made. The actual executables were frozen before evaluation; their
SHA-256 values are recorded in the JSON. Subsequent runs used the same frozen
release probe. This distinguishes rule provenance from a whole-build commit claim.

The first debug Android history run exhausted its 60-second budget, and the
first partial-clone jEdit history run also failed. These are recorded execution
failures, **not misses**. Historical objects were materialised and optimised runs
completed successfully. Complete jEdit clones and materialised Android history
were used for final event results. No final scored record has a parse or analysis
error. Tomcat's retained depth is 500, sufficient for its explicit 128-commit window.
Enforma and Roommate were also rerun with the frozen release probe; their four
mode outputs match the initial debug-probe outputs byte for byte.

The source defaults remained three shared members, three independent consumers,
and 60% overlap. Source findings can also come from the earlier parallel-
declaration rule; the tab-size finding comes from that rule, not member bundles.
This round produced no historical target hit attributable to the new bundle rule.

The raw working evidence is under `/tmp/bluecheese-cochange-20261020/`. The JSON
records commit parents, roots, inventories, complete source/history/default
findings for the scored episodes, and reference counts. Empty lists with errors
are never scored as successful clean scans. CLI/public-API agreement was checked
on the jEdit masking case and the August Android history finding. The recorded
summary was recomputed from the per-case matching fields.
All 41 existing source-bundle, source-declaration and history regression tests
pass. This research change adds documentation/data and a read-only probe;
it does not change the application rule.

To build the probe alongside the pinned analyser's release libraries:

```sh
CARGO_TARGET_DIR=/tmp/bluecheese-eval-build cargo build --locked --release
analysis_target=/tmp/bluecheese-eval-build/release/deps
rustc -O --edition 2024 reference/cochange-probe.rs \
  -L "dependency=$analysis_target" \
  --extern "bluecheese=$(ls "$analysis_target"/libbluecheese-*.rlib | head -1)" \
  --extern "serde_json=$(ls "$analysis_target"/libserde_json-*.rlib | head -1)" \
  -o /tmp/probe-release
```

For a pinned checkout, the four probe modes are:

```sh
probe-release /path/to/pinned/source source
probe-release /path/to/pinned/source history
probe-release /path/to/pinned/source reference
probe-release /path/to/pinned/source default
```

`history` directly calls the existing public `history::detect` API because
the CLI currently provides no history-only switch. It preserves the source
scope and Git root; the downloaded repositories are treated as data throughout.
