//! Java adapter: maps tree-sitter-java onto the shared concepts.
//!
//! The rules in `metrics` and `types` were reverse-engineered from Spoon rather
//! than guessed, and `tests/parity.rs` holds them to Spoon's own output. See
//! `reference/README.md` for the semantics and how they were discovered.

pub mod calls;
mod metrics;
mod types;

use tree_sitter::{Language, Node};

use crate::lang::{LanguageId, LanguageModel, SourceFile, SourceNode};

/// The Java adapter. Unit-valued: it carries no state.
pub struct JavaModel;

pub static JAVA: JavaModel = JavaModel;

impl LanguageModel for JavaModel {
    fn field_access_parts<'a>(&self, node: Node<'a>) -> Option<(Node<'a>, Node<'a>)> {
        if node.kind() != "field_access" {
            return None;
        }
        Some((
            node.child_by_field_name("object")?,
            node.child_by_field_name("field")?,
        ))
    }

    fn is_statement(&self, node: Node<'_>) -> bool {
        node.kind().ends_with("_statement") || node.kind() == "local_variable_declaration"
    }

    fn accessor_chain_depth(&self, source: SourceNode<'_>) -> usize {
        let SourceNode { file, node } = source;
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

    fn fields<'a>(&self, ty: Node<'a>) -> Vec<Node<'a>> {
        types::fields(ty)
    }

    fn field_type<'a>(&self, field: Node<'a>) -> Option<Node<'a>> {
        field.parent()?.child_by_field_name("type")
    }

    fn supports_primitive_fields(&self, kind: &str) -> bool {
        kind == "class_declaration"
    }

    fn is_primitive_type(&self, raw: &str) -> bool {
        matches!(
            super::primitive_type_name(raw),
            "byte" | "short" | "int" | "long" | "float" | "double" | "char" | "boolean"
        )
    }

    fn method_comparison_owners<'a>(&self, file: &'a SourceFile) -> Vec<Node<'a>> {
        self.types(file)
            .into_iter()
            .filter(|ty| ty.kind() == "class_declaration")
            .collect()
    }

    fn type_name<'a>(&self, source: SourceNode<'a>) -> Option<&'a str> {
        let SourceNode { file, node: ty } = source;
        types::simple_name(file, ty)
    }

    fn methods<'a>(&self, source: SourceNode<'a>) -> Vec<Node<'a>> {
        let SourceNode {
            file: _file,
            node: ty,
        } = source;
        types::methods(ty)
    }

    fn constructors<'a>(&self, source: SourceNode<'a>) -> Vec<Node<'a>> {
        let SourceNode {
            file: _file,
            node: ty,
        } = source;
        types::constructors(ty)
    }

    fn parameters(&self, node: Node<'_>) -> usize {
        types::parameters(node)
    }

    fn is_override(&self, source: SourceNode<'_>) -> bool {
        let SourceNode { file, node } = source;
        types::is_override(file, node)
    }

    fn body<'a>(&self, node: Node<'a>) -> Option<Node<'a>> {
        types::body(node)
    }

    fn decl_line(&self, node: Node<'_>) -> usize {
        types::decl_line(node)
    }

    fn lloc(&self, source: SourceNode<'_>) -> usize {
        let SourceNode { file, node: body } = source;
        metrics::lloc(file, body)
    }

    fn complexity(&self, source: SourceNode<'_>) -> usize {
        let SourceNode { file, node: method } = source;
        metrics::cyclomatic_complexity(file, method)
    }
}
