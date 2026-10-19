//! Characterisation tests for the Rust adapter.
//!
//! There is no reference implementation for Rust smells: SmellBench and the
//! Java tool it feeds are Java-only. Unlike `parity.rs`, these expectations are
//! reasoned by hand from the fixture source rather than proven against another
//! tool. They lock in behaviour; they are not evidence of correctness.

mod common;

use std::collections::{BTreeMap, BTreeSet};

use bluecheese::lang::{LanguageId, load_project, model_for};
use common::{TempTree, file, keys, line_of, load};

/// `Analyzer::accumulate` is the only method over the threshold: 33 logical
/// lines against a limit of 30, complexity 14 against 15, so it is the line
/// count driving the finding and not the complexity.
#[test]
fn finds_the_long_method_and_nothing_else() {
    let files = load("rust").files;
    let findings = bluecheese::detect(&files, "long-method").expect("run the detector");

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

    let findings = bluecheese::detect(&files, "long-method").expect("run the detector");
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
    let sample: BTreeSet<(String, String, Vec<usize>)> = [(
        "rust/sample.rs".to_string(),
        "Analyzer".to_string(),
        vec![18],
    )]
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

#[test]
fn a_trait_default_method_belongs_to_the_trait() {
    let features = file("rust-features", "features.rs");
    let model = model_for(features.language);

    let volume = model
        .types(&features)
        .into_iter()
        .find(|ty| model.type_name(&features, *ty) == Some("Volume"))
        .expect("Volume is a type");
    let names: Vec<&str> = model
        .methods(&features, volume)
        .into_iter()
        .map(|method| features.text(method.child_by_field_name("name").expect("a name")))
        .collect();

    // Both are methods of the trait, as an abstract Java method is a method of
    // its interface. Only `doubled` has a body, so only it can be measured: a
    // detector must skip the other rather than treat it as an empty method, and
    // a detector that counted methods would count both.
    assert_eq!(names, ["size", "doubled"]);
}

#[test]
fn attributes_do_not_shift_a_declarations_line() {
    let features = file("rust-features", "features.rs");
    let model = model_for(features.language);

    let krate = model
        .types(&features)
        .into_iter()
        .find(|ty| model.type_name(&features, *ty) == Some("Crate"))
        .expect("Crate is a type");
    let declaration_line = line_of(&features, "pub struct Crate");
    assert_eq!(model.decl_line(krate), declaration_line);
    assert!(
        features
            .source
            .lines()
            .nth(declaration_line - 2)
            .expect("the line above")
            .contains("#[allow"),
        "the attributes sit above the declaration and must not be reported as it"
    );

    // A leaf has nothing but itself to report, and the search for a first token
    // finds no children at all.
    let identifier = bluecheese::walk::descendants(features.root())
        .into_iter()
        .find(|node| node.kind() == "identifier")
        .expect("the fixture has identifiers");
    assert_eq!(model.decl_line(identifier), features.line(identifier));
}

#[test]
fn logical_operators_are_decisions_but_bitwise_is_not() {
    let features = file("rust-features", "features.rs");
    let model = model_for(features.language);

    let mut complexity = None;
    for ty in model.types(&features) {
        for method in model.methods(&features, ty) {
            let name = method.child_by_field_name("name").expect("a name");
            if features.text(name) == "fits" {
                complexity = Some(model.complexity(&features, method));
            }
        }
    }

    // One base, `&&` and `||`: three. The `1 & 2` on the line above is
    // arithmetic, and counting it would inflate every bit-manipulating method.
    assert_eq!(complexity, Some(3));
}

#[test]
fn an_impl_without_a_body_yields_no_methods() {
    // Not valid Rust, and tree-sitter recovers by giving the `impl` a target and
    // no body. Reaching for the body anyway must not panic.
    let broken = file("rust-broken", "impl_without_body.rs");
    let model = model_for(broken.language);

    for ty in model.types(&broken) {
        for method in model.methods(&broken, ty) {
            let name = method.child_by_field_name("name").expect("a name");
            assert_ne!(broken.text(name), "anything", "there is nothing in there");
        }
    }
}

/// A Rust file whose name is not valid UTF-8 has no pseudo-type, because the
/// pseudo-type is the file's stem and a stem that is not text cannot be a name.
/// Its free functions are therefore skipped in silence, which is a known
/// limitation and not a design: the alternative was to fail the whole scan over
/// one badly named file.
///
/// It is pinned because the failure mode — a file that is read, understood, and
/// contributes nothing — looks exactly like clean code. Both detectors that
/// attribute a finding to a type are run over it, since the skip happens before
/// either has a chance to find anything.
#[cfg(unix)]
#[test]
fn a_rust_file_whose_name_is_not_utf8_reports_nothing() {
    let tree = TempTree::new("rust-unreadable");
    tree.write(
        "named.rs",
        "pub fn named(a: i32, b: i32, c: i32, d: i32) -> i32 {\n    a + b\n}\n",
    )
    .write_unreadable_name(
        b"\xff\xfe.rs",
        "pub fn odd(a: i32, b: i32, c: i32, d: i32) -> i32 {\n    a + b\n}\n",
    );

    let files = load_project(tree.path()).expect("load the tree").files;
    assert_eq!(files.len(), 2, "both files are read and parsed");

    for smell in bluecheese::ported_smells() {
        let findings = bluecheese::detect(&files, smell).expect("run a ported detector");
        assert!(
            findings
                .iter()
                .all(|finding| finding.relative_file_path == "named.rs"),
            "only the file with a readable name can be attributed ({smell}): {findings:#?}"
        );
    }

    // And the readable one really is reported, so the check above is not passing
    // because nothing was found anywhere: four parameters is over the limit.
    let findings = bluecheese::detect(&files, "long-params").expect("run the detector");
    assert_eq!(
        findings.len(),
        1,
        "the readable file's four-parameter function is a finding: {findings:#?}"
    );
}
