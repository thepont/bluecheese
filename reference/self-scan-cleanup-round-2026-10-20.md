# Self-scan cleanup round — 2026-10-20

Ran all 22 detectors on a frozen source snapshot at `87fd6bd`, then reran them
after five small refactoring commits. The scope excludes `tests/` and `reference/`,
retaining the implementation and examples. Concurrent unrelated work was excluded
from the snapshot; this report describes that reproducible round.

## Changes

- `4574838`: replaced the long detector-dispatch chain with a function registry.
- `eb74dc7`: derived supported labels from that registry at compile time. Public
  signatures, label order and error handling remain unchanged.
- `1d63312`: reused Middle Man's existing analysis context to reduce a
  four-argument helper to three arguments.
- `d5be1df`: replaced Temporary Field's opaque field-metadata triples with
  DeclaredField and passed its node and name together. Its nullable-field helper
  now also takes three arguments.
- `0fd521e`: replaced three member-access language switches in Inappropriate
  Intimacy with a receiver/field mapping supplied by the language adapters.

## Validation

The broader baseline and final runs each passed **205 tests**, with the existing
source-cleanliness assertion filtered out. That assertion was run separately and
still failed on remaining data-clump candidates. All tests and golden reports were
unchanged. Strict **all-targets** Clippy and changed-file rustfmt checks passed.

All 22 detectors completed successfully on source both before and after cleanup.
All 22 fixture reports were byte-for-byte identical before and after cleanup.
Fixture comparison covers 206 supported source files. Source findings are review
candidates: these counts are neither confirmed defects nor an estimate of precision.
No suppressions, detector-rule changes or threshold changes were made.

## Source scan comparison

Counts below are reported locations, rather than grouped Finding objects. They are
not deduplicated across detectors and should not be added up as distinct defects.

| Detector | Before locations | After locations |
| --- | ---: | ---: |
| alt-classes | 0 | 0 |
| comments | 242 | 242 |
| data-class | 6 | 6 |
| data-clumps | 87 | 84 |
| divergent-change | 0 | 0 |
| dup-code | 43 | 43 |
| feature-envy | 0 | 0 |
| incomp-lib-class | 0 | 0 |
| intimacy | 24 | 24 |
| large-class | 4 | 4 |
| lazy-class | 0 | 0 |
| long-method | 4 | 3 |
| long-params | 2 | 0 |
| message-chains | 0 | 0 |
| middle-man | 0 | 0 |
| parallel-hierarchy | 0 | 0 |
| prim-obsession | 1 | 1 |
| refused-bequest | 0 | 0 |
| shotgun | 17 | 17 |
| spec-gen | 0 | 0 |
| switch-stmts | 13 | 10 |
| temp-field | 0 | 0 |

The long-parameter candidates fell from 2 to 0; long-method candidates from 4 to 3;
switch locations from 13 to 10; and data-clump locations from 87 to 84.
Other detector totals did not change. Zero results do not establish complete
language support or the absence of a smell.

The three remaining long-method candidates are in Data Class's accessor analysis,
Inappropriate Intimacy's non-Java project scan, and Refused Bequest's non-Java
analysis. Larger project analyser objects and repeated grammar decisions elsewhere
remain candidates for subsequent review. Comments and shared utility coupling
require contextual review rather than automatic removal.

## Remaining source inventory

All findings below remain unsuppressed and unreviewed. Locations refer to the end
of this snapshot round; later edits may move them. The owners and additional
information are retained from the analyser's output.

| Detector | File | Owner | Locations | Additional information |
| --- | --- | --- | --- | --- |
| comments | examples/dump_metrics.rs | dump_metrics | 1, 2, 4, 24, 25 |  |
| comments | examples/dump_types.rs | dump_types | 1, 3 |  |
| comments | examples/parse_check.rs | parse_check | 1, 3, 4, 5, 7 |  |
| comments | src/detectors/alt_classes.rs | alt_classes | 1, 3, 4, 5, 6, 7 |  |
| comments | src/detectors/comments.rs | comments | 1 |  |
| comments | src/detectors/cross_language_duplicates.rs | cross_language_duplicates | 1 |  |
| comments | src/detectors/data_class.rs | data_class | 1 |  |
| comments | src/detectors/data_clumps.rs | data_clumps | 1, 104, 105, 106 |  |
| comments | src/detectors/divergent_change.rs | divergent_change | 1, 3, 4, 5 |  |
| comments | src/detectors/dup_code.rs | dup_code | 1 |  |
| comments | src/detectors/feature_envy.rs | feature_envy | 1, 3, 4, 5 |  |
| comments | src/detectors/incomplete_library_class.rs | incomplete_library_class | 1, 3, 4, 5, 6 |  |
| comments | src/detectors/intimacy.rs | intimacy | 1 |  |
| comments | src/detectors/large_class.rs | large_class | 1 |  |
| comments | src/detectors/lazy_class.rs | lazy_class | 1, 3, 4, 5 |  |
| comments | src/detectors/long_method.rs | long_method | 3, 4, 5, 7, 8, 9 |  |
| comments | src/detectors/long_params.rs | long_params | 3, 4, 5, 8, 9, 12, 13 |  |
| comments | src/detectors/message_chains.rs | message_chains | 1, 3, 4 |  |
| comments | src/detectors/middle_man.rs | middle_man | 1 |  |
| comments | src/detectors/mod.rs | mod | 1 |  |
| comments | src/detectors/parallel_hierarchy.rs | parallel_hierarchy | 1, 3, 4, 5 |  |
| comments | src/detectors/primitive_obsession.rs | primitive_obsession | 1, 3, 4, 5 |  |
| comments | src/detectors/refused_bequest.rs | refused_bequest | 1 |  |
| comments | src/detectors/shotgun.rs | shotgun | 1, 2 |  |
| comments | src/detectors/spec_gen.rs | spec_gen | 1 |  |
| comments | src/detectors/switch_stmts.rs | switch_stmts | 1, 3, 4, 5, 6 |  |
| comments | src/detectors/temp_field.rs | temp_field | 1, 3, 4, 5 |  |
| comments | src/finding.rs | finding | 1, 2, 3 |  |
| comments | src/lang/coupling.rs | coupling | 1 |  |
| comments | src/lang/ecmascript/calls.rs | calls | 1, 2 |  |
| comments | src/lang/ecmascript/mod.rs | mod | 1, 2, 3 |  |
| comments | src/lang/java/calls.rs | calls | 1, 2, 3 |  |
| comments | src/lang/java/metrics.rs | metrics | 3, 4, 11, 12, 13, 15, 16, 17, 19, 20, 53 |  |
| comments | src/lang/java/mod.rs | mod | 1, 3, 4, 5 |  |
| comments | src/lang/java/types.rs | types | 1, 2, 9, 56, 71, 72, 73, 74, 142, 143 |  |
| comments | src/lang/mod.rs | mod | 3, 4, 5, 8, 9, 10, 11, 12, 29, 186, 187, 188, 190, 191, 192, 195, 198, 199, 200, 201, 204, 205, 206, 208, 209, 210, 211, 212, 213, 214, 276, 277, 314 |  |
| comments | src/lang/mod.rs | SourceFile | 42 |  |
| comments | src/lang/mod.rs | Project | 69, 70, 71, 72 |  |
| comments | src/lang/rust.rs | rust | 3, 4, 5, 6, 7, 9, 11, 12, 13, 14, 15, 23, 26, 91, 112, 123, 125, 126, 127, 136, 137, 138, 151, 152, 153, 154, 166, 167, 168, 239, 240, 241 |  |
| comments | src/lang/rust_calls.rs | rust_calls | 1 |  |
| comments | src/lang/structure.rs | structure | 1, 2, 3 |  |
| comments | src/lib.rs | lib | 1, 3, 4, 5, 6, 104, 105, 116, 117 |  |
| comments | src/main.rs | main | 6, 44, 45, 71, 72, 86, 87, 88, 146, 147, 148 |  |
| comments | src/main.rs | Args | 10, 22, 23, 24, 28, 29, 30 |  |
| comments | src/smells.rs | smells | 39, 177, 178, 186, 187, 188, 189, 204, 205 |  |
| comments | src/smells.rs | Smell | 25, 26, 27, 28, 30, 32, 33, 34, 35 |  |
| comments | src/walk.rs | walk | 1, 12 |  |
| data-class | src/lang/coupling.rs | MethodCoupling | 7 |  |
| data-class | src/lang/coupling.rs | Callable | 16 |  |
| data-class | src/lang/mod.rs | Project | 67 |  |
| data-class | src/lang/mod.rs | Scope | 216 |  |
| data-class | src/lang/structure.rs | Function | 30 |  |
| data-class | src/smells.rs | Smell | 24 |  |
| data-clumps | src/detectors/data_class.rs | data_class | 161, 234, 294 |  |
| data-clumps | src/detectors/data_clumps.rs | data_clumps | 33, 48 |  |
| data-clumps | src/detectors/divergent_change.rs | divergent_change | 45, 92, 118, 156 |  |
| data-clumps | src/detectors/dup_code.rs | dup_code | 84, 93 |  |
| data-clumps | src/detectors/incomplete_library_class.rs | incomplete_library_class | 92, 108, 120 |  |
| data-clumps | src/detectors/intimacy.rs | Analyzer | 417, 452, 481, 514 |  |
| data-clumps | src/detectors/middle_man.rs | middle_man | 56, 185, 266, 321, 378 |  |
| data-clumps | src/detectors/parallel_hierarchy.rs | parallel_hierarchy | 92, 98, 118, 146 |  |
| data-clumps | src/detectors/primitive_obsession.rs | primitive_obsession | 38, 84 |  |
| data-clumps | src/detectors/refused_bequest.rs | refused_bequest | 131, 238, 383, 392, 404, 473, 494, 509, 515 |  |
| data-clumps | src/detectors/switch_stmts.rs | switch_stmts | 118, 136 |  |
| data-clumps | src/detectors/temp_field.rs | temp_field | 111, 133, 144, 164, 194 |  |
| data-clumps | src/lang/ecmascript/calls.rs | calls | 517, 534, 549, 558 |  |
| data-clumps | src/lang/ecmascript/calls.rs | Project | 161, 229, 260, 274, 346, 379, 420 |  |
| data-clumps | src/lang/ecmascript/mod.rs | mod | 182, 208 |  |
| data-clumps | src/lang/ecmascript/mod.rs | EcmaModel | 76, 83, 91 |  |
| data-clumps | src/lang/java/calls.rs | Project | 327, 352, 361, 371, 380, 386, 396, 405 |  |
| data-clumps | src/lang/java/mod.rs | JavaModel | 35, 77, 81, 89 |  |
| data-clumps | src/lang/mod.rs | mod | 239, 273 |  |
| data-clumps | src/lang/mod.rs | LanguageModel | 81, 111, 118, 124, 137 |  |
| data-clumps | src/lang/rust.rs | RustModel | 102, 109 |  |
| data-clumps | src/lang/structure.rs | Normalizer | 177, 216 |  |
| dup-code | src/detectors/data_class.rs | data_class | 13, 82, 161, 234 |  |
| dup-code | src/detectors/data_clumps.rs | data_clumps | 177, 189 |  |
| dup-code | src/detectors/divergent_change.rs | divergent_change | 45, 92, 118 |  |
| dup-code | src/detectors/intimacy.rs | intimacy | 30, 85 |  |
| dup-code | src/detectors/intimacy.rs | Analyzer | 452, 481 |  |
| dup-code | src/detectors/large_class.rs | large_class | 34, 107, 135, 231 |  |
| dup-code | src/detectors/lazy_class.rs | lazy_class | 44, 54 |  |
| dup-code | src/detectors/middle_man.rs | middle_man | 56, 185, 266, 321 |  |
| dup-code | src/detectors/refused_bequest.rs | refused_bequest | 45, 131, 143, 238, 473, 494 |  |
| dup-code | src/lang/ecmascript/calls.rs | Project | 161, 207, 260, 274, 346, 379, 420 |  |
| dup-code | src/lang/java/calls.rs | calls | 494, 505 |  |
| dup-code | src/lang/java/calls.rs | Project | 258, 306, 327 |  |
| dup-code | src/lang/structure.rs | Normalizer | 177, 216 |  |
| intimacy | src/detectors/dup_code.rs | MethodFingerprint | 68 |  |
| intimacy | src/detectors/intimacy.rs | Analyzer | 377, 386, 390, 391, 426, 428, 432, 458, 462, 487, 497, 498, 522, 524, 530 |  |
| intimacy | src/lang/ecmascript/calls.rs | Project | 158, 162, 171, 186, 215, 226, 279, 286 |  |
| large-class | src/detectors/intimacy.rs | Analyzer | 20 |  |
| large-class | src/lang/ecmascript/calls.rs | Project | 27 |  |
| large-class | src/lang/java/calls.rs | Project | 28 |  |
| large-class | src/lang/structure.rs | Normalizer | 70 |  |
| long-method | src/detectors/data_class.rs | data_class | 161 |  |
| long-method | src/detectors/intimacy.rs | intimacy | 85 |  |
| long-method | src/detectors/refused_bequest.rs | refused_bequest | 143 |  |
| prim-obsession | src/lang/coupling.rs | MethodCoupling | 7 |  |
| shotgun | src/lang/accessors.rs | MemberAccessorSyntax | 47 | Shotgun Surgery [CM=3, CC=3, Fan-Out=1] |
| shotgun | src/lang/coupling.rs | coupling | 34 | Shotgun Surgery [CM=20, CC=18, Fan-Out=0] |
| shotgun | src/lang/ecmascript/mod.rs | mod | 163 | Shotgun Surgery [CM=4, CC=3, Fan-Out=0] |
| shotgun | src/lang/ecmascript/mod.rs | mod | 182 | Shotgun Surgery [CM=3, CC=3, Fan-Out=1] |
| shotgun | src/lang/ecmascript/mod.rs | mod | 194 | Shotgun Surgery [CM=3, CC=2, Fan-Out=0] |
| shotgun | src/lang/ecmascript/mod.rs | mod | 204 | Shotgun Surgery [CM=3, CC=2, Fan-Out=0] |
| shotgun | src/lang/ecmascript/mod.rs | mod | 208 | Shotgun Surgery [CM=2, CC=2, Fan-Out=1] |
| shotgun | src/lang/ecmascript/mod.rs | mod | 219 | Shotgun Surgery [CM=4, CC=3, Fan-Out=0] |
| shotgun | src/lang/java/calls.rs | calls | 480 | Shotgun Surgery [CM=56, CC=22, Fan-Out=0] |
| shotgun | src/lang/mod.rs | mod | 163 | Shotgun Surgery [CM=40, CC=24, Fan-Out=0] |
| shotgun | src/lang/mod.rs | mod | 234 | Shotgun Surgery [CM=3, CC=3, Fan-Out=0] |
| shotgun | src/lang/mod.rs | SourceFile | 51 | Shotgun Surgery [CM=18, CC=12, Fan-Out=0] |
| shotgun | src/lang/mod.rs | SourceFile | 56 | Shotgun Surgery [CM=10, CC=9, Fan-Out=0] |
| shotgun | src/lang/mod.rs | SourceFile | 60 | Shotgun Surgery [CM=109, CC=33, Fan-Out=0] |
| shotgun | src/lang/mod.rs | ParserCache | 321 | Shotgun Surgery [CM=13, CC=8, Fan-Out=0] |
| shotgun | src/walk.rs | walk | 6 | Shotgun Surgery [CM=57, CC=24, Fan-Out=0] |
| shotgun | src/walk.rs | walk | 23 | Shotgun Surgery [CM=3, CC=3, Fan-Out=0] |
| switch-stmts | src/detectors/data_class.rs | data_class | 84 |  |
| switch-stmts | src/detectors/intimacy.rs | intimacy | 261 |  |
| switch-stmts | src/detectors/large_class.rs | large_class | 40, 89 |  |
| switch-stmts | src/detectors/middle_man.rs | middle_man | 47, 219 |  |
| switch-stmts | src/detectors/primitive_obsession.rs | primitive_obsession | 39, 85, 111 |  |
| switch-stmts | src/lang/rust_calls.rs | rust_calls | 90 |  |
