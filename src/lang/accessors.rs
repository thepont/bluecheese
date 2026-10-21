use super::{SourceFile, SourceNode};
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

pub(super) struct MemberAccessorSyntax {
    member_kind: &'static str,
    name_field: &'static str,
    receiver_field: &'static str,
    prefixes: &'static [&'static str],
}

pub(super) const RUST: MemberAccessorSyntax = MemberAccessorSyntax {
    member_kind: "field_expression",
    name_field: "field",
    receiver_field: "value",
    prefixes: &["get", "is"],
};

pub(super) const ECMASCRIPT: MemberAccessorSyntax = MemberAccessorSyntax {
    member_kind: "member_expression",
    name_field: "property",
    receiver_field: "object",
    prefixes: &["get", "is", "has"],
};

impl MemberAccessorSyntax {
    pub(super) fn navigation_depth(&self, source: SourceNode<'_>) -> usize {
        let file = source.file();
        let node = source.node();
        if node.parent().is_some_and(|parent| {
            parent.kind() == "call_expression"
                && parent.child_by_field_name("function") == Some(node)
        }) {
            return 0;
        }
        let mut depth = 0;
        let mut current = Some(node);
        while let Some(node) = current.and_then(unwrap_parens) {
            let member = if node.kind() == "call_expression" {
                let Some(function) = node.child_by_field_name("function") else {
                    break;
                };
                let Some(name) = function.child_by_field_name(self.name_field) else {
                    break;
                };
                if !self
                    .prefixes
                    .iter()
                    .any(|prefix| file.text(name).starts_with(prefix))
                {
                    break;
                }
                function
            } else {
                node
            };
            if member.kind() != self.member_kind && member.kind() != "subscript_expression" {
                break;
            }
            depth += 1;
            current = member.child_by_field_name(self.receiver_field);
        }
        depth
    }

    pub(super) fn depth(&self, file: &SourceFile, node: Node<'_>) -> usize {
        let Some(function) = node.child_by_field_name("function") else {
            return 0;
        };
        if function.kind() != self.member_kind {
            return 0;
        }
        let Some(name) = function
            .child_by_field_name(self.name_field)
            .map(|field| file.text(field))
        else {
            return 0;
        };
        if !self.prefixes.iter().any(|prefix| name.starts_with(prefix)) {
            return 0;
        }
        let Some(receiver) = function
            .child_by_field_name(self.receiver_field)
            .and_then(unwrap_parens)
        else {
            return 1;
        };
        let depth = if receiver.kind() == "call_expression" {
            self.depth(file, receiver)
        } else {
            0
        };
        if depth == 0 { 1 } else { depth + 1 }
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
            .find(|child| child.kind() != "comment")?;
    }
    Some(node)
}

fn is_accessor(name: &str) -> bool {
    name.starts_with("get") || name.starts_with("is")
}

fn unwrap_receiver(node: Node<'_>) -> Option<Node<'_>> {
    let node = unwrap_parens(node)?;
    (node.kind() == "method_invocation").then_some(node)
}
