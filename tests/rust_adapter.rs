//! Characterisation tests for the Rust adapter.
//!
//! There is no reference implementation for Rust smells: SmellBench and the
//! Java tool it feeds are Java-only. Unlike `parity.rs`, these expectations are
//! reasoned by hand from the fixture source rather than proven against another
//! tool. They lock in behaviour; they are not evidence of correctness.

mod common;

use std::collections::{BTreeMap, BTreeSet};

use bluecheese::lang::{LanguageId, load_project, model_for};
use common::{keys, load};

/// `Analyzer::accumulate` is the only method over the threshold: 33 logical
/// lines against a limit of 30, complexity 14 against 15, so it is the line
/// count driving the finding and not the complexity.
#[test]
fn finds_the_long_method_and_nothing_else() {
    let files = load("rust").files;
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
    let files = load("rust").files;
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
    let files = load("rust").files;
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
    let files = load_project(&common::fixtures())
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

    // Nothing crosses over: every finding is attributed to a file an adapter of
    // the same language claimed, and the two sets together are all of them.
    assert!(
        java.iter().all(|(path, _, _)| path.ends_with(".java")),
        "a Rust file must not produce a Java finding"
    );
    assert!(
        rust.iter().all(|(path, _, _)| path.ends_with(".rs")),
        "a Java file must not produce a Rust finding"
    );
    // The whole Rust corpus is pinned by `smelly_rust.rs`; all this test needs
    // from it is that the sample under `rust/` is still the one finding there.
    let under_rust: BTreeSet<_> = rust
        .iter()
        .filter(|(path, _, _)| path.starts_with("rust/"))
        .cloned()
        .collect();
    let sample: BTreeSet<(String, String, Vec<usize>)> =
        [("rust/sample.rs".to_string(), "Analyzer".to_string(), vec![18])]
            .into_iter()
            .collect();
    assert_eq!(under_rust, sample);
    assert_eq!(
        &java
            .union(&rust)
            .cloned()
            .collect::<BTreeSet<(String, String, Vec<usize>)>>(),
        &keys(&findings),
        "every finding is either Rust or Java; none is produced twice or dropped"
    );
}
