//! Best-effort Lazy Class heuristic for Java and ECMAScript.
//!
//! A fieldless class with exactly one method whose body consists of a single
//! call is treated as a likely forwarding shell. This intentionally favors a
//! narrow, concrete signal over trying to infer the value of every small class.

use crate::finding::Finding;
use crate::lang::{LanguageId, SourceFile, model_for};
use tree_sitter::Node;

pub const SMELL_NAME: &str = "Lazy Class";

pub fn detect(files: &[SourceFile]) -> Vec<Finding> {
    files
        .iter()
        .filter(|file| file.language != LanguageId::Rust)
        .flat_map(file_findings)
        .collect()
}

fn file_findings(file: &SourceFile) -> Vec<Finding> {
    model_for(file.language)
        .types(file)
        .into_iter()
        .filter(|ty| model_for(file.language).is_data_type(ty.kind()))
        .filter(|ty| has_no_fields(file, *ty))
        .filter(|ty| {
            let methods = model_for(file.language).methods(file.at(*ty));
            methods.len() == 1 && is_forwarding_method(file, methods[0])
        })
        .filter_map(|ty| {
            Some(Finding {
                smell_name: SMELL_NAME.to_owned(),
                file_path: file.rel.clone(),
                class_name: model_for(file.language).type_name(file.at(ty))?.to_owned(),
                line_numbers: vec![model_for(file.language).decl_line(ty)],
                relative_file_path: file.rel.clone(),
                additional_info: String::new(),
            })
        })
        .collect()
}

fn has_no_fields(file: &SourceFile, class: Node<'_>) -> bool {
    if file.language != LanguageId::Java {
        let model = model_for(file.language);
        return model.fields(class).is_empty() && model.constructors(file.at(class)).is_empty();
    }
    let Some(body) = class.child_by_field_name("body") else {
        return false;
    };
    let mut cursor = body.walk();
    !body
        .named_children(&mut cursor)
        .any(|child| child.kind() == "field_declaration")
}

fn is_forwarding_method(file: &SourceFile, method: Node<'_>) -> bool {
    if file.language != LanguageId::Java {
        let Some(statements) = model_for(file.language).body_statements(method) else {
            return false;
        };
        if statements.len() != 1
            || !matches!(
                statements[0].kind(),
                "return_statement" | "expression_statement"
            )
        {
            return false;
        }
        return statements[0]
            .named_child(0)
            .is_some_and(|expression| expression.kind() == "call_expression");
    }
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
