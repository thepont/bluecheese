//! Parity tests: every ported detector must reproduce the Java tool's findings
//! on the SmellBench corpus exactly.
//!
//! The golden file is the JSON report produced by the reference StaticNose
//! build (see `tests/fixtures/README.md` for how it was generated).

use std::collections::BTreeSet;
use std::path::PathBuf;

use bluecheese::finding::Finding;

fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

fn golden() -> Vec<Finding> {
    let path = fixtures().join("golden/smellbench.json");
    let raw = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("reading {}: {e}", path.display()));
    serde_json::from_str(&raw).expect("golden report is valid JSON")
}

/// Compare one detector's output with the Java golden output for that smell.
fn assert_parity(smell: &str, golden_name: &str) {
    let root = fixtures().join("smellbench");
    let files = bluecheese::lang::load_project(&root)
        .expect("load smellbench fixtures")
        .files;

    let actual: BTreeSet<_> = bluecheese::detect(&files, smell)
        .iter()
        .map(Finding::key)
        .collect();
    let expected: BTreeSet<_> = golden()
        .into_iter()
        .filter(|f| f.smell_name == golden_name)
        .map(|f| f.key())
        .collect();

    let missing: Vec<_> = expected.difference(&actual).collect();
    let extra: Vec<_> = actual.difference(&expected).collect();

    assert!(
        missing.is_empty() && extra.is_empty(),
        "{golden_name}: {} missing, {} extra\n--- missing (Java only) ---\n{missing:#?}\n--- extra (Rust only) ---\n{extra:#?}",
        missing.len(),
        extra.len(),
    );
}

#[test]
fn long_method_matches_java() {
    assert_parity("long-method", "Long Method");
}
