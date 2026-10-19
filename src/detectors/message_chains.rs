//! Message Chains: report continuous getter chains of three or more calls.
//!
//! StaticNose's detector treats Java `get*` and `is*` calls as a chain and
//! ignores ordinary fluent APIs such as `StringBuilder` and streams.

use crate::finding::Finding;
use crate::lang::{LanguageId, SourceFile, model_for};
use crate::walk::descendants;
use tree_sitter::Node;

pub const SMELL_NAME: &str = "Message Chains";
pub const MINIMUM_CALLS: usize = 3;

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

            let mut lines = Vec::new();
            for method in model.methods(file, ty) {
                for node in descendants(method) {
                    if node.kind() == "method_invocation"
                        && accessor_chain_depth(file, node) >= MINIMUM_CALLS
                    {
                        lines.push(file.line(node));
                    }
                }
            }
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

fn accessor_chain_depth(file: &SourceFile, node: Node<'_>) -> usize {
    let Some(name) = node.child_by_field_name("name").map(|name| file.text(name)) else {
        return 0;
    };
    if !is_accessor(name) {
        return 0;
    }
    let Some(object) = node.child_by_field_name("object") else {
        return 1;
    };
    let Some(receiver_call) = unwrap_receiver(object) else {
        return 1;
    };
    let receiver_depth = accessor_chain_depth(file, receiver_call);
    if receiver_depth == 0 {
        1
    } else {
        1 + receiver_depth
    }
}

fn is_accessor(name: &str) -> bool {
    name.starts_with("get") || name.starts_with("is")
}

fn unwrap_receiver<'tree>(mut node: Node<'tree>) -> Option<Node<'tree>> {
    while matches!(
        node.kind(),
        "primary_expression" | "parenthesized_expression"
    ) {
        let mut cursor = node.walk();
        node = node
            .named_children(&mut cursor)
            .find(|child| child.kind() != "identifier")?;
    }
    (node.kind() == "method_invocation").then_some(node)
}
