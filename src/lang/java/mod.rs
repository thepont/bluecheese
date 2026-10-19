//! Java adapter: maps tree-sitter-java onto the shared concepts.
//!
//! The rules in `metrics` and `types` were reverse-engineered from Spoon rather
//! than guessed, and `tests/parity.rs` holds them to Spoon's own output. See
//! `reference/README.md` for the semantics and how they were discovered.

pub mod calls;
mod metrics;
mod types;

use tree_sitter::{Language, Node};

use crate::lang::{LanguageId, LanguageModel, SourceFile};

/// The Java adapter. Unit-valued: it carries no state.
pub struct JavaModel;

pub static JAVA: JavaModel = JavaModel;

impl LanguageModel for JavaModel {
    fn is_statement(&self, node: Node<'_>) -> bool {
        node.kind().ends_with("_statement") || node.kind() == "local_variable_declaration"
    }

    fn accessor_chain_depth(&self, file: &SourceFile, node: Node<'_>) -> usize {
        if node.kind() != "method_invocation" {
            return 0;
        }
        crate::lang::accessors::java_accessor_depth(file, node)
    }

    fn is_comment(&self, node: Node<'_>) -> bool {
        matches!(node.kind(), "line_comment" | "block_comment")
    }

    fn method_coupling(&self, files: &[SourceFile]) -> Vec<crate::lang::coupling::MethodCoupling> {
        calls::coupling(files)
    }

    fn id(&self) -> LanguageId {
        LanguageId::Java
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["java"]
    }

    fn grammar(&self) -> Language {
        tree_sitter_java::LANGUAGE.into()
    }

    fn types<'a>(&self, file: &'a SourceFile) -> Vec<Node<'a>> {
        types::all_types(file)
    }

    fn method_comparison_owners<'a>(&self, file: &'a SourceFile) -> Vec<Node<'a>> {
        self.types(file)
            .into_iter()
            .filter(|ty| ty.kind() == "class_declaration")
            .collect()
    }

    fn type_name<'a>(&self, file: &'a SourceFile, ty: Node<'a>) -> Option<&'a str> {
        types::simple_name(file, ty)
    }

    fn methods<'a>(&self, _file: &'a SourceFile, ty: Node<'a>) -> Vec<Node<'a>> {
        types::methods(ty)
    }

    fn constructors<'a>(&self, _file: &SourceFile, ty: Node<'a>) -> Vec<Node<'a>> {
        types::constructors(ty)
    }

    fn parameters(&self, node: Node<'_>) -> usize {
        types::parameters(node)
    }

    fn is_override(&self, file: &SourceFile, node: Node<'_>) -> bool {
        types::is_override(file, node)
    }

    fn body<'a>(&self, node: Node<'a>) -> Option<Node<'a>> {
        types::body(node)
    }

    fn decl_line(&self, node: Node<'_>) -> usize {
        types::decl_line(node)
    }

    fn lloc(&self, file: &SourceFile, body: Node<'_>) -> usize {
        metrics::lloc(file, body)
    }

    fn complexity(&self, file: &SourceFile, method: Node<'_>) -> usize {
        metrics::cyclomatic_complexity(file, method)
    }
}
