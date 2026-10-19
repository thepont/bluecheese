//! Helpers shared by the integration tests.
//!
//! Each integration test file is its own crate, so anything here that a given
//! test does not use would otherwise warn as dead code.
#![allow(dead_code)]

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use bluecheese::finding::Finding;
use bluecheese::lang::{Project, SourceFile, load_project};

/// The directory of committed fixtures.
pub fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

/// The repository root, for the self-checks that scan this project.
pub fn repo() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Everything an adapter claims under `tests/fixtures/<dir>`.
pub fn load(dir: &str) -> Project {
    load_project(&fixtures().join(dir)).expect("load fixture")
}

/// One file out of a fixture directory, by its path relative to that directory.
pub fn file(dir: &str, name: &str) -> SourceFile {
    load(dir)
        .files
        .into_iter()
        .find(|f| f.rel == name)
        .unwrap_or_else(|| panic!("no fixture {dir}/{name}"))
}

/// Findings as an order-insensitive set of `(path, class, lines)`, the same
/// shape `parity.rs` compares Spoon's report by.
pub fn keys(findings: &[Finding]) -> BTreeSet<(String, String, Vec<usize>)> {
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

/// Findings for one file of a fixture directory, so each example can be checked
/// on its own without an aggregate expectation that every new example would have
/// to rewrite.
pub fn findings_in(dir: &str, name: &str) -> Vec<Finding> {
    let files: Vec<SourceFile> = load(dir)
        .files
        .into_iter()
        .filter(|f| f.rel.ends_with(name))
        .collect();
    assert!(!files.is_empty(), "no fixture matching {dir}/{name}");
    bluecheese::detect(&files, "long-method")
}

/// A throwaway directory tree, so behaviour that depends on ignore files can be
/// exercised without committing files git itself would refuse to track.
pub struct TempTree(PathBuf);

impl TempTree {
    pub fn new(name: &str) -> Self {
        let dir = std::env::temp_dir().join(format!("bluecheese-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("create temp tree");
        Self(dir)
    }

    pub fn write(&self, rel: &str, contents: &str) -> &Self {
        let path = self.0.join(rel);
        std::fs::create_dir_all(path.parent().expect("has a parent")).expect("create directory");
        std::fs::write(path, contents).expect("write file");
        self
    }

    pub fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempTree {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// The 1-based line of the first line containing `needle`.
pub fn line_of(file: &SourceFile, needle: &str) -> usize {
    file.source
        .lines()
        .position(|line| line.contains(needle))
        .unwrap_or_else(|| panic!("no line contains {needle:?}"))
        + 1
}
