//! Diagnostic: report parse problems per file.
//!
//! Tree-sitter recovers from malformed input, so a file that does not parse
//! yields *fewer findings* rather than an error, which is indistinguishable from
//! clean code. This makes the difference visible.
//!
//! Run: `cargo run --example parse_check -- <dir>`

use std::path::Path;

use bluecheese::lang::load_project;
use tree_sitter::Node;

/// Count `ERROR` and `MISSING` nodes anywhere in the subtree.
fn problems(node: Node<'_>) -> (usize, usize) {
    let (mut errors, mut missing) = (0, 0);
    let mut stack = vec![node];
    while let Some(n) = stack.pop() {
        if n.is_error() {
            errors += 1;
        }
        if n.is_missing() {
            missing += 1;
        }
        let mut cursor = n.walk();
        stack.extend(n.children(&mut cursor).collect::<Vec<_>>());
    }
    (errors, missing)
}

fn main() {
    let root = std::env::args().nth(1).expect("usage: parse_check <dir>");
    let project = load_project(Path::new(&root)).expect("load project");
    let files = project.files;

    let mut flagged = 0;
    for file in &files {
        let (errors, missing) = problems(file.root());
        if errors + missing > 0 {
            flagged += 1;
            println!("{}\t{errors} error node(s), {missing} missing", file.rel);
        }
    }

    println!("{} files, {} with parse problems", files.len(), flagged);
}
