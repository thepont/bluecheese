//! Parity tests: every ported detector must reproduce the Java tool's findings
//! on every corpus, exactly.
//!
//! The golden files are JSON reports produced by the reference StaticNose build
//! (see `tests/fixtures/README.md` for how each was generated). A port is
//! finished only when the difference is empty in both directions.
//!
//! These tests are evidence about the port, and nothing else. The corpus tests
//! say what the right answer is and why; these say the Java tool agrees.

use std::collections::BTreeSet;
use std::path::PathBuf;

use bluecheese::finding::Finding;
use bluecheese::lang::{LanguageId, Project, load_project};

fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

fn golden(name: &str) -> Vec<Finding> {
    let path = fixtures().join("golden").join(name);
    let raw = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("reading {}: {e}", path.display()));
    serde_json::from_str(&raw).expect("golden report is valid JSON")
}

/// Compare one detector's output over one corpus with the reference tool's
/// report for that smell.
///
/// Only files this tool claims as Java take part, because Spoon reads nothing
/// else: a Rust fixture cannot be evidence about the Java tool either way, and
/// letting its findings through as "extra" would fail every run for a reason
/// that has nothing to do with the port.
///
/// The golden files carry all 19 smells the Java CLI exposes, so a port that
/// lands later has its evidence already captured.
fn assert_parity(corpus: &str, golden_name: &str, smell: &str, java_smell: &str) {
    let Project { files, .. } = load_project(&fixtures().join(corpus)).expect("load fixtures");
    let java: Vec<_> = files
        .into_iter()
        .filter(|file| file.language == LanguageId::Java)
        .collect();

    let actual: BTreeSet<_> = bluecheese::detect(&java, smell)
        .expect("run a ported detector")
        .iter()
        .map(Finding::key)
        .collect();
    let expected: BTreeSet<_> = golden(golden_name)
        .into_iter()
        .filter(|finding| finding.smell_name == java_smell)
        .map(|finding| finding.key())
        .collect();

    let missing: Vec<_> = expected.difference(&actual).collect();
    let extra: Vec<_> = actual.difference(&expected).collect();

    assert!(
        missing.is_empty() && extra.is_empty(),
        "{corpus}/{java_smell}: {} missing, {} extra\n--- missing (Java only) ---\n{missing:#?}\n--- extra (Rust only) ---\n{extra:#?}",
        missing.len(),
        extra.len(),
    );
}

#[test]
fn long_method_matches_java_on_smellbench() {
    assert_parity(
        "smellbench",
        "smellbench.json",
        "long-method",
        "Long Method",
    );
}

#[test]
fn long_method_matches_java_on_the_smelly_corpus() {
    assert_parity("smelly", "smelly.json", "long-method", "Long Method");
}

#[test]
fn long_method_matches_java_on_the_refactored_corpus() {
    // The interesting half: the Java tool also reports the body that Replace
    // Method with Method Object moved into a new class, and reports it under
    // that class. The pair's claim is verified rather than asserted.
    assert_parity(
        "refactored",
        "refactored.json",
        "long-method",
        "Long Method",
    );
}
