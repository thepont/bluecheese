//! Port of `edu.northeastern.utils.Metrics`.
//!
//! Only the metrics the ported detectors need. Each function mirrors the Java
//! implementation, and `tests/parity.rs` holds it to Spoon's own output.

use tree_sitter::Node;

use crate::lang::SourceFile;
use crate::walk::{ancestor_while, descendants};

/// Node kinds that correspond to Spoon `CtStatement` elements. Spoon excludes
/// nested `CtBlock`s, so block nodes are not listed; and because Spoon has no
/// expression-statement wrapper, the *inner* expression kinds
/// (`method_invocation`, `object_creation_expression`, `assignment_expression`)
/// are counted instead of `expression_statement`. Spoon also counts nested
/// invocations and constructor calls, which is why these are matched anywhere
/// in the body rather than only at statement position.
///
/// Notably absent: `catch_clause` (Spoon `CtCatch` is not a `CtStatement`).
/// `update_expression` and `unary_expression` *are* counted, matching Spoon's
/// `CtUnaryOperator` (`i++`, `!x`).
const STATEMENT_KINDS: &[&str] = &[
    "method_invocation",
    "object_creation_expression",
    "explicit_constructor_invocation",
    "assignment_expression",
    "local_variable_declaration",
    "resource",
    "update_expression",
    "unary_expression",
    "line_comment",
    "block_comment",
    "if_statement",
    "for_statement",
    "enhanced_for_statement",
    "while_statement",
    "do_statement",
    "switch_expression",
    "switch_label",
    "switch_rule",
    "return_statement",
    "throw_statement",
    "break_statement",
    "continue_statement",
    "yield_statement",
    "try_statement",
    "try_with_resources_statement",
    "synchronized_statement",
    "labeled_statement",
    "assert_statement",
];

/// Decision points counted by Spoon: `CtIf`, `CtFor`, `CtForEach`, `CtWhile`,
/// `CtDo`, `CtConditional`, `CtCatch`, `CtThrow`.
const DECISION_KINDS: &[&str] = &[
    "if_statement",
    "for_statement",
    "enhanced_for_statement",
    "while_statement",
    "do_statement",
    "ternary_expression",
    "catch_clause",
    "throw_statement",
];

const METHOD_KINDS: &[&str] = &["method_declaration", "constructor_declaration"];
const LOOP_OR_BRANCH_KINDS: &[&str] = &[
    "if_statement",
    "for_statement",
    "enhanced_for_statement",
    "while_statement",
    "do_statement",
    "ternary_expression",
];

/// `Metrics.calculateLLOC(CtExecutable)`.
///
/// Logical lines of code: every statement under the body, excluding blocks.
/// A for-each loop additionally contributes its implicit loop variable, which
/// Spoon materialises as a `CtLocalVariable`.
pub fn lloc(_file: &SourceFile, body: Node<'_>) -> usize {
    let nodes = descendants(body);
    let statements = nodes
        .iter()
        .filter(|n| STATEMENT_KINDS.contains(&n.kind()))
        .count();
    let implicit_loop_variables = nodes
        .iter()
        .filter(|n| n.kind() == "enhanced_for_statement")
        .count();
    statements + implicit_loop_variables
}

/// `Metrics.calculateCyclomaticComplexity(CtMethod)`.
pub fn cyclomatic_complexity(file: &SourceFile, method: Node<'_>) -> usize {
    let mut complexity = 1;

    for node in descendants(method) {
        let kind = node.kind();
        if DECISION_KINDS.contains(&kind) {
            complexity += 1;
        }
        if kind == "switch_label" && !matches!(file.text(node).trim(), "default:" | "default") {
            complexity += 1;
        }
        if kind == "binary_expression" {
            let op = node.child_by_field_name("operator").map(|n| file.text(n));
            if matches!(op, Some("&&") | Some("||")) {
                let enclosing = ancestor_while(
                    node,
                    |n| METHOD_KINDS.contains(&n.kind()),
                    |n| LOOP_OR_BRANCH_KINDS.contains(&n.kind()),
                );
                if enclosing.is_some() {
                    complexity += 1;
                }
            }
        }
    }

    complexity
}
