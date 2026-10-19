# BlueCheese findings in Jumper

Inventory refreshed 2026-10-19 with the current `/home/pont/projects/bluecheese` build.

**Open findings: 40 Long Method detector records across 30 Rust files.**

BlueCheese currently catalogues all 22 Fowler chapter entries. Its metadata marks 19 as present in the reference tool, but the Rust implementation dispatches only `long-method`; the other 18 reference-supported smells are not implemented yet. The catalogue entries are names, not detector coverage. The current binary timestamp is newer than `src/smells.rs`, so rebuilding does not add those missing detectors. BlueCheese tests pass (all crate, CLI, corpus, parity, and Rust adapter tests).

These are review candidates. Confirm responsibility and existing tests, then make behavior-preserving refactors one at a time, test each, and commit each isolated change. This scan excluded `target/**`, `vendor/**`, and `rl/**`.

## Findings

### `deploy/fsm/build.rs`

- [ ] Line 111: `build` — Long Method

### `deploy/fsm/examples/check_bundle.rs`

- [ ] Line 25: `check_bundle` — Long Method

### `deploy/fsm/src/bin/controller.rs`

- [ ] Line 131: `controller` — Long Method

### `deploy/fsm/src/bundle.rs`

- [ ] Line 114: `Bundle` — Long Method

### `deploy/fsm/src/config.rs`

- [ ] Line 681: `FsmConfig` — Long Method
- [ ] Line 568: `Error` — Long Method

### `deploy/fsm/src/control.rs`

- [ ] Line 287, 495, 656: `Controller` — Long Method
- [ ] Line 2684: `Buttons` — Long Method

### `deploy/fsm/src/dds.rs`

- [ ] Line 148: `DdsIo` — Long Method

### `deploy/fsm/src/joint_motion.rs`

- [ ] Line 222, 313: `JointMotion` — Long Method

### `deploy/fsm/src/obs.rs`

- [ ] Line 409, 629, 803: `ObservationBuilder` — Long Method

### `deploy/fsm/src/operator.rs`

- [ ] Line 314, 567, 717: `OperatorSpec` — Long Method
- [ ] Line 998, 1126: `Operator` — Long Method

### `deploy/fsm/src/py.rs`

- [ ] Line 1037, 1162: `py` — Long Method
- [ ] Line 792: `PySceneMotion` — Long Method
- [ ] Line 912: `PyTracking` — Long Method
- [ ] Line 1095: `PyCapture` — Long Method

### `deploy/fsm/src/py/wbc.rs`

- [ ] Line 696: `PyWbcController` — Long Method

### `deploy/fsm/src/reference.rs`

- [ ] Line 231: `Reference` — Long Method

### `deploy/fsm/src/rknn.rs`

- [ ] Line 217: `RknnEngine` — Long Method

### `deploy/fsm/src/tracking.rs`

- [ ] Line 146, 281: `Tracking` — Long Method

### `deploy/fsm/tests/realtime_allocations.rs`

- [ ] Line 110: `realtime_allocations` — Long Method

### `deploy/native-sim/src/contacts.rs`

- [ ] Line 184: `ContactWorkspace` — Long Method

### `deploy/native-sim/src/dynamics.rs`

- [ ] Line 265: `DynamicsRuntime` — Long Method
- [ ] Line 437, 594: `DynamicsReader` — Long Method

### `deploy/native-sim/src/video.rs`

- [ ] Line 82: `Video` — Long Method

### `deploy/native-sim/tests/actuation.rs`

- [ ] Line 274: `actuation` — Long Method

### `deploy/native-sim/tests/contacts.rs`

- [ ] Line 21, 101, 143, 211, 258, 327, 390: `contacts` — Long Method

### `deploy/native-sim/tests/dynamics.rs`

- [ ] Line 71, 121, 195, 454, 618, 1113, 1244: `dynamics` — Long Method

### `deploy/native-sim/tests/friction.rs`

- [ ] Line 22, 418: `friction` — Long Method

### `deploy/native-sim/tests/integration.rs`

- [ ] Line 424, 496: `integration` — Long Method

### `deploy/native-sim/tests/physics.rs`

- [ ] Line 208: `physics` — Long Method

### `deploy/wbc-contact-adapter/src/lib.rs`

- [ ] Line 149: `lib` — Long Method

### `deploy/wbc-contact-adapter/src/state_constraints.rs`

- [ ] Line 73, 186, 395: `StateConstraintWorkspace` — Long Method

### `deploy/wbc-contact-adapter/tests/contract.rs`

- [ ] Line 303, 859, 1480, 1630, 1967: `contract` — Long Method

### `deploy/whole-body-control/src/lib.rs`

- [ ] Line 614: `lib` — Long Method
- [ ] Line 290, 377: `Model` — Long Method
- [ ] Line 435: `Problem` — Long Method
- [ ] Line 1206: `Controller` — Long Method

### `deploy/whole-body-control/tests/allocations.rs`

- [ ] Line 663: `allocations` — Long Method
