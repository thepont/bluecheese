//! The language abstraction.
//!
//! Detectors speak in *concepts* — "the methods of this type", "the statements
//! of this body" — never in a grammar's node names. Each language provides a
//! [`LanguageModel`] that maps its own syntax onto those concepts, so adding a
//! language never touches a detector.
//!
//! A file is handled by exactly one adapter, chosen by extension. A directory
//! holding several languages is therefore partitioned naturally: each file is
//! analysed by the model that claims it. Nothing is ever resolved *between*
//! adapters, which keeps a mixed project from inventing relationships that do
//! not exist in either language.

pub mod java;
pub mod rust;

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use tree_sitter::{Language, Node, Parser, Tree};

/// A language the tool can analyse.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LanguageId {
    Java,
    Rust,
}

/// A parsed source file, tagged with the adapter that parsed it.
pub struct SourceFile {
    pub path: PathBuf,
    /// Path relative to the project root, with `/` separators, matching the
    /// Java tool's `relativeFilePath`.
    pub rel: String,
    pub source: String,
    pub tree: Tree,
    pub language: LanguageId,
}

impl SourceFile {
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

/// Maps one language's syntax onto the concepts the detectors use.
///
/// The vocabulary is deliberately small: it is what the ported detectors
/// actually need, and it grows only when a detector needs a new concept.
pub trait LanguageModel {
    fn id(&self) -> LanguageId;

    /// File extensions this adapter claims, without the leading dot.
    fn extensions(&self) -> &'static [&'static str];

    fn grammar(&self) -> Language;

    /// Top-level types declared in the file.
    fn types<'a>(&self, file: &'a SourceFile) -> Vec<Node<'a>>;

    /// Simple name of a type declaration.
    fn type_name<'a>(&self, file: &'a SourceFile, ty: Node<'a>) -> Option<&'a str>;

    /// Methods belonging to `ty`, as the language defines that relationship.
    /// Constructors are not methods.
    ///
    /// The file is passed because a language may not keep methods inside the
    /// type declaration (Rust puts them in `impl` blocks elsewhere in the file).
    fn methods<'a>(&self, file: &'a SourceFile, ty: Node<'a>) -> Vec<Node<'a>>;

    /// The body of a method, if it has one.
    fn body<'a>(&self, node: Node<'a>) -> Option<Node<'a>>;

    /// Line a declaration starts on, as the reference tool reports it.
    fn decl_line(&self, node: Node<'_>) -> usize;

    /// Logical lines of code in a method body.
    fn lloc(&self, file: &SourceFile, body: Node<'_>) -> usize;

    /// Cyclomatic complexity of a method.
    fn complexity(&self, file: &SourceFile, method: Node<'_>) -> usize;
}

/// All adapters the tool knows about.
fn models() -> [&'static dyn LanguageModel; 2] {
    [&java::JAVA, &rust::RUST]
}

pub fn model_for(id: LanguageId) -> &'static dyn LanguageModel {
    match id {
        LanguageId::Java => &java::JAVA,
        LanguageId::Rust => &rust::RUST,
    }
}

/// The adapter that claims `path`, if any. This is the per-file factory.
pub fn model_for_path(path: &Path) -> Option<&'static dyn LanguageModel> {
    let extension = path.extension().and_then(|e| e.to_str())?;
    models()
        .into_iter()
        .find(|model| model.extensions().contains(&extension))
}

/// Every file extension the tool understands, for diagnostics.
pub fn supported_extensions() -> Vec<&'static str> {
    models()
        .into_iter()
        .flat_map(|model| model.extensions().iter().copied())
        .collect()
}

/// Load and parse every source file under `root` that a known adapter claims,
/// sorted by path for determinism.
///
/// Files no adapter claims are skipped, so a mixed or partly-irrelevant tree can
/// be pointed at safely. Hidden directories and `target` are skipped: build
/// output can contain generated source that is not the project's own. Parsers
/// are reused per language.
pub fn load_project(root: &Path) -> Result<Vec<SourceFile>> {
    let walker = walkdir::WalkDir::new(root).into_iter().filter_entry(|e| {
        if e.depth() == 0 {
            return true;
        }
        let name = e.file_name().to_string_lossy();
        !(e.file_type().is_dir() && (name == "target" || name.starts_with('.')))
    });

    let mut paths: Vec<PathBuf> = walker
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.into_path())
        .collect();
    paths.sort();

    let mut parsers: HashMap<LanguageId, Parser> = HashMap::new();
    let mut files = Vec::new();

    for path in paths {
        let Some(model) = model_for_path(&path) else {
            continue;
        };

        if let Entry::Vacant(slot) = parsers.entry(model.id()) {
            let mut parser = Parser::new();
            parser
                .set_language(&model.grammar())
                .with_context(|| format!("loading the {:?} grammar", model.id()))?;
            slot.insert(parser);
        }
        let parser = parsers.get_mut(&model.id()).expect("parser just inserted");

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

        files.push(SourceFile {
            path,
            rel,
            source,
            tree,
            language: model.id(),
        });
    }

    Ok(files)
}
