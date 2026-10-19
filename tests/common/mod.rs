//! Helpers shared by the integration tests.
//!
//! Each integration test file is its own crate, so anything here that a given
//! test does not use would otherwise warn as dead code.
#![allow(dead_code)]

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

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

/// Findings for one file of a fixture directory, as an order-insensitive set of
/// `(path, class, lines)`, so each corpus example can be checked on its own
/// without an aggregate expectation that every new example would have to
/// rewrite.
pub fn finding_keys_in(dir: &str, name: &str) -> BTreeSet<(String, String, Vec<usize>)> {
    let files: Vec<SourceFile> = load(dir)
        .files
        .into_iter()
        .filter(|f| f.rel.ends_with(name))
        .collect();
    assert!(!files.is_empty(), "no fixture matching {dir}/{name}");
    keys(&bluecheese::detect(&files, "long-method").expect("run the long-method detector"))
}

/// How many lines of `source` start, after indentation, with `prefix`.
///
/// The corpus uses this to show that a trap is real: the code really is long,
/// and is still not reported.
pub fn count_lines_starting_with(source: &SourceFile, prefix: &str) -> usize {
    source
        .source
        .lines()
        .filter(|line| line.trim_start().starts_with(prefix))
        .count()
}

/// Counts trees so two parallel tests cannot pick the same directory.
static NEXT_TREE: AtomicUsize = AtomicUsize::new(0);

/// A throwaway directory tree, so behaviour that depends on ignore files can be
/// exercised without committing files git itself would refuse to track.
pub struct TempTree(PathBuf);

impl TempTree {
    pub fn new(name: &str) -> Self {
        // Tests run in parallel within one binary, so the name alone is not
        // unique: two tests wanting the same fixture would share a directory and
        // delete each other's files.
        let serial = NEXT_TREE.fetch_add(1, Ordering::Relaxed);
        let dir =
            std::env::temp_dir().join(format!("bluecheese-{}-{serial}-{name}", std::process::id()));
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

    /// Write a file from raw bytes, for contents that are not text at all.
    pub fn write_bytes(&self, rel: &str, contents: &[u8]) -> &Self {
        let path = self.0.join(rel);
        std::fs::create_dir_all(path.parent().expect("has a parent")).expect("create directory");
        std::fs::write(path, contents).expect("write file");
        self
    }

    /// Write a file whose name is not valid UTF-8, which no `&str` can express.
    #[cfg(unix)]
    pub fn write_unreadable_name(&self, name: &[u8], contents: &str) -> &Self {
        use std::os::unix::ffi::OsStrExt;
        let path = self.0.join(std::ffi::OsStr::from_bytes(name));
        std::fs::write(path, contents).expect("write file");
        self
    }
}

impl Drop for TempTree {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// A parsed single file, built from `source` written to a throwaway tree. The
/// extension of `name` decides which adapter reads it.
pub fn parse(name: &str, source: &str) -> SourceFile {
    let tree = TempTree::new(name);
    tree.write(name, source);
    load_project(tree.path())
        .expect("load the throwaway tree")
        .files
        .pop()
        .expect("the tree holds the one file it was given")
}

/// The 1-based line of the first line containing `needle`.
pub fn line_of(file: &SourceFile, needle: &str) -> usize {
    file.source
        .lines()
        .position(|line| line.contains(needle))
        .unwrap_or_else(|| panic!("no line contains {needle:?}"))
        + 1
}
