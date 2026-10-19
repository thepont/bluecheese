# Public shotgun surgery examples

Investigated 2026-10-20. This is an evidence notebook, not a list of confirmed
defects in current releases. Keep adding examples here, including weak leads
and controls. A large commit or frequently co-changing files alone does not
establish that a responsibility is scattered.

## What BlueCheese currently identifies

Checked against BlueCheese revision `92560222b1745a47d3ba152846445483d6017e15`.
`src/lib.rs` recognises the `shotgun` catalogue label but refuses it because
there is no detector. The CLI analyses source snapshots, not Git history.

| Input actually scanned with `-s shotgun` | Source files analysed | Observed result |
| --- | ---: | --- |
| Tomcat 7.0.39, `java/` | 1,260 Java | Exit 1: no detector for `shotgun` |
| Roommate immediately before the developer-labelled commit below, `src/main/java/` | 35 Java | Exit 1: no detector for `shotgun` |
| Bundled SmellBench `main/java/shotgun/` | 6 Java | Exit 1: no detector for `shotgun` |
| Clean Code School's two shotgun surgery files | 0 | Exit 1: no source files; JavaScript has no adapter |

These are unsupported analyses, **not clean results and not measured false
negatives**. Detection precision and recall cannot yet be calculated. No
detector was implemented or changed during this investigation.

## 1. Apache Tomcat: asynchronous request state

**Evidence:** independently published example, plus related Git-history
observations. **Status:** retain as a benchmark candidate; the published result
has not been reproduced here.

The [HIST paper, Figure 5 and section 3.2.2](https://fpalomba.github.io/pdf/Journals/J1.pdf)
names `org.apache.coyote.AsyncStateMachine.isAsync()` as a shotgun surgery
example. Its historical analysis associates it with 48 methods in 31 classes.
Those figures are the authors' findings, not measurements from this run.

The [public replication package](https://doi.org/10.6084/m9.figshare.1157374)
was downloaded and inspected. Its nested archive
`Study I - Oracles/ShotgunSurgery.zip`, file
`ShotgunSurgery/apache-tomcat-shotgun-surgery-ORACLE.csv`, contains these pairs:

```text
org.apache.coyote.http11.Http11AprProcessor.process(SocketWrapper);org.apache.coyote.http11.Http11Processor.process(SocketWrapper)
org.apache.coyote.http11.Http11AprProcessor.process(SocketWrapper);org.apache.coyote.AsyncStateMachine.isAsync()
```

The corresponding `Study I - Identified Smells/ShotgunSurgery.zip` HIST result
contains the same two rows. These are supplied labels and supplied detector
outputs; their agreement is not an independent rerun of HIST.

For a separate, bounded Git check, cloned [apache/tomcat](https://github.com/apache/tomcat)
at tag `7.0.39`, revision `5f6f258107e7e463cce41187e13474f3c894693e`,
with depth 500. Snapshot date: 2013-03-22. Relevant changes found:

| Commit | Logical concern | Files changed | Production Java files |
| --- | --- | ---: | ---: |
| [cf4fae533cb303c97031b68ec652d6624207df21](https://github.com/apache/tomcat/commit/cf4fae533cb303c97031b68ec652d6624207df21) | Async timeout and error dispatch handling | 5 | 3 |
| [c1dc59e99108f24b041d37fb4de8ae5da75e135c](https://github.com/apache/tomcat/commit/c1dc59e99108f24b041d37fb4de8ae5da75e135c) | Further async error handling changes | 7 | 6 |
| [fd58f3a281292fced855453a85aab149e73f063e](https://github.com/apache/tomcat/commit/fd58f3a281292fced855453a85aab149e73f063e) | Correct when async-started state remains true | 3 | 1 |
| [57b2c6bd0f083588bfedc53994273e666b7de16e](https://github.com/apache/tomcat/commit/57b2c6bd0f083588bfedc53994273e666b7de16e) | Follow-up correction for that state fix | 2 | 1 |

The first two repeatedly involve `AsyncStateMachine` and `AsyncContextImpl`.
The second also touches `CoyoteAdapter`, `ActionCode`, `AbstractAjpProcessor`
and `AbstractHttp11Processor`. This is a lead for reviewing how async rules
cross container and protocol boundaries. The last two are useful controls:
tests and a changelog increase file counts without spreading production logic.

**Limits:** these Git observations concern the class and async subsystem.
`git log -L` over the outer `isAsync()` body found no edits within this bounded
window. The apparent introduction at the shallow boundary is not evidence of
its actual introduction. This branch/window is not established as the paper's
exact input. Do not present these commits as reproduction of the method-level
published example.

**Our result:** unsupported detector on the historical Java snapshot.

## 2. Roommate: booking-day construction

**Evidence:** developer-labelled commit and surrounding history.
**Status:** candidate with substantial normal caller-migration noise.

[team-propra/roommate commit 3f1348ec8bd79dfaabd2ecb17d70059722271dc0](https://github.com/team-propra/roommate/commit/3f1348ec8bd79dfaabd2ecb17d70059722271dc0),
2023-12-19, explicitly mentions fear of shotgun surgery.

It changes seven Java files: three production files and four test/support
files. It moves `createBookingDays(int)` from `RoomDomainService` into
`BookingDays`, restricts constructors, and updates `BookDataForm`, test
factories and tests to use the factory.

Related history:

- [4a6d78e1c03262109a212f100b97c78356d07f39](https://github.com/team-propra/roommate/commit/4a6d78e1c03262109a212f100b97c78356d07f39),
  2023-12-10: booking-day changes spread through domain objects, controller,
  form, persistence, service, template and tests; 19 files changed.
- [141ca1c6d86969059481b54ae9ce3316d0eb2a8b](https://github.com/team-propra/roommate/commit/141ca1c6d86969059481b54ae9ce3316d0eb2a8b),
  2023-12-20: further booking-form changes span application, controller,
  domain, interfaces, persistence, template and tests; 29 files changed.
  Package moves and other edits confound the file count.

**Interpretation:** there is observable change propagation around the booking
representation. The named commit also centralises construction. Neither its
title nor its seven-file footprint proves an unresolved smell: ordinary
interface migration requires updating callers. Keep this as a review/control
case rather than a confirmed positive.

Cloned all available history: 636 commits reachable from fetched refs at
inspection. Clone HEAD was `07a7c0422c264edfe4cd921c77c5b4d109d0ac66`.
The detector input was the production Java tree at the labelled commit's
parent, `604bb07ea52692bd84bdf330c5365e77f45c5d59`.

**Our result:** unsupported detector on that before snapshot.

## 3. Clean Code School: duplicated minimum-balance policy

**Evidence:** explicit public before/after teaching example.
**Status:** useful semantic example, not longitudinal evidence.

[guidesmiths/clean-code-school commit e88f760414faac08e1f91b211c37f4daad4b4211](https://github.com/guidesmiths/clean-code-school/commit/e88f760414faac08e1f91b211c37f4daad4b4211),
2019-11-18, adds `code-smells/shotgun-surgery/before.js` and `after.js`.

In `before.js`, `transfer` and `sendWarningMessage` independently compare the
balance with 500. Changing the minimum-balance policy requires editing both.
`after.js` illustrates extracting the decision into `isUnderBudget`.

**Cautions:** both versions arrive in one commit, so Git co-change repetition
cannot establish the smell. The after example also uses unresolved names
(`amount` and `creditAmount`) and changes the transfer operation. It is not
verified as a runnable, behaviour-preserving refactoring pair.

Cloned all available history: 136 commits reachable from fetched refs.
Clone HEAD was `b7791a5e41cc6b70937efc8857c35de5db0ab549`; inspected and
scanned the files at the introducing commit above.

**Our result:** JavaScript unsupported; no smell determination.

## 4. SmellBench: grading, sensor batteries and bank accounts

**Evidence:** public annotated teaching corpus already bundled in this repo.
**Status:** immediate Java examples and proposed before/after controls.

The corpus comes through [StaticNose](https://github.com/refactoring-assistant/StaticNose).
Local explanations: [shotgun README](../tests/fixtures/smellbench/main/java/shotgun/README.md).

- **Grading:** `case1/SSBE1.java` repeats grading decisions in subject classes;
  `SSGE1.java` moves the decision to the grade owner.
- **Sensor batteries:** `case2/SSBE2.java` spreads battery concerns among sensor
  types; `SSGE2.java` illustrates centralising the shared data and decisions.
- **Bank accounts:** `case3/SSBE3.java` duplicates account printing across
  `CheckProcessor` and `DirectDepositProcessor`, so changing printed fields
  affects both; `SSGE3.java` moves `printAccountDetails` into `BankAccount`.

The existing saved StaticNose results in
`tests/fixtures/golden/smellbench.json` contain these shotgun findings:

| File | Findings |
| --- | ---: |
| `case1/SSBE1.java` | 2 |
| `case1/SSGE1.java` | 2 |
| `case2/SSBE2.java` | 1 |
| `case2/SSGE2.java` | 3 |
| `case3/SSBE3.java` | 4 |
| `case3/SSGE3.java` | 0 |

This is inspection of saved reference output, not a fresh StaticNose run.
For example, the reference flags `SubjectsGradingGood`, `BatteryGood` and
`SensorDataGood`. The conflict with intended after controls needs semantic
review. Matching this golden would demonstrate parity, not validated
discrimination of shotgun surgery.

**Our result:** unsupported detector on all six Java files.

## 5. Codesai: indexing and canonical URL rules

**Evidence:** practitioners' account of a real client case.
**Status:** public explanation; no reproducible repository identified in the post.

[A case of Shotgun Surgery](https://codesai.com/posts/2025/02/a-case-of-shotgun-surgery),
Fran Reyes and Manuel Rivero, 2025-02-03 (translation of a 2022 post).
Indexing and canonical URL rules leaked across `IndexingCalculator` and
`CanonicalCalculator`, so modifying either concern required considering both
implementations. Client code also owned an untested execution order. They
introduced `PageSEO` to make sequencing explicit and testable, then moved the
rules to their owners.

**Our result:** not run; the article supplies no checkout/commit for this case.
Do not manufacture a fixture and describe it as a run on the original system.

## More independent benchmark leads

The downloaded HIST oracle also has nonempty files for Aardvark CMS, Eclipse,
jEdit, Android frameworks-base and Android frameworks-support. These are
**discovered labels, not cases independently checked in Git or scanned here**.
Examples include `ContentManagerImpl` (Aardvark), `ProgressCounter` (Eclipse),
and associations involving `FragmentPagerAdapter.instantiateItem` (Android
support). Preserve the raw oracle schema: some records are class names and
others are pairs of methods; rows are not interchangeable counts of smells.

Package provenance: figshare article `1157374`, file `1684507`,
`Replication Package.zip`, 7,807,175 bytes; published MD5
`c467a08b90936ebf8dc3fc84a89ad160`.
The downloaded archive's MD5 matched that value.

## Repeating the checks

These commands inspect public source as data; none builds or runs code from
the downloaded repositories. Use fresh destination directories for clones.

```sh
cargo build --locked
git clone --single-branch --branch 7.0.39 --depth 500 \
  https://github.com/apache/tomcat.git /tmp/shotgun-tomcat
git -C /tmp/shotgun-tomcat log --no-merges --format=fuller --stat -- \
  java/org/apache/coyote/AsyncStateMachine.java
git -C /tmp/shotgun-tomcat show cf4fae533cb303c97031b68ec652d6624207df21
git -C /tmp/shotgun-tomcat show c1dc59e99108f24b041d37fb4de8ae5da75e135c
target/debug/bluecheese -f /tmp/shotgun-tomcat/java -s shotgun
target/debug/bluecheese -f tests/fixtures/smellbench/main/java/shotgun -s shotgun

git clone https://github.com/team-propra/roommate.git /tmp/shotgun-roommate
git -C /tmp/shotgun-roommate show 3f1348ec8bd79dfaabd2ecb17d70059722271dc0
mkdir -p /tmp/shotgun-roommate-before
git -C /tmp/shotgun-roommate archive \
  604bb07ea52692bd84bdf330c5365e77f45c5d59 src/main/java |
  tar -x -C /tmp/shotgun-roommate-before
target/debug/bluecheese -f /tmp/shotgun-roommate-before -s shotgun

git clone https://github.com/guidesmiths/clean-code-school.git /tmp/shotgun-school
git -C /tmp/shotgun-school show e88f760414faac08e1f91b211c37f4daad4b4211
```

All four scan attempts above were observed failing with exit 1 for the stated
coverage reasons. Only documentation is added here; no runtime behaviour or
tests changed. Foreign projects' tests and a fresh StaticNose run were not
run. Git investigation was targeted, not an exhaustive or random survey of
public repositories.

For a future history detector, retain production-code spread separately from
tests/docs/generated files, exclude merges and flag bulk edits, report the
history boundary, and inspect diffs for a shared reason to change. Evaluate
against independently labelled positives and appropriate negative controls.
Repeated co-change is a candidate signal; responsibility analysis is still
needed to judge whether that coupling is harmful.
