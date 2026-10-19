//! Tests for what the loader picks up, prunes, and leaves behind.
//!
//! These exist because a partial analysis is indistinguishable from a clean one
//! unless the loader accounts for what it did not read.

mod common;

use std::path::PathBuf;

use bluecheese::lang::{LanguageId, load_project};
use common::TempTree;

fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

#[test]
fn prunes_conventional_output_directories() {
    let project = load_project(&fixtures().join("unsupported")).expect("load fixture");

    let loaded: Vec<&str> = project.files.iter().map(|f| f.rel.as_str()).collect();
    assert_eq!(loaded, ["real.rs"], "target/ must not be visited at all");
}

#[test]
fn accounts_for_files_no_adapter_understands() {
    let project = load_project(&fixtures().join("unsupported")).expect("load fixture");

    assert_eq!(project.skipped.get("py"), Some(&1));
    // A file inside a pruned directory is never seen, so it is not reported as
    // skipped either: it was not considered, rather than considered and refused.
    assert_eq!(project.skipped.len(), 1, "skipped: {:?}", project.skipped);
}

#[test]
fn a_file_with_no_extension_is_skipped_under_a_name_of_its_own() {
    // The commonest unreadable files in a real tree are the ones with no
    // extension at all: build files, licences, lock files. They have no
    // extension to report, so they are accounted for together rather than
    // dropped.
    let tree = TempTree::new("extensionless");
    tree.write("Makefile", "all:\n\techo hi\n")
        .write("LICENSE", "all rights reserved\n")
        .write("kept.rs", "pub fn kept() -> i32 {\n    1\n}\n");

    let project = load_project(tree.path()).expect("load temp tree");

    let loaded: Vec<&str> = project.files.iter().map(|f| f.rel.as_str()).collect();
    assert_eq!(loaded, ["kept.rs"]);
    assert_eq!(project.skipped.get("<none>"), Some(&2));
}

#[test]
fn a_file_that_is_not_text_at_all_is_refused_by_name() {
    // The loader reads source as text, so a file of raw bytes cannot be
    // analysed. Failing loudly with the path is the point: a scan that skipped
    // it in silence would report a shorter, cleaner project than the one it was
    // pointed at. The extension has to be one an adapter claims, or the file
    // would be skipped as unsupported before it was ever read.
    let tree = TempTree::new("not-text");
    tree.write("good.rs", "pub fn good() -> i32 {\n    1\n}\n")
        .write_bytes("bad.rs", &[0xff, 0xfe, 0x00, 0x01]);

    let error = match load_project(tree.path()) {
        Err(error) => error,
        Ok(project) => panic!(
            "bytes are not source, but the loader read {} files",
            project.files.len()
        ),
    };
    let message = format!("{error:#}");

    assert!(
        message.contains("bad.rs"),
        "the refusal must name the file; it was: {message}"
    );
}

#[test]
fn respects_the_projects_own_gitignore() {
    let tree = TempTree::new("gitignore");
    tree.write(".gitignore", "generated/\n*.gen.rs\n")
        .write("kept.rs", "pub fn kept() -> i32 {\n    1\n}\n")
        .write("generated/dropped.rs", "pub fn dropped() {}\n")
        .write("also_dropped.gen.rs", "pub fn dropped() {}\n");

    let project = load_project(tree.path()).expect("load temp tree");

    let loaded: Vec<&str> = project.files.iter().map(|f| f.rel.as_str()).collect();
    assert_eq!(loaded, ["kept.rs"]);
    // Ignored files are excluded on purpose, so they must not be conflated with
    // files no adapter understands: only the latter is a gap in coverage.
    assert!(
        project.skipped.is_empty(),
        "an ignored file is not an unsupported one: {:?}",
        project.skipped
    );
    assert_eq!(project.files[0].language, LanguageId::Rust);
}
