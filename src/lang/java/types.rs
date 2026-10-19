//! Type and method enumeration, mirroring what Spoon's
//! `CtModel.getAllTypes()` and `CtType.getMethods()` hand to a detector.

use tree_sitter::Node;

use crate::lang::SourceFile;

/// Spoon `CtType` implementations that appear in `getAllTypes()`.
pub const TYPE_KINDS: &[&str] = &[
    "class_declaration",
    "interface_declaration",
    "enum_declaration",
    "record_declaration",
    "annotation_type_declaration",
];

/// All top-level types in the file, in source order. Spoon's
/// `CtModel.getAllTypes()` does not include nested types, so neither does this.
pub fn all_types<'a>(file: &'a SourceFile) -> Vec<Node<'a>> {
    let root = file.root();
    let mut out = Vec::new();
    let mut cursor = root.walk();
    for child in root.named_children(&mut cursor) {
        if TYPE_KINDS.contains(&child.kind()) {
            out.push(child);
        }
    }
    out
}

/// Simple name of a type declaration (`CtType.getSimpleName()`).
pub fn simple_name<'a>(file: &'a SourceFile, ty: Node<'a>) -> Option<&'a str> {
    ty.child_by_field_name("name").map(|n| file.text(n))
}

pub fn body<'a>(ty: Node<'a>) -> Option<Node<'a>> {
    ty.child_by_field_name("body")
}

/// 1-based line where Spoon starts a declaration's position: leading
/// annotations are excluded, so `@Override` on its own line does not shift the
/// reported line. When the only modifier is an annotation, the position starts
/// at the next token (e.g. the return type).
pub fn decl_line(node: Node<'_>) -> usize {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "modifiers" {
            let mut modifiers_cursor = child.walk();
            for g in child.children(&mut modifiers_cursor) {
                if matches!(g.kind(), "annotation" | "marker_annotation") {
                    continue;
                }
                return g.start_position().row + 1;
            }
            // Only annotations in `modifiers`; the position starts at the next child.
            continue;
        }
        return child.start_position().row + 1;
    }
    node.start_position().row + 1
}

/// Methods declared directly in `ty`. Spoon's `CtType.getMethods()` returns
/// methods only, not constructors, so `constructor_declaration` is excluded.
///
/// Enum members are wrapped in an `enum_body_declarations` node, so that level
/// is transparently descended into.
pub fn methods<'a>(ty: Node<'a>) -> Vec<Node<'a>> {
    let mut out = Vec::new();
    if let Some(body) = body(ty) {
        collect_methods(body, &mut out);
    }
    out
}

fn collect_methods<'a>(body: Node<'a>, out: &mut Vec<Node<'a>>) {
    let mut cursor = body.walk();
    for child in body.named_children(&mut cursor) {
        match child.kind() {
            "method_declaration" => out.push(child),
            "enum_body_declarations" => collect_methods(child, out),
            _ => {}
        }
    }
}
