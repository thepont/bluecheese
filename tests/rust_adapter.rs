//! Characterisation tests for the Rust adapter.
//!
//! There is no reference implementation for Rust smells: SmellBench and the
//! Java tool it feeds are Java-only. Unlike `parity.rs`, these expectations are
//! reasoned by hand from the fixture source rather than proven against another
//! tool. They lock in behaviour; they are not evidence of correctness.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use bluecheese::finding::Finding;
use bluecheese::lang::{LanguageId, load_project, model_for};

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

fn keys(findings: &[Finding]) -> BTreeSet<(String, String, Vec<usize>)> {
    findings
        .iter()
        .map(|f| {
            (
                f.relative_file_path.clone(),
                f.class_name.clone(),
                f.line_numbers.clone(),
            )
        })
        .collect()
}

/// `Analyzer::accumulate` is the only method over the threshold: 33 logical
/// lines against a limit of 30, complexity 14 against 15, so it is the line
/// count driving the finding and not the complexity.
#[test]
fn finds_the_long_method_and_nothing_else() {
    let files = load_project(&fixture_root().join("rust"))
        .expect("load rust fixture")
        .files;
    let findings = bluecheese::detect(&files, "long-method");

    let expected: BTreeSet<(String, String, Vec<usize>)> =
        [("sample.rs".to_string(), "Analyzer".to_string(), vec![18])]
            .into_iter()
            .collect();

    assert_eq!(keys(&findings), expected);
}

/// Rust methods live in `impl` blocks, and free functions belong to no type, so
/// they are reported under a pseudo-type named after the file.
#[test]
fn maps_impl_blocks_and_free_functions() {
    let files = load_project(&fixture_root().join("rust"))
        .expect("load rust fixture")
        .files;
    let file = files
        .iter()
        .find(|f| f.rel == "sample.rs")
        .expect("sample.rs is loaded");

    let model = model_for(file.language);
    let mut by_type: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for ty in model.types(file) {
        let name = model
            .type_name(file, ty)
            .expect("type has a name")
            .to_string();
        let methods = model
            .methods(file, ty)
            .into_iter()
            .map(|m| {
                let name = m.child_by_field_name("name").expect("method has a name");
                file.text(name).to_string()
            })
            .collect();
        by_type.insert(name, methods);
    }

    assert_eq!(
        by_type.get("Analyzer"),
        Some(&vec![
            "new".to_string(),
            "total".to_string(),
            "accumulate".to_string()
        ])
    );
    assert_eq!(
        by_type.get("sample"),
        Some(&vec!["free_short".to_string(), "classify".to_string()])
    );
    assert_eq!(by_type.get("Status"), Some(&Vec::new()));
}

/// A wildcard arm is the fall-through case, not a decision, so it must not
/// raise complexity. `classify` has two real arms plus a wildcard: 1 + 2 = 3.
/// This bug was found by hand-checking `pick_output_buffer_size` in a real
/// codebase, where the wildcard arm inflated the count.
#[test]
fn wildcard_match_arm_is_not_a_decision() {
    let files = load_project(&fixture_root().join("rust"))
        .expect("load rust fixture")
        .files;
    let file = files
        .iter()
        .find(|f| f.rel == "sample.rs")
        .expect("sample.rs is loaded");
    let model = model_for(file.language);

    let mut classified = None;
    for ty in model.types(file) {
        for method in model.methods(file, ty) {
            let name = method
                .child_by_field_name("name")
                .expect("method has a name");
            if file.text(name) == "classify" {
                classified = Some(model.complexity(file, method));
            }
        }
    }

    assert_eq!(classified, Some(3), "1 base + two non-wildcard arms");
}

/// A tree holding several languages is partitioned: every file is handled by
/// the adapter that claims it, and findings never cross over.
#[test]
fn partitions_a_mixed_tree_by_language() {
    let files = load_project(&fixture_root())
        .expect("load mixed fixture tree")
        .files;
    assert!(files.iter().any(|f| f.language == LanguageId::Java));
    assert!(files.iter().any(|f| f.language == LanguageId::Rust));

    let findings = bluecheese::detect(&files, "long-method");
    let rust: BTreeSet<_> = keys(&findings)
        .into_iter()
        .filter(|(path, _, _)| path.ends_with(".rs"))
        .collect();
    let java: BTreeSet<_> = keys(&findings)
        .into_iter()
        .filter(|(path, _, _)| path.ends_with(".java"))
        .collect();

    // The Rust side is exactly the sample; the Java side is the full SmellBench
    // set, which `parity.rs` already checks against the golden report.
    assert_eq!(rust.len(), 1);
    assert!(
        rust.iter()
            .all(|(path, class, _)| path == "rust/sample.rs" && class == "Analyzer")
    );
    assert_eq!(java.len(), 12);
    assert!(
        java.iter()
            .all(|(path, _, _)| path.starts_with("smellbench/"))
    );
}
