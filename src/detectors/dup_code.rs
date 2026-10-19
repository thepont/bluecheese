//! Repeated executable fragments within Java classes.

use std::collections::BTreeSet;

use crate::finding::Finding;
use crate::lang::{LanguageId, SourceFile, model_for};
use crate::walk::descendants;
use tree_sitter::Node;

pub const SMELL_NAME: &str = "Duplicate Code";
const MIN_SHARED_STATEMENTS: usize = 3;

pub fn detect(files: &[SourceFile]) -> Vec<Finding> {
    let mut findings = Vec::new();
    for file in files {
        if file.language != LanguageId::Java {
            continue;
        }
        let model = model_for(file.language);
        for ty in model.types(file) {
            if ty.kind() != "class_declaration" {
                continue;
            }
            let Some(class_name) = model.type_name(file, ty) else {
                continue;
            };
            let methods = model.methods(file, ty);
            let mut duplicate_lines = BTreeSet::new();
            for left in 0..methods.len() {
                for right in left + 1..methods.len() {
                    if duplicate_pair(file, methods[left], methods[right]) {
                        duplicate_lines.insert(model.decl_line(methods[left]));
                        duplicate_lines.insert(model.decl_line(methods[right]));
                    }
                }
            }
            if !duplicate_lines.is_empty() {
                findings.push(Finding {
                    smell_name: SMELL_NAME.to_string(),
                    file_path: file.rel.clone(),
                    class_name: class_name.to_string(),
                    line_numbers: duplicate_lines.into_iter().collect(),
                    relative_file_path: file.rel.clone(),
                    additional_info: String::new(),
                });
            }
        }
    }
    findings
}

fn duplicate_pair(file: &SourceFile, left: Node<'_>, right: Node<'_>) -> bool {
    let left_statements = statement_fingerprints(file, left);
    let right_statements = statement_fingerprints(file, right);
    if left_statements.intersection(&right_statements).count() >= MIN_SHARED_STATEMENTS {
        return true;
    }
    arithmetic_fingerprints(file, left)
        .intersection(&arithmetic_fingerprints(file, right))
        .next()
        .is_some()
}

fn statement_fingerprints(file: &SourceFile, method: Node<'_>) -> BTreeSet<String> {
    descendants(method)
        .into_iter()
        .filter(|node| is_statement(*node))
        .map(|node| normalize(file.text(node)))
        .collect()
}

fn is_statement(node: Node<'_>) -> bool {
    node.kind().ends_with("_statement") || node.kind() == "local_variable_declaration"
}

fn arithmetic_fingerprints(file: &SourceFile, method: Node<'_>) -> BTreeSet<String> {
    descendants(method)
        .into_iter()
        .filter(|node| node.kind() == "binary_expression")
        .filter(|node| {
            let mut cursor = node.walk();
            node.children(&mut cursor)
                .any(|child| matches!(file.text(child), "+" | "-" | "*" | "/" | "%"))
        })
        .map(|node| normalize(file.text(node)))
        .collect()
}

fn normalize(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}
