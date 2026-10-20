//! JavaScript/TypeScript syntax adapters. Metrics describe source structure;
//! resolving runtime mutation, arbitrary require() calls and decorators is not
//! attempted. JS and TS share one project-local ES-module call resolver.

pub mod calls;

use crate::lang::{LanguageId, LanguageModel, SourceFile, SourceNode, coupling::MethodCoupling};
use crate::walk::descendants;
use tree_sitter::{Language, Node};

pub struct EcmaModel(pub LanguageId);
pub static JAVASCRIPT: EcmaModel = EcmaModel(LanguageId::JavaScript);
pub static TYPESCRIPT: EcmaModel = EcmaModel(LanguageId::TypeScript);
pub static TSX: EcmaModel = EcmaModel(LanguageId::Tsx);

impl LanguageModel for EcmaModel {
    fn field_access_parts<'a>(&self, node: Node<'a>) -> Option<(Node<'a>, Node<'a>)> {
        if node.kind() != "member_expression" {
            return None;
        }
        Some((
            node.child_by_field_name("object")?,
            node.child_by_field_name("property")?,
        ))
    }

    fn is_statement(&self, node: Node<'_>) -> bool {
        node.kind().ends_with("_statement")
            || matches!(node.kind(), "lexical_declaration" | "variable_declaration")
    }

    fn accessor_chain_depth(&self, file: &SourceFile, node: Node<'_>) -> usize {
        if node.kind() != "call_expression" {
            return 0;
        }
        crate::lang::accessors::ECMASCRIPT.depth(file, node)
    }

    fn is_comment(&self, node: Node<'_>) -> bool {
        node.kind() == "comment"
    }

    fn method_coupling(&self, files: &[SourceFile]) -> Vec<MethodCoupling> {
        calls::coupling(files, self.0)
    }
    fn id(&self) -> LanguageId {
        self.0
    }
    fn extensions(&self) -> &'static [&'static str] {
        if self.0 == LanguageId::JavaScript {
            &["js", "jsx", "mjs", "cjs"]
        } else if self.0 == LanguageId::TypeScript {
            &["ts", "mts", "cts"]
        } else {
            &["tsx"]
        }
    }
    fn grammar(&self) -> Language {
        if self.0 == LanguageId::JavaScript {
            tree_sitter_javascript::LANGUAGE.into()
        } else if self.0 == LanguageId::TypeScript {
            tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()
        } else {
            tree_sitter_typescript::LANGUAGE_TSX.into()
        }
    }
    fn types<'a>(&self, file: &'a SourceFile) -> Vec<Node<'a>> {
        std::iter::once(file.root())
            .chain(
                descendants(file.root())
                    .into_iter()
                    .filter(|node| is_owner(*node)),
            )
            .collect()
    }
    fn type_name<'a>(&self, source: SourceNode<'a>) -> Option<&'a str> {
        let SourceNode { file, node: ty } = source;
        if ty.kind() == "program" {
            Some(&file.rel)
        } else {
            owner_name(file, ty)
        }
    }

    fn fields<'a>(&self, ty: Node<'a>) -> Vec<Node<'a>> {
        self.body_children(ty)
            .unwrap_or_default()
            .into_iter()
            .filter(|field| {
                matches!(
                    field.kind(),
                    "public_field_definition" | "field_definition" | "property_definition"
                )
            })
            .collect()
    }

    fn supports_primitive_fields(&self, kind: &str) -> bool {
        self.0 != LanguageId::JavaScript
            && matches!(
                kind,
                "class_declaration" | "class" | "abstract_class_declaration"
            )
    }

    fn is_primitive_type(&self, raw: &str) -> bool {
        self.0 != LanguageId::JavaScript
            && matches!(
                super::primitive_type_name(raw),
                "number" | "string" | "boolean" | "bigint" | "symbol"
            )
    }
    fn methods<'a>(&self, source: SourceNode<'a>) -> Vec<Node<'a>> {
        let SourceNode { file, node: ty } = source;
        descendants(ty)
            .into_iter()
            .filter(|node| is_function(*node))
            .filter(|node| function_name(file, *node).is_some_and(|name| name != "constructor"))
            .filter(|node| owner(*node).id() == ty.id() && enclosing_function(*node).is_none())
            .collect()
    }
    fn constructors<'a>(&self, source: SourceNode<'a>) -> Vec<Node<'a>> {
        let SourceNode { file, node: ty } = source;
        descendants(ty)
            .into_iter()
            .filter(|node| node.kind() == "method_definition")
            .filter(|node| {
                node.child_by_field_name("name")
                    .is_some_and(|name| file.text(name) == "constructor")
            })
            .filter(|node| owner(*node).id() == ty.id())
            .collect()
    }
    fn parameters(&self, node: Node<'_>) -> usize {
        if let Some(parameters) = node.child_by_field_name("parameters") {
            return children(parameters).len();
        }
        usize::from(node.child_by_field_name("parameter").is_some())
    }
    fn is_override(&self, _file: &SourceFile, _node: Node<'_>) -> bool {
        false
    }
    fn body<'a>(&self, node: Node<'a>) -> Option<Node<'a>> {
        node.child_by_field_name("body")
    }
    fn decl_line(&self, node: Node<'_>) -> usize {
        node.start_position().row + 1
    }
    fn lloc(&self, _file: &SourceFile, body: Node<'_>) -> usize {
        descendants(body)
            .into_iter()
            .filter(|node| node.kind() == "statement_block")
            .map(|node| children(node).len())
            .sum()
    }
    fn complexity(&self, _file: &SourceFile, method: Node<'_>) -> usize {
        1 + descendants(method)
            .into_iter()
            .filter(|node| {
                [
                    "if_statement",
                    "for_statement",
                    "for_in_statement",
                    "while_statement",
                    "do_statement",
                    "catch_clause",
                    "ternary_expression",
                    "switch_case",
                ]
                .contains(&node.kind())
            })
            .count()
    }
}

pub fn is_ecmascript(language: LanguageId) -> bool {
    [
        LanguageId::JavaScript,
        LanguageId::TypeScript,
        LanguageId::Tsx,
    ]
    .contains(&language)
}

pub fn is_class(node: Node<'_>) -> bool {
    node.is_named()
        && [
            "class_declaration",
            "class",
            "abstract_class_declaration",
            "interface_declaration",
        ]
        .contains(&node.kind())
}
pub fn is_function(node: Node<'_>) -> bool {
    [
        "method_definition",
        "function_declaration",
        "generator_function_declaration",
        "function_expression",
        "arrow_function",
        "method_signature",
    ]
    .contains(&node.kind())
}

pub fn children(node: Node<'_>) -> Vec<Node<'_>> {
    let mut cursor = node.walk();
    node.named_children(&mut cursor)
        .filter(|node| node.kind() != "comment")
        .collect()
}

pub fn function_name(file: &SourceFile, node: Node<'_>) -> Option<String> {
    if let Some(name) = node.child_by_field_name("name") {
        return Some(file.text(name).into());
    }
    let parent = node.parent()?;
    let name = parent
        .child_by_field_name("name")
        .or_else(|| parent.child_by_field_name("key"))
        .or_else(|| parent.child_by_field_name("property"))?;
    Some(file.text(name).into())
}

pub fn owner(mut node: Node<'_>) -> Node<'_> {
    while let Some(parent) = node.parent() {
        if is_owner(parent) {
            return parent;
        }
        node = parent;
    }
    node
}

pub fn is_owner(node: Node<'_>) -> bool {
    is_class(node) || node.kind() == "object"
}

pub fn owner_name<'a>(file: &'a SourceFile, node: Node<'_>) -> Option<&'a str> {
    let name = node.child_by_field_name("name").or_else(|| {
        node.parent().and_then(|parent| {
            parent
                .child_by_field_name("name")
                .or_else(|| parent.child_by_field_name("key"))
        })
    })?;
    Some(file.text(name))
}

pub fn enclosing_function(node: Node<'_>) -> Option<Node<'_>> {
    let mut parent = node.parent();
    while let Some(node) = parent {
        if is_function(node) {
            return Some(node);
        }
        if is_class(node) {
            return None;
        }
        parent = node.parent();
    }
    None
}
