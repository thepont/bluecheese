//! Comments: report explanatory comments longer than five words.

use std::collections::BTreeMap;

use crate::finding::Finding;
use crate::lang::{LanguageId, SourceFile, model_for};
use crate::walk::descendants;
use tree_sitter::Node;

pub const SMELL_NAME: &str = "Comments";
pub const MAX_COMMENT_WORDS: usize = 5;

pub fn detect(files: &[SourceFile]) -> Vec<Finding> {
    let mut findings = Vec::new();
    for file in files {
        let model = model_for(file.language);
        if file.language == LanguageId::Java {
            for ty in model.types(file) {
                let Some(class_name) = model.type_name(file, ty) else {
                    continue;
                };
                let mut lines: Vec<usize> = descendants(ty)
                    .into_iter()
                    .filter(|node| matches!(node.kind(), "line_comment" | "block_comment"))
                    .filter(|node| !is_leading_declaration_comment(*node))
                    .filter(|node| is_explanatory(file, *node))
                    .map(|node| file.line(node))
                    .collect();
                lines.sort_unstable();
                lines.dedup();
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
            continue;
        }

        let Some(root) = model.types(file).first().copied() else {
            continue;
        };
        let mut grouped = BTreeMap::<(usize, String), Vec<usize>>::new();
        for comment in descendants(root)
            .into_iter()
            .filter(|node| model.is_comment(*node))
        {
            if is_leading_non_java_declaration_comment(comment) || !is_explanatory(file, comment) {
                continue;
            }
            let owner = comment_owner(comment).unwrap_or(root);
            let Some(class_name) = model.type_name(file, owner) else {
                continue;
            };
            grouped
                .entry((owner.start_byte(), class_name.to_owned()))
                .or_default()
                .push(file.line(comment));
        }
        for ((_, class_name), mut lines) in grouped {
            lines.sort_unstable();
            lines.dedup();
            findings.push(Finding {
                smell_name: SMELL_NAME.to_string(),
                file_path: file.rel.clone(),
                class_name,
                line_numbers: lines,
                relative_file_path: file.rel.clone(),
                additional_info: String::new(),
            });
        }
    }
    findings
}

fn is_explanatory(file: &SourceFile, node: Node<'_>) -> bool {
    word_count(file.text(node)) > MAX_COMMENT_WORDS || file.text(node).contains("TODO")
}

fn comment_owner(comment: Node<'_>) -> Option<Node<'_>> {
    let mut parent = comment.parent();
    while let Some(node) = parent {
        if matches!(
            node.kind(),
            "struct_item"
                | "enum_item"
                | "trait_item"
                | "union_item"
                | "class_declaration"
                | "class"
                | "abstract_class_declaration"
                | "interface_declaration"
        ) {
            return Some(node);
        }
        parent = node.parent();
    }
    None
}

fn is_leading_non_java_declaration_comment(comment: Node<'_>) -> bool {
    let mut next = comment.next_named_sibling();
    while let Some(sibling) = next {
        if matches!(sibling.kind(), "line_comment" | "block_comment" | "comment") {
            next = sibling.next_named_sibling();
        } else {
            return matches!(
                sibling.kind(),
                "function_item"
                    | "function_signature_item"
                    | "struct_item"
                    | "enum_item"
                    | "trait_item"
                    | "union_item"
                    | "class_declaration"
                    | "class"
                    | "abstract_class_declaration"
                    | "interface_declaration"
                    | "method_definition"
                    | "function_declaration"
            );
        }
    }
    false
}

fn is_leading_declaration_comment(node: Node<'_>) -> bool {
    if node
        .prev_named_sibling()
        .is_some_and(|previous| previous.end_position().row == node.start_position().row)
    {
        return false;
    }
    let mut next = node.next_named_sibling();
    while let Some(sibling) = next {
        if matches!(sibling.kind(), "line_comment" | "block_comment") {
            next = sibling.next_named_sibling();
        } else {
            return matches!(
                sibling.kind(),
                "method_declaration" | "constructor_declaration" | "class_declaration"
            );
        }
    }
    false
}

fn word_count(comment: &str) -> usize {
    comment
        .split_whitespace()
        .filter(|word| word.chars().any(char::is_alphanumeric))
        .count()
}
