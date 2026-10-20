//! Switch Statements: report switch-like control flow in methods.
//!
//! Java's grammar represents both switch statements and switch expressions as
//! `switch_expression`; Rust's corresponding construct is `match_expression`.
//! Findings point to the start of each construct and are grouped by type.
//! Java type-code `if` / `else if` ladders longer than two branches are also
//! reported.

use crate::finding::Finding;
use crate::lang::{SourceFile, model_for};
use crate::walk::descendants;

pub const SMELL_NAME: &str = "Switch Statements";
pub const MAX_IF_CHAIN_LENGTH: usize = 2;

pub fn detect(files: &[SourceFile]) -> Vec<Finding> {
    let mut findings = Vec::new();

    for file in files {
        let model = model_for(file.language);

        for ty in model.types(file) {
            let Some(class_name) = model.type_name(file, ty) else {
                continue;
            };

            let mut lines = Vec::new();
            for method in model.methods(file, ty) {
                for node in descendants(method) {
                    if is_switch(file, node.kind()) {
                        lines.push(file.line(node));
                    } else if file.language == crate::lang::LanguageId::Java
                        && node.kind() == "if_statement"
                        && is_if_chain_root(node)
                        && let Some(target) = if_chain_target(file, node)
                    {
                        lines.push(file.line(target));
                    }
                }
            }

            if !lines.is_empty() {
                findings.push(Finding {
                    smell_name: SMELL_NAME.to_string(),
                    file_path: file.rel.clone(),
                    class_name: class_name.to_string(),
                    line_numbers: lines,
                    relative_file_path: file.rel.clone(),
                    additional_info: String::new(),
                });
            }
        }
    }

    findings
}

fn is_if_chain_root(node: tree_sitter::Node<'_>) -> bool {
    let mut ancestor = node.parent();
    while let Some(current) = ancestor {
        if current.kind() == "if_statement"
            && current
                .child_by_field_name("alternative")
                .is_some_and(|alternative| {
                    alternative.start_byte() <= node.start_byte()
                        && node.end_byte() <= alternative.end_byte()
                })
        {
            return false;
        }
        ancestor = current.parent();
    }
    true
}

fn if_chain_target<'tree>(
    file: &SourceFile,
    root: tree_sitter::Node<'tree>,
) -> Option<tree_sitter::Node<'tree>> {
    let mut chain = vec![root];
    let mut current = root;
    while let Some(alternative) = current.child_by_field_name("alternative") {
        if alternative.kind() == "if_statement" {
            current = alternative;
            chain.push(current);
            continue;
        }
        let mut cursor = alternative.walk();
        let Some(next_if) = alternative
            .named_children(&mut cursor)
            .find(|child| child.kind() == "if_statement")
        else {
            break;
        };
        current = next_if;
        chain.push(current);
    }

    if chain.len() <= MAX_IF_CHAIN_LENGTH {
        return None;
    }

    let selectors: Vec<_> = chain
        .iter()
        .filter_map(|branch| Condition::new(file, *branch)?.selector())
        .collect();
    if selectors.len() == chain.len() && selectors.iter().all(|item| item == &selectors[0]) {
        return Some(root);
    }

    chain.iter().rev().copied().find(|branch| {
        Condition::new(file, *branch).is_some_and(|condition| condition.is_enum_comparison())
    })
}

struct Condition<'a> {
    file: &'a SourceFile,
    node: tree_sitter::Node<'a>,
}

impl<'a> Condition<'a> {
    fn new(file: &'a SourceFile, branch: tree_sitter::Node<'a>) -> Option<Self> {
        Some(Self {
            file,
            node: branch.child_by_field_name("condition")?,
        })
    }

    fn selector(&self) -> Option<String> {
        let raw = self.file.text(self.node).trim();
        if let Some(index) = raw.find(".equals(") {
            return Some(
                raw[..index]
                    .trim()
                    .trim_start_matches('!')
                    .trim()
                    .to_string(),
            );
        }
        ["==", "!=", ">=", "<=", ">", "<"]
            .iter()
            .find_map(|operator| raw.split_once(operator).map(|(left, _)| left.trim()))
            .map(str::to_string)
    }

    fn is_enum_comparison(&self) -> bool {
        let raw = self.file.text(self.node);
        raw.contains("==")
            && raw.split_once("==").is_some_and(|(_, right)| {
                right.contains('.') && right.chars().any(char::is_uppercase)
            })
    }
}

fn is_switch(file: &SourceFile, kind: &str) -> bool {
    (file.language == crate::lang::LanguageId::Java && kind == "switch_expression")
        || (file.language == crate::lang::LanguageId::Rust && kind == "match_expression")
        || (matches!(
            file.language,
            crate::lang::LanguageId::JavaScript
                | crate::lang::LanguageId::TypeScript
                | crate::lang::LanguageId::Tsx
        ) && kind == "switch_statement")
}
