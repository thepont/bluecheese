//! Invariants that hold for every fixture in the corpus, whatever smell it is
//! there to pin.
//!
//! A fixture is evidence, and evidence the grammar only half-understood is
//! evidence about something else. tree-sitter recovers from a syntax error by
//! inventing nodes, which is what makes it usable on real source and dangerous
//! here: a fixture that does not parse still produces a tree, and the metrics
//! then measure the recovery rather than the code. Nothing else in the suite
//! would notice — every corpus test would still pass, against the wrong tree.

mod common;

use bluecheese::lang::SourceFile;
use bluecheese::walk::descendants;

/// Every place the grammar could not read `file`, as `kind at line n`.
fn parse_errors(file: &SourceFile) -> Vec<String> {
    descendants(file.root())
        .into_iter()
        .filter(|node| node.is_error() || node.is_missing())
        .map(|node| format!("{} at line {}", node.kind(), node.start_position().row + 1))
        .collect()
}

#[test]
fn every_fixture_parses_into_the_grammar_it_claims() {
    // `smellbench/` and `rust-broken/` are out of scope on purpose: the first is a
    // third party's corpus, which this project does not get to edit, and the
    // second exists to be malformed.
    let mut broken = Vec::new();
    for dir in ["smelly", "refactored"] {
        for file in common::load(dir).files {
            for problem in parse_errors(&file) {
                broken.push(format!("{dir}/{}: {problem}", file.rel));
            }
        }
    }

    assert!(
        broken.is_empty(),
        "a fixture the grammar could not read as written: {broken:#?}"
    );
}

#[test]
fn a_file_the_grammar_cannot_read_is_reported_as_one() {
    // The corpus is clean, so the test above cannot show that it is able to fail.
    // This is the same check against source that is broken on purpose, in both
    // grammars, so a walk that quietly found nothing would be caught here.
    let rust = common::parse("broken.rs", "pub fn broken( { let x = ; }\n");
    let java = common::parse("Broken.java", "class Broken { void f( { int x = ; } }\n");

    assert!(
        !parse_errors(&rust).is_empty(),
        "a Rust function with no parameter list must not read as clean"
    );
    assert!(
        !parse_errors(&java).is_empty(),
        "a Java method with no parameter list must not read as clean"
    );
}
