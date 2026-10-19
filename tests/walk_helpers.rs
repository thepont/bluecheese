//! The traversal helpers, on their own.
//!
//! The Java adapter's complexity rule turns on both of these: a logical operator
//! counts only when a branch sits between it and the method, which is a search
//! up the tree that has to take the nearest branch and stop at the method. The
//! corpus tests cover the rule; these cover the search it is built from.

mod common;

use bluecheese::lang::SourceFile;
use bluecheese::walk::{ancestor_while, descendants};
use common::{line_of, parse};
use tree_sitter::Node;

/// A method holding an `if` inside an `if`, plus a call outside both.
fn parsed() -> SourceFile {
    parse(
        "Walk.java",
        r#"class Walk {
    void run() {
        if (a) {
            if (b) {
                deep();
            }
        }
        shallow();
    }
}
"#,
    )
}

/// The identifier spelled `name`.
fn node_named<'a>(file: &'a SourceFile, name: &str) -> Node<'a> {
    descendants(file.root())
        .into_iter()
        .find(|node| node.kind() == "identifier" && file.text(*node) == name)
        .unwrap_or_else(|| panic!("no identifier named {name}"))
}

#[test]
fn descendants_are_pre_order_and_include_the_root() {
    let file = parsed();
    let walked = descendants(file.root());

    assert_eq!(
        walked.first().map(Node::kind),
        Some("program"),
        "a walk of a node starts at that node"
    );

    // Source order, which for a caller means "the identifier of the enclosing
    // declaration comes before anything it contains".
    let names: Vec<&str> = walked
        .iter()
        .filter(|node| node.kind() == "identifier")
        .map(|node| file.text(*node))
        .collect();
    assert_eq!(names, ["Walk", "run", "a", "b", "deep", "shallow"]);
}

#[test]
fn the_nearest_matching_ancestor_wins() {
    let file = parsed();
    let deep = node_named(&file, "deep");

    let nearest = ancestor_while(deep, |_| false, |node| node.kind() == "if_statement")
        .expect("`deep()` is inside two `if`s");

    assert_eq!(
        file.line(nearest),
        line_of(&file, "if (b)"),
        "the inner `if` is the branch that encloses the call, not the outer one"
    );
}

#[test]
fn a_stop_that_comes_first_wins_over_a_match_further_out() {
    let file = parsed();
    let deep = node_named(&file, "deep");

    // This is the rule in miniature: an operator inside a method but outside any
    // branch must not be credited to a branch it cannot see, so the search stops
    // at the method boundary and reports nothing.
    let found = ancestor_while(
        deep,
        |node| node.kind() == "if_statement",
        |node| node.kind() == "method_declaration",
    );

    assert_eq!(found, None);
}

#[test]
fn nothing_matching_gives_nothing() {
    let file = parsed();
    let shallow = node_named(&file, "shallow");

    let found = ancestor_while(shallow, |_| false, |node| node.kind() == "if_statement");

    assert_eq!(
        found, None,
        "a call outside every branch has no branch above it"
    );
}
