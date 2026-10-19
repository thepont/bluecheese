//! Tests for what the loader picks up, prunes, and leaves behind.
//!
//! These exist because a partial analysis is indistinguishable from a clean one
//! unless the loader accounts for what it did not read.

use std::path::{Path, PathBuf};

use bluecheese::lang::{LanguageId, load_project};

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

/// A throwaway directory tree, so a `.gitignore` can be exercised without
/// committing files git itself would refuse to track.
struct TempTree(PathBuf);

impl TempTree {
    fn new(name: &str) -> Self {
        let dir = std::env::temp_dir().join(format!("bluecheese-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("create temp tree");
        Self(dir)
    }

    fn write(&self, rel: &str, contents: &str) -> &Self {
        let path = self.0.join(rel);
        std::fs::create_dir_all(path.parent().expect("has a parent")).expect("create directory");
        std::fs::write(path, contents).expect("write file");
        self
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempTree {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
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
