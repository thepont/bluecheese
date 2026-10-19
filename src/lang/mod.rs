//! The language abstraction.
//!
//! Detectors speak in *concepts* — "the methods of this type", "the statements
//! of this body" — never in a grammar's node names. Each language provides a
//! [`LanguageModel`] that maps its own syntax onto those concepts, so adding a
//! language never touches a detector.
//!
//! A file is handled by exactly one adapter, chosen by extension. A directory
//! holding several languages is therefore partitioned naturally: each file is
//! analysed by the model that claims it. Call resolution stays inside compatible
//! language families: JavaScript and TypeScript share an ES-module resolver,
//! while Java and Rust do not acquire call edges just from matching names.

mod accessors;
pub mod coupling;
pub mod ecmascript;
pub mod java;
pub mod rust;
pub mod rust_calls;
pub mod structure;

use std::collections::hash_map::Entry;
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use tree_sitter::{Language, Node, Parser, Tree};

/// A language the tool can analyse.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LanguageId {
    Java,
    Rust,
    JavaScript,
    TypeScript,
    Tsx,
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

/// A loaded project: the files an adapter claimed, and an account of the files
/// that were left out because no adapter understands them.
pub struct Project {
    pub files: Vec<SourceFile>,
    /// Skipped files by extension. Surfaced because an empty report from a
    /// working analyser and an empty report from an analyser that understood
    /// nothing are otherwise the same bytes: a tree of JavaScript analysed by a
    /// Rust-only tool must not look like clean code.
    pub skipped: BTreeMap<String, usize>,
}

/// Maps one language's syntax onto the concepts the detectors use.
///
/// The vocabulary is deliberately small: it is what the ported detectors
/// actually need, and it grows only when a detector needs a new concept.
pub trait LanguageModel {
    fn accessor_chain_depth(&self, file: &SourceFile, node: Node<'_>) -> usize;

    fn is_comment(&self, node: Node<'_>) -> bool;

    /// Adapter-resolved coupling. Languages without a resolver return no
    /// relationships; adapters must never invent cross-language call edges.
    fn method_coupling(&self, _files: &[SourceFile]) -> Vec<coupling::MethodCoupling> {
        Vec::new()
    }

    fn id(&self) -> LanguageId;

    /// File extensions this adapter claims, without the leading dot.
    fn extensions(&self) -> &'static [&'static str];

    fn grammar(&self) -> Language;

    /// Top-level types declared in the file.
    fn types<'a>(&self, file: &'a SourceFile) -> Vec<Node<'a>>;

    /// Owners included in comparisons between methods of the same type.
    fn method_comparison_owners<'a>(&self, file: &'a SourceFile) -> Vec<Node<'a>> {
        self.types(file)
    }

    /// Simple name of a type declaration.
    fn type_name<'a>(&self, file: &'a SourceFile, ty: Node<'a>) -> Option<&'a str>;

    /// Methods belonging to `ty`, as the language defines that relationship.
    /// Constructors are not methods.
    ///
    /// The file is passed because a language may not keep methods inside the
    /// type declaration (Rust puts them in `impl` blocks elsewhere in the file).
    fn methods<'a>(&self, file: &'a SourceFile, ty: Node<'a>) -> Vec<Node<'a>>;

    /// Constructors belonging to `ty`. A constructor is not a method, and the
    /// reference tool reads them separately, so this is a separate
    /// relationship and not a filter over [`LanguageModel::methods`]. A
    /// language without constructors returns nothing.
    fn constructors<'a>(&self, file: &SourceFile, ty: Node<'a>) -> Vec<Node<'a>>;

    /// How many parameters a caller must supply to this method or constructor.
    ///
    /// An implicit receiver (`this`, or Rust's `self`) is not one of them: it is
    /// not something the caller writes, and a Java method and its Rust
    /// equivalent would otherwise never agree on a count.
    fn parameters(&self, node: Node<'_>) -> usize;

    /// Whether this method only restates a signature it inherited rather than
    /// declaring one of its own. The reference tool leaves overrides out of
    /// Long Parameter List: their parameter list is fixed by the declaration
    /// they override, so the smell, and the fix, belong there.
    fn is_override(&self, file: &SourceFile, node: Node<'_>) -> bool;

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
fn models() -> [&'static dyn LanguageModel; 5] {
    [
        &java::JAVA,
        &rust::RUST,
        &ecmascript::JAVASCRIPT,
        &ecmascript::TYPESCRIPT,
        &ecmascript::TSX,
    ]
}

pub fn model_for(id: LanguageId) -> &'static dyn LanguageModel {
    models()
        .into_iter()
        .find(|model| model.id() == id)
        .expect("every LanguageId has a model")
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

/// Conventional build-output directories, pruned even in a tree that carries no
/// ignore file. The primary mechanism is `.gitignore`, below; this is only a
/// fallback for projects that have declared nothing.
///
/// `vendor` is deliberately *not* listed. A project that ignores `vendor/*` and
/// then re-includes one crate (`!vendor/cpal/`) is stating that crate *is* its
/// source, and a name we guessed must not overrule a declaration like that.
const CONVENTIONAL_OUTPUT_DIRECTORIES: &[&str] = &["target", "node_modules"];

/// Which of a project's files to analyse, on top of what the project itself
/// declares.
///
/// `.gitignore` answers "what is not source". It cannot answer "what is source
/// but not worth analysing here" — a test fixture full of deliberate smells is
/// tracked, and no ignore file will ever exclude it. `exclude` drops such paths
/// and `include` rescues what an ignore file dropped, such as a vendored crate a
/// project patched and kept.
///
/// Patterns use gitignore syntax and are relative to the scanned root. `include`
/// is a whitelist: the moment it holds a pattern, only paths matching it are
/// scanned at all, so `exclude` then narrows that set rather than the whole tree.
///
/// Because a whitelist also *prunes* unmatched directories, an `include` that
/// reaches into a directory the project's ignore file dropped must name that
/// directory as well as its contents: `vendor/**` alone matches nothing but
/// files under `vendor`, never the `vendor` entry the walk asks about first, so
/// the walk never descends and the include appears to do nothing. `vendor/` plus
/// `vendor/**` is the working form, exactly as re-including a directory in
/// gitignore requires naming the directory before its contents.
#[derive(Debug, Default, Clone)]
pub struct Scope {
    pub include: Vec<String>,
    pub exclude: Vec<String>,
}

/// Load and parse every source file under `root` that a known adapter claims,
/// sorted by path for determinism.
///
/// Files no adapter claims are skipped, so a mixed or partly-irrelevant tree can
/// be pointed at safely; the skipped ones are counted rather than dropped
/// silently.
///
/// What counts as the project's source is the project's own declaration: hidden
/// files, and anything `.gitignore` or `.ignore` excludes, are left out, so a
/// checked-out dependency tree or a build directory is never reported as the
/// project's own smells. [`CONVENTIONAL_OUTPUT_DIRECTORIES`] is pruned as well,
/// for trees that carry no ignore file. Use [`load_project_with`] to narrow the
/// scan further.
pub fn load_project(root: &Path) -> Result<Project> {
    load_project_with(root, &Scope::default())
}

/// [`load_project`], narrowed by an explicit [`Scope`].
pub fn load_project_with(root: &Path, scope: &Scope) -> Result<Project> {
    let mut parsers = ParserCache::default();
    let mut files = Vec::new();
    let mut skipped: BTreeMap<String, usize> = BTreeMap::new();

    for path in candidate_paths(root, scope)? {
        let Some(model) = model_for_path(&path) else {
            *skipped.entry(extension_of(&path)).or_default() += 1;
            continue;
        };
        let parser = parsers.get(model)?;
        let (source, tree) = read_and_parse(&path, parser)?;
        files.push(SourceFile {
            rel: relative_path(root, &path),
            path,
            source,
            tree,
            language: model.id(),
        });
    }

    Ok(Project { files, skipped })
}

/// Everything under `root` that policy does not prune, sorted for determinism.
///
/// Which files an adapter actually claims is decided later, in
/// [`load_project`]: this step only answers "what counts as this project's
/// source at all?".
///
/// The walker's standard filters are on, so hidden files and anything matched by
/// a `.gitignore` or `.ignore` file are excluded. That is the whole mechanism: a
/// project's own declaration of what is not its source beats any guess of ours.
/// An explicit [`Scope`] then overrides it, in both directions.
fn candidate_paths(root: &Path, scope: &Scope) -> Result<Vec<PathBuf>> {
    let mut overrides = ignore::overrides::OverrideBuilder::new(root);
    for pattern in &scope.exclude {
        // Overrides are inverted relative to gitignore: a plain glob whitelists
        // a path and `!` drops it. The sign therefore flips here, and only here.
        overrides
            .add(&format!("!{pattern}"))
            .with_context(|| format!("invalid exclude pattern '{pattern}'"))?;
    }
    for pattern in &scope.include {
        overrides
            .add(pattern)
            .with_context(|| format!("invalid include pattern '{pattern}'"))?;
    }

    let walker = ignore::WalkBuilder::new(root)
        .overrides(overrides.build()?)
        .filter_entry(|entry| !is_pruned(entry))
        .build();

    let mut paths: Vec<PathBuf> = walker
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_some_and(|kind| kind.is_file()))
        .map(|entry| entry.into_path())
        .collect();
    paths.sort();
    Ok(paths)
}

/// Whether to skip descending into `entry`.
///
/// Hidden directories and ignore files are already handled by the walker; this
/// adds only the conventional fallback.
fn is_pruned(entry: &ignore::DirEntry) -> bool {
    if entry.depth() == 0 || !entry.file_type().is_some_and(|kind| kind.is_dir()) {
        return false;
    }
    let name = entry.file_name().to_string_lossy();
    CONVENTIONAL_OUTPUT_DIRECTORIES.contains(&name.as_ref())
}

/// Parsers are reusable and costly to set up, so keep one per language.
#[derive(Default)]
struct ParserCache {
    parsers: HashMap<LanguageId, Parser>,
}

impl ParserCache {
    fn get(&mut self, model: &'static dyn LanguageModel) -> Result<&mut Parser> {
        if let Entry::Vacant(slot) = self.parsers.entry(model.id()) {
            let mut parser = Parser::new();
            parser
                .set_language(&model.grammar())
                .with_context(|| format!("loading the {:?} grammar", model.id()))?;
            slot.insert(parser);
        }
        Ok(self
            .parsers
            .get_mut(&model.id())
            .expect("parser just inserted"))
    }
}

/// Read and parse one file, giving back what the caller needs to record it.
///
/// Naming the file relative to the project is the caller's business: this only
/// has to turn a path into source and a tree.
fn read_and_parse(path: &Path, parser: &mut Parser) -> Result<(String, Tree)> {
    let source =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let tree = parser
        .parse(&source, None)
        .with_context(|| format!("parsing {}", path.display()))?;

    Ok((source, tree))
}

/// The path as the reference tool reports it: relative to the project root,
/// with `/` separators.
fn relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

/// The extension a skipped file is counted under, for reporting.
fn extension_of(path: &Path) -> String {
    path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("<none>")
        .to_string()
}
