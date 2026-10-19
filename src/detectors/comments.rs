//! Comments: report explanatory comments longer than five words.

use crate::finding::Finding;
use crate::lang::{LanguageId, SourceFile, model_for};
use crate::walk::descendants;
use tree_sitter::Node;

pub const SMELL_NAME: &str = "Comments";
pub const MAX_COMMENT_WORDS: usize = 5;

pub fn detect(files: &[SourceFile]) -> Vec<Finding> {
    let mut findings = Vec::new();
    for file in files {
        if file.language != LanguageId::Java {
            continue;
        }
        let model = model_for(file.language);
        for ty in model.types(file) {
            let Some(class_name) = model.type_name(file, ty) else {
                continue;
            };
            let mut lines: Vec<usize> = descendants(ty)
                .into_iter()
                .filter(|node| matches!(node.kind(), "line_comment" | "block_comment"))
                .filter(|node| !is_leading_declaration_comment(*node))
                .filter(|node| {
                    word_count(file.text(*node)) > MAX_COMMENT_WORDS
                        || file.text(*node).contains("TODO")
                })
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
    }
    findings
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
