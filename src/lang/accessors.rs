use super::SourceFile;
use tree_sitter::Node;

pub(super) fn java_accessor_depth(file: &SourceFile, node: Node<'_>) -> usize {
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
    let receiver_depth = java_accessor_depth(file, receiver_call);
    if receiver_depth == 0 {
        1
    } else {
        1 + receiver_depth
    }
}

pub(super) fn rust_accessor_depth(file: &SourceFile, node: Node<'_>) -> usize {
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
    if depth == 0 { 1 } else { depth + 1 }
}

pub(super) fn ecma_accessor_depth(file: &SourceFile, node: Node<'_>) -> usize {
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
    if depth == 0 { 1 } else { depth + 1 }
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
