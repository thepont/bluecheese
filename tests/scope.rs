//! What a scan is narrowed to, on top of what the project itself declares.
//!
//! `.gitignore` answers "what is not source", and `loading.rs` pins that. It
//! cannot answer "what is source but should not be analysed here": a checked-in
//! fixture full of deliberate smells is tracked, and no ignore file will ever
//! exclude it. `Scope` is that second answer, and these tests pin both
//! directions of it.

mod common;

use bluecheese::lang::{Scope, load_project_with};
use common::TempTree;

/// A Rust function well over the long-method limit, so that whether a file was
/// analysed is directly observable as whether it produced a finding.
fn smelly_fn(name: &str) -> String {
    let mut source = format!("pub fn {name}() -> i32 {{\n    let mut total = 0;\n");
    for step in 1..=31 {
        source.push_str(&format!("    total += {step};\n"));
    }
    source.push_str("    total\n}\n");
    source
}

/// The paths a scan picked up, in the order the loader sorts them.
fn analysed(tree: &TempTree, scope: Scope) -> Vec<String> {
    load_project_with(tree.path(), &scope)
        .expect("load temp tree")
        .files
        .into_iter()
        .map(|file| file.rel)
        .collect()
}

/// Two files that are each smelly, so which one was read is visible.
fn two_files(name: &str) -> TempTree {
    let tree = TempTree::new(name);
    tree.write("src/a.rs", &smelly_fn("a"))
        .write("fixtures/b.rs", &smelly_fn("b"));
    tree
}

#[test]
fn without_a_scope_every_source_file_is_read() {
    let tree = two_files("scope-none");

    assert_eq!(analysed(&tree, Scope::default()), ["fixtures/b.rs", "src/a.rs"]);
}

#[test]
fn excluding_a_directory_drops_everything_under_it() {
    let tree = two_files("scope-exclude");

    let scope = Scope {
        exclude: vec!["fixtures/".to_string()],
        ..Scope::default()
    };

    assert_eq!(analysed(&tree, scope), ["src/a.rs"]);
}

#[test]
fn including_a_pattern_is_a_whitelist() {
    let tree = two_files("scope-include");

    let scope = Scope {
        include: vec!["fixtures/**".to_string()],
        ..Scope::default()
    };

    // Not an addition to the default set: the moment a whitelist exists, a file
    // matching nothing in it is left out, which is why the CLI documents
    // `--include` as taking precedence over `--exclude`.
    assert_eq!(analysed(&tree, scope), ["fixtures/b.rs"]);
}

#[test]
fn an_exclude_narrows_a_whitelist_rather_than_widening_it() {
    let tree = two_files("scope-narrow");

    let scope = Scope {
        include: vec!["**/*.rs".to_string()],
        exclude: vec!["fixtures/".to_string()],
    };

    assert_eq!(analysed(&tree, scope), ["src/a.rs"]);
}

#[test]
fn an_include_has_to_name_the_directory_it_wants_to_reach() {
    let tree = TempTree::new("scope-rescue");
    tree.write(".gitignore", "vendor/\n")
        .write("kept.rs", "pub fn kept() {}\n")
        .write("vendor/patched.rs", &smelly_fn("patched"));

    assert_eq!(
        analysed(&tree, Scope::default()),
        ["kept.rs"],
        "the project says vendor/ is not its source"
    );

    // A whitelist naming only the *contents* does not match the `vendor`
    // directory entry itself. Unmatched under a whitelist means ignored, the
    // walk never descends, and the include looks as if it did nothing.
    let contents_only = Scope {
        include: vec!["vendor/**".to_string()],
        ..Scope::default()
    };
    assert_eq!(analysed(&tree, contents_only), Vec::<String>::new());

    // Naming the directory as well is what lets the walk reach inside it, and
    // then the include does outrank the project's own ignore file.
    let directory_and_contents = Scope {
        include: vec!["vendor/".to_string(), "vendor/**".to_string()],
        ..Scope::default()
    };
    assert_eq!(
        analysed(&tree, directory_and_contents),
        ["vendor/patched.rs"]
    );
}

#[test]
fn the_ignore_file_is_honoured_without_a_repository() {
    // `.ignore` is the tool-agnostic twin of `.gitignore`, and is read whether
    // or not the tree is a git checkout at all.
    let tree = TempTree::new("scope-ignore");
    tree.write(".ignore", "generated/\n")
        .write("kept.rs", "pub fn kept() {}\n")
        .write("generated/dropped.rs", &smelly_fn("dropped"));

    assert_eq!(analysed(&tree, Scope::default()), ["kept.rs"]);
}

#[test]
fn a_malformed_pattern_is_rejected_with_the_pattern_named() {
    let tree = two_files("scope-malformed");

    let Err(error) = load_project_with(
        tree.path(),
        &Scope {
            exclude: vec!["[".to_string()],
            ..Scope::default()
        },
    ) else {
        panic!("an unclosed character class is not a pattern");
    };

    let message = format!("{error:#}");
    assert!(
        message.contains("invalid exclude pattern '['"),
        "the error must name the offending pattern; it was: {message}"
    );
}
