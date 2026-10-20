//! Best-effort Lazy Class heuristic for Java.
//!
//! A fieldless class with exactly one method whose body consists of a single
//! call is treated as a likely forwarding shell. This intentionally favors a
//! narrow, concrete signal over trying to infer the value of every small class.

use crate::finding::Finding;
use crate::lang::{LanguageId, LanguageModel, SourceFile, java};
use tree_sitter::Node;

pub const SMELL_NAME: &str = "Lazy Class";

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
        .filter(|ty| has_no_fields(*ty))
        .filter(|ty| {
            let methods = java::JAVA.methods(file, *ty);
            methods.len() == 1 && is_forwarding_method(methods[0])
        })
        .filter_map(|ty| {
            Some(Finding {
                smell_name: SMELL_NAME.to_owned(),
                file_path: file.rel.clone(),
                class_name: java::JAVA.type_name(file.at(ty))?.to_owned(),
                line_numbers: vec![java::JAVA.decl_line(ty)],
                relative_file_path: file.rel.clone(),
                additional_info: String::new(),
            })
        })
        .collect()
}

fn has_no_fields(class: Node<'_>) -> bool {
    let Some(body) = class.child_by_field_name("body") else {
        return false;
    };
    let mut cursor = body.walk();
    !body
        .named_children(&mut cursor)
        .any(|child| child.kind() == "field_declaration")
}

fn is_forwarding_method(method: Node<'_>) -> bool {
    let Some(body) = method.child_by_field_name("body") else {
        return false;
    };
    let mut cursor = body.walk();
    let statements = body.named_children(&mut cursor).collect::<Vec<_>>();
    if statements.len() != 1
        || !matches!(
            statements[0].kind(),
            "return_statement" | "expression_statement"
        )
    {
        return false;
    }
    let mut calls = 0;
    let mut stack = vec![statements[0]];
    while let Some(node) = stack.pop() {
        if node.kind() == "method_invocation" {
            calls += 1;
            if calls > 1 {
                return false;
            }
        }
        let mut cursor = node.walk();
        stack.extend(node.named_children(&mut cursor));
    }
    calls == 1
}
