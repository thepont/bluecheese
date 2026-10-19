//! Minimal Java front end: load every `.java` file under a directory and parse
//! it with tree-sitter. This is the tree-sitter equivalent of the single
//! whole-project Spoon model the Java tool builds.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use tree_sitter::{Node, Parser, Tree};

pub struct JavaFile {
    pub path: PathBuf,
    /// Path relative to the project root, with `/` separators, matching the
    /// Java tool's `relativeFilePath`.
    pub rel: String,
    pub source: String,
    pub tree: Tree,
}

impl JavaFile {
    pub fn root(&self) -> Node<'_> {
        self.tree.root_node()
    }

    /// 1-based line of a node, matching Spoon's `getPosition().getLine()`.
    pub fn line(&self, node: Node<'_>) -> usize {
        node.start_position().row + 1
    }

    pub fn text(&self, node: Node<'_>) -> &str {
        &self.source[node.byte_range()]
    }
}

/// Load and parse all Java files under `root`, sorted by path for determinism.
pub fn load_project(root: &Path) -> Result<Vec<JavaFile>> {
    let mut paths: Vec<PathBuf> = walkdir::WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.into_path())
        .filter(|p| p.extension().is_some_and(|x| x == "java"))
        .collect();
    paths.sort();

    let mut parser = Parser::new();
    let language = tree_sitter_java::LANGUAGE.into();
    parser
        .set_language(&language)
        .context("failed to load tree-sitter-java grammar")?;

    let mut files = Vec::with_capacity(paths.len());
    for path in paths {
        let source = std::fs::read_to_string(&path)
            .with_context(|| format!("reading {}", path.display()))?;
        let tree = parser
            .parse(&source, None)
            .with_context(|| format!("parsing {}", path.display()))?;
        let rel = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");
        files.push(JavaFile {
            path,
            rel,
            source,
            tree,
        });
    }
    Ok(files)
}
