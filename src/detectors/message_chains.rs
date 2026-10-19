//! Message Chains: report continuous getter chains of three or more calls.
//!
//! StaticNose's detector treats Java `get*` and `is*` calls as a chain and
//! ignores ordinary fluent APIs such as `StringBuilder` and streams.

use crate::finding::Finding;
use crate::lang::{model_for, LanguageId, SourceFile};
use crate::walk::descendants;
use tree_sitter::Node;

pub const SMELL_NAME: &str = "Message Chains";
pub const MINIMUM_CALLS: usize = 3;

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
                    if is_call(file.language, node)
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
    match file.language {
        LanguageId::Java => java_accessor_depth(file, node),
        LanguageId::Rust => rust_accessor_depth(file, node),
        LanguageId::JavaScript | LanguageId::TypeScript | LanguageId::Tsx => {
            ecma_accessor_depth(file, node)
        }
    }
}

fn is_call(language: LanguageId, node: Node<'_>) -> bool {
    match language {
        LanguageId::Java => node.kind() == "method_invocation",
        LanguageId::Rust | LanguageId::JavaScript | LanguageId::TypeScript | LanguageId::Tsx => {
            node.kind() == "call_expression"
        }
    }
}

fn java_accessor_depth(file: &SourceFile, node: Node<'_>) -> usize {
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

fn rust_accessor_depth(file: &SourceFile, node: Node<'_>) -> usize {
    let Some(function) = node.child_by_field_name("function") else {
        return 0;
    };
    if function.kind() != "field_expression" {
        return 0;
    }
    let Some(name) = function
        .child_by_field_name("field")
        .map(|field| file.text(field))
    else {
        return 0;
    };
    if !is_accessor(name) {
        return 0;
    }
    let Some(receiver) = function
        .child_by_field_name("value")
        .and_then(unwrap_parens)
    else {
        return 1;
    };
    let depth = if receiver.kind() == "call_expression" {
        rust_accessor_depth(file, receiver)
    } else {
        0
    };
    if depth == 0 {
        1
    } else {
        depth + 1
    }
}

fn ecma_accessor_depth(file: &SourceFile, node: Node<'_>) -> usize {
    let Some(function) = node.child_by_field_name("function") else {
        return 0;
    };
    if function.kind() != "member_expression" {
        return 0;
    }
    let Some(name) = function
        .child_by_field_name("property")
        .map(|property| file.text(property))
    else {
        return 0;
    };
    if !is_ecma_accessor(name) {
        return 0;
    }
    let Some(receiver) = function
        .child_by_field_name("object")
        .and_then(unwrap_parens)
    else {
        return 1;
    };
    let depth = if receiver.kind() == "call_expression" {
        ecma_accessor_depth(file, receiver)
    } else {
        0
    };
    if depth == 0 {
        1
    } else {
        depth + 1
    }
}

fn unwrap_parens(mut node: Node<'_>) -> Option<Node<'_>> {
    while matches!(
        node.kind(),
        "parenthesized_expression" | "primary_expression"
    ) {
        let mut cursor = node.walk();
        node = node
            .named_children(&mut cursor)
            .find(|child| child.kind() != "identifier")?;
    }
    Some(node)
}

fn is_ecma_accessor(name: &str) -> bool {
    name.starts_with("get") || name.starts_with("is") || name.starts_with("has")
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
