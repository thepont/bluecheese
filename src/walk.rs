//! Tree traversal helpers shared by the detectors.

use tree_sitter::Node;

/// Depth-first pre-order walk over `node` and all of its descendants.
pub fn descendants<'tree>(node: Node<'tree>) -> Vec<Node<'tree>> {
    let mut out = Vec::new();
    let mut cursor = node.walk();
    let mut stack = vec![node];
    while let Some(n) = stack.pop() {
        out.push(n);
        // Push children in reverse so pop() yields them in source order.
        let children: Vec<Node<'tree>> = n.children(&mut cursor).collect();
        for child in children.into_iter().rev() {
            stack.push(child);
        }
    }
    out
}

/// The nearest ancestor of `node` (exclusive) that satisfies `pred`, stopping
/// (returning `None`) if `stop` matches first.
pub fn ancestor_while<'tree>(
    node: Node<'tree>,
    mut stop: impl FnMut(Node<'tree>) -> bool,
    mut pred: impl FnMut(Node<'tree>) -> bool,
) -> Option<Node<'tree>> {
    let mut current = node.parent();
    while let Some(n) = current {
        if stop(n) {
            return None;
        }
        if pred(n) {
            return Some(n);
        }
        current = n.parent();
    }
    None
}
