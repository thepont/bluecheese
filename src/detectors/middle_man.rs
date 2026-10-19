//! Port of StaticNose's pure delegation and fan out rule.

use std::collections::BTreeSet;

use crate::finding::Finding;
use crate::lang::{LanguageId, SourceFile, model_for};
use crate::walk::descendants;
use tree_sitter::Node;

pub const SMELL_NAME: &str = "Middle Man";

struct Analysis<'a> {
    file: &'a SourceFile,
    project_types: &'a BTreeSet<String>,
}

pub fn detect(files: &[SourceFile]) -> Vec<Finding> {
    let project_types: BTreeSet<String> = files
        .iter()
        .filter(|file| file.language == LanguageId::Java)
        .flat_map(|file| {
            let model = model_for(LanguageId::Java);
            model
                .types(file)
                .into_iter()
                .filter_map(move |ty| model.type_name(file, ty).map(str::to_owned))
        })
        .collect();
    files
        .iter()
        .filter(|file| file.language == LanguageId::Java)
        .flat_map(|file| file_findings(file, &project_types))
        .collect()
}

fn file_findings(file: &SourceFile, project_types: &BTreeSet<String>) -> Vec<Finding> {
    let model = model_for(LanguageId::Java);
    let analysis = Analysis {
        file,
        project_types,
    };
    model
        .types(file)
        .into_iter()
        .filter_map(|ty| {
            let name = model.type_name(file, ty)?;
            let methods = model.methods(file, ty);
            if methods.is_empty() {
                return None;
            }
            let delegate_count = methods
                .iter()
                .filter(|method| is_pure_delegate(&analysis, **method, ty))
                .count();
            let fan_out = referenced_project_types(&analysis, ty);
            if delegate_count < 2 || delegate_count * 2 < methods.len() || fan_out == 0 {
                return None;
            }
            Some(Finding {
                smell_name: SMELL_NAME.to_string(),
                file_path: file.rel.clone(),
                class_name: name.to_string(),
                line_numbers: vec![model.decl_line(ty)],
                relative_file_path: file.rel.clone(),
                additional_info: String::new(),
            })
        })
        .collect()
}

fn referenced_project_types(analysis: &Analysis<'_>, ty: Node<'_>) -> usize {
    let mut names = BTreeSet::new();
    for node in descendants(ty) {
        if node.kind() == "type_identifier" {
            let name = analysis
                .file
                .text(node)
                .rsplit('.')
                .next()
                .unwrap_or(analysis.file.text(node));
            if name != analysis.file.text(ty.child_by_field_name("name").unwrap())
                && analysis.project_types.contains(name)
            {
                names.insert(name.to_owned());
            }
        }
    }
    names.len()
}

fn is_pure_delegate(analysis: &Analysis<'_>, method: Node<'_>, owner: Node<'_>) -> bool {
    let Some(body) = method.child_by_field_name("body") else {
        return false;
    };
    let mut cursor = body.walk();
    let statements: Vec<_> = body
        .named_children(&mut cursor)
        .filter(|node| !node.kind().ends_with("comment"))
        .collect();
    if statements.is_empty() {
        return false;
    }

    let mut found_delegate = false;
    for statement in statements {
        if let Some(call) = statement_call(statement) {
            if delegates_to_field(analysis, call, owner) {
                found_delegate = true;
            } else if statement.kind() != "expression_statement" {
                return false;
            }
        } else if statement.kind() == "return_statement" {
            if let Some(expression) = first_named(statement) {
                if !matches!(
                    expression.kind(),
                    "identifier"
                        | "decimal_integer_literal"
                        | "string_literal"
                        | "true"
                        | "false"
                        | "null_literal"
                ) {
                    return false;
                }
            }
        } else {
            return false;
        }
    }
    found_delegate
}

fn statement_call(statement: Node<'_>) -> Option<Node<'_>> {
    if statement.kind() == "expression_statement" {
        let mut cursor = statement.walk();
        return statement
            .named_children(&mut cursor)
            .find(|node| node.kind() == "method_invocation");
    }
    if statement.kind() == "return_statement" {
        return descendants(statement)
            .into_iter()
            .find(|node| node.kind() == "method_invocation");
    }
    None
}

fn delegates_to_field(analysis: &Analysis<'_>, call: Node<'_>, owner: Node<'_>) -> bool {
    let Some(object) = call.child_by_field_name("object") else {
        return false;
    };
    let object_name = if object.kind() == "field_access" {
        object
            .child_by_field_name("field")
            .map(|field| analysis.file.text(field))
            .unwrap_or_default()
    } else {
        analysis.file.text(object)
    };
    let Some(owner_body) = owner.child_by_field_name("body") else {
        return false;
    };
    let mut cursor = owner_body.walk();
    owner_body
        .named_children(&mut cursor)
        .filter(|node| node.kind() == "field_declaration")
        .any(|field| {
            let type_name = field.child_by_field_name("type").map(|ty| {
                analysis
                    .file
                    .text(ty)
                    .rsplit('.')
                    .next()
                    .unwrap_or(analysis.file.text(ty))
            });
            if !type_name.is_some_and(|name| analysis.project_types.contains(name)) {
                return false;
            }
            let mut cursor = field.walk();
            field
                .named_children(&mut cursor)
                .filter(|node| node.kind() == "variable_declarator")
                .any(|variable| {
                    variable
                        .child_by_field_name("name")
                        .is_some_and(|name| analysis.file.text(name) == object_name)
                })
        })
}

fn first_named(node: Node<'_>) -> Option<Node<'_>> {
    let mut cursor = node.walk();
    node.named_children(&mut cursor).next()
}
