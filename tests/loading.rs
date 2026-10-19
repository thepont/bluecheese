//! Tests for what the loader picks up, prunes, and leaves behind.
//!
//! These exist because a partial analysis is indistinguishable from a clean one
//! unless the loader accounts for what it did not read.

use std::path::PathBuf;

use bluecheese::lang::load_project;

fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

#[test]
fn prunes_dependency_directories() {
    let project = load_project(&fixtures().join("unsupported")).expect("load fixture");

    let loaded: Vec<&str> = project.files.iter().map(|f| f.rel.as_str()).collect();
    assert_eq!(loaded, ["real.rs"], "vendor/ must not be visited at all");
}

#[test]
fn accounts_for_files_no_adapter_understands() {
    let project = load_project(&fixtures().join("unsupported")).expect("load fixture");

    assert_eq!(project.skipped.get("py"), Some(&1));
    // Anything inside a pruned directory is never seen, so it is not reported
    // as skipped either.
    assert_eq!(project.skipped.len(), 1, "skipped: {:?}", project.skipped);
}
