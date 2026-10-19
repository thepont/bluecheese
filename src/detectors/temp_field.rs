//! Best-effort Temporary Field heuristic for Java.
//!
//! A class field is a candidate when every reference found in its methods is
//! nested inside an `if` branch. This identifies state used only for a
//! conditional workflow; it does not reason about runtime call sequences.

use std::collections::BTreeSet;

use crate::finding::Finding;
use crate::lang::{LanguageId, LanguageModel, SourceFile, java};
use crate::walk::descendants;
use tree_sitter::Node;

pub const SMELL_NAME: &str = "Temporary Field";

pub fn detect(files: &[SourceFile]) -> Vec<Finding> {
    files
        .iter()
        .filter(|file| file.language == LanguageId::Java)
        .flat_map(file_findings)
        .collect()
}

fn file_findings(file: &SourceFile) -> Vec<Finding> {
    java::JAVA
        .types(file)
        .into_iter()
        .filter(|ty| ty.kind() == "class_declaration")
        .filter_map(|ty| {
            let methods = java::JAVA.methods(file, ty);
            let lines = fields(file, ty)
                .into_iter()
                .filter(|(_, name)| references_only_in_branches(file, name, &methods))
                .map(|(field, _)| file.line(field))
                .collect::<BTreeSet<_>>();
            (!lines.is_empty()).then(|| Finding {
                smell_name: SMELL_NAME.to_owned(),
                file_path: file.rel.clone(),
                class_name: java::JAVA
                    .type_name(file, ty)
                    .unwrap_or("<anonymous>")
                    .to_owned(),
                line_numbers: lines.into_iter().collect(),
                relative_file_path: file.rel.clone(),
                additional_info: String::new(),
            })
        })
        .collect()
}

fn fields<'a>(file: &SourceFile, class: Node<'a>) -> Vec<(Node<'a>, String)> {
    let Some(body) = class.child_by_field_name("body") else {
        return Vec::new();
    };
    let mut cursor = body.walk();
    body.named_children(&mut cursor)
        .filter(|child| child.kind() == "field_declaration")
        .flat_map(|declaration| {
            let mut cursor = declaration.walk();
            declaration
                .named_children(&mut cursor)
                .filter(|child| child.kind() == "variable_declarator")
                .filter_map(|field| field.child_by_field_name("name").map(|name| (field, name)))
                .collect::<Vec<_>>()
        })
        .map(|(field, name)| (field, file.text(name).to_owned()))
        .collect()
}

fn references_only_in_branches(file: &SourceFile, name: &str, methods: &[Node<'_>]) -> bool {
    let mut saw_reference = false;
    for method in methods {
        let Some(body) = method.child_by_field_name("body") else {
            continue;
        };
        for reference in descendants(body)
            .into_iter()
            .filter(|node| node.kind() == "identifier" && file.text(*node) == name)
        {
            saw_reference = true;
            if !inside_if_branch(reference, *method) {
                return false;
            }
        }
    }
    saw_reference
}

fn inside_if_branch(mut node: Node<'_>, method: Node<'_>) -> bool {
    while let Some(parent) = node.parent() {
        if parent.kind() == "if_statement"
            && !parent
                .child_by_field_name("condition")
                .is_some_and(|condition| {
                    condition.start_byte() <= node.start_byte()
                        && condition.end_byte() >= node.end_byte()
                })
        {
            return true;
        }
        if parent.id() == method.id() {
            break;
        }
        node = parent;
    }
    false
}
