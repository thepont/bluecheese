//! Rust adapter.
//!
//! There is **no reference implementation** for Rust smells: SmellBench and the
//! Java tool it feeds are Java-only. The metrics below are therefore *our*
//! definition, chosen as the natural analogue of the Java ones. The tests in
//! `tests/rust_adapter.rs` lock in observed behaviour; they do not prove parity
//! with another tool, and should not be read as if they did.
//!
//! Two mappings are worth stating because Rust does not look like Java:
//!
//! - Methods do not live in the type. They live in `impl` blocks, which may sit
//!   anywhere in the file, so a type's methods are found by scanning for `impl`
//!   blocks whose target is that type.
//! - Free functions belong to no type. They are reported under a pseudo-type
//!   named after the file, so that a module of free functions is still
//!   analysed rather than silently dropped.

use tree_sitter::{Language, Node};

use crate::lang::{LanguageId, LanguageModel, SourceFile};
use crate::walk::{ancestor_while, descendants};

/// Item kinds that declare a type.
const TYPE_KINDS: &[&str] = &["struct_item", "enum_item", "trait_item", "union_item"];

/// Branch points. Rust has no ternary; `if` covers it. `try_expression` is the
/// `?` operator, which is a branch.
const DECISION_KINDS: &[&str] = &[
    "if_expression",
    "for_expression",
    "while_expression",
    "loop_expression",
    "try_expression",
];

pub struct RustModel;

pub static RUST: RustModel = RustModel;

impl LanguageModel for RustModel {
    fn accessor_chain_depth(&self, file: &SourceFile, node: Node<'_>) -> usize {
        if node.kind() != "call_expression" {
            return 0;
        }
        crate::lang::accessors::rust_accessor_depth(file, node)
    }

    fn is_comment(&self, node: Node<'_>) -> bool {
        matches!(node.kind(), "line_comment" | "block_comment")
    }

    fn id(&self) -> LanguageId {
        LanguageId::Rust
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["rs"]
    }

    fn grammar(&self) -> Language {
        tree_sitter_rust::LANGUAGE.into()
    }

    fn types<'a>(&self, file: &'a SourceFile) -> Vec<Node<'a>> {
        let root = file.root();
        // The file itself acts as the module, so free functions are analysable.
        let mut out = vec![root];
        let mut cursor = root.walk();
        for child in root.named_children(&mut cursor) {
            if TYPE_KINDS.contains(&child.kind()) {
                out.push(child);
            }
        }
        out
    }

    fn type_name<'a>(&self, file: &'a SourceFile, ty: Node<'a>) -> Option<&'a str> {
        if ty.kind() == "source_file" {
            return file.path.file_stem().and_then(|s| s.to_str());
        }
        ty.child_by_field_name("name").map(|n| file.text(n))
    }

    fn methods<'a>(&self, file: &'a SourceFile, ty: Node<'a>) -> Vec<Node<'a>> {
        let mut out = Vec::new();

        // Free functions: direct children of the file.
        if ty.kind() == "source_file" {
            let mut cursor = ty.walk();
            for child in ty.named_children(&mut cursor) {
                if child.kind() == "function_item" {
                    out.push(child);
                }
            }
            return out;
        }

        // Methods of a type: functions inside `impl` blocks targeting it.
        //
        // Every type `types()` returns above the file itself is a struct, enum,
        // trait or union, all of which name themselves, so the name is there.
        // Asking for it as an option and skipping when it is absent would be a
        // branch no input can reach.
        let name = self
            .type_name(file, ty)
            .expect("a type returned by types() has a name");
        for node in descendants(file.root()) {
            if node.kind() != "impl_item" {
                continue;
            }
            // A malformed `impl Foo` with no braces has a target but no
            // body, so both fields are required and either absence means
            // the same thing: there is nothing here to collect.
            let (Some(target), Some(body)) = (
                node.child_by_field_name("type"),
                node.child_by_field_name("body"),
            ) else {
                continue;
            };
            if !impl_targets(file.text(target), name) {
                continue;
            }
            out.extend(functions_in(body));
        }

        // Default methods declared in the trait itself. Every trait has a body
        // — a trait with none is not valid Rust — so this is not an optional
        // step, and a missing body would be a grammar change worth hearing
        // about rather than a trait that quietly has no methods.
        if ty.kind() == "trait_item" {
            let described = ty
                .child_by_field_name("body")
                .expect("a trait_item has a body");
            out.extend(functions_in(described));
        }

        out
    }

    fn constructors<'a>(&self, _file: &SourceFile, _ty: Node<'a>) -> Vec<Node<'a>> {
        // Rust has no constructors: `fn new` is an ordinary function returning
        // the type, and nothing in the grammar marks it as special. A type
        // therefore has none, and the constructor threshold never applies.
        Vec::new()
    }

    /// Declared parameters, less the receiver. `&self` is not something a
    /// caller writes at the call site, and Java's implicit `this` is not
    /// counted either, so leaving it in would make the two adapters disagree
    /// about a method that looks the same to a reader.
    fn parameters(&self, node: Node<'_>) -> usize {
        let params = node
            .child_by_field_name("parameters")
            .expect("a function declaration has a parameter list");
        let mut cursor = params.walk();
        params
            .named_children(&mut cursor)
            .filter(|child| child.kind() != "self_parameter")
            .count()
    }

    /// A function written inside `impl Trait for Type` restates the trait's
    /// signature, which is what `@Override` marks in Java. A default method
    /// declared in the trait itself is not: it is the declaration.
    fn is_override(&self, _file: &SourceFile, node: Node<'_>) -> bool {
        ancestor_while(node, |_| false, |ancestor| ancestor.kind() == "impl_item")
            .is_some_and(|imp| imp.child_by_field_name("trait").is_some())
    }

    fn body<'a>(&self, node: Node<'a>) -> Option<Node<'a>> {
        node.child_by_field_name("body")
    }

    /// Line of a declaration, which is its own line: in this grammar an
    /// attribute is a sibling of the item it annotates, never a child of it, so
    /// nothing has to be walked past. The Java adapter has to skip annotations
    /// because tree-sitter-java nests them, and this one deliberately does not
    /// guess at a shape its own grammar never produces.
    fn decl_line(&self, node: Node<'_>) -> usize {
        node.start_position().row + 1
    }

    /// Logical lines of code: the number of statements, summed over every block
    /// in the body. A statement is any named child of a block that is not a
    /// comment, which counts nested statements without counting the expressions
    /// inside them. The Java metric is far more inclusive (Spoon counts nested
    /// invocations); this one is deliberately simpler, since there is no
    /// reference to match.
    fn lloc(&self, _file: &SourceFile, body: Node<'_>) -> usize {
        descendants(body)
            .into_iter()
            .filter(|n| n.kind() == "block")
            .map(|block| {
                let mut cursor = block.walk();
                block
                    .named_children(&mut cursor)
                    .filter(|c| !matches!(c.kind(), "line_comment" | "block_comment"))
                    .count()
            })
            .sum()
    }

    /// Cyclomatic complexity: one plus every branch point. A `match` arm is a
    /// branch, except the wildcard arm, which is the fall-through case rather
    /// than a decision.
    fn complexity(&self, file: &SourceFile, method: Node<'_>) -> usize {
        let mut complexity = 1;
        for node in descendants(method) {
            let kind = node.kind();
            if DECISION_KINDS.contains(&kind) {
                complexity += 1;
            }
            if kind == "match_arm" {
                // The wildcard arm is the fall-through case, not a decision.
                // tree-sitter-rust does not always give `_` a distinct kind, so
                // the text is checked as well as the kind.
                let is_wildcard = node
                    .child_by_field_name("pattern")
                    .is_some_and(|p| p.kind() == "wildcard_pattern" || file.text(p).trim() == "_");
                if !is_wildcard {
                    complexity += 1;
                }
            }
            if kind == "binary_expression" {
                let op = node.child_by_field_name("operator").map(|n| file.text(n));
                if matches!(op, Some("&&") | Some("||")) {
                    complexity += 1;
                }
            }
        }
        complexity
    }
}

/// Functions directly inside an `impl` or trait body.
///
/// A trait may declare a method without a body, and tree-sitter gives that a
/// different kind from a function definition. Both are methods of the trait: a
/// measure that only counted the ones with bodies would make a trait of
/// signatures look like a trait with no methods at all, which is not what the
/// Java adapter does with an interface's abstract methods. Nothing can be
/// measured in a bodyless method, and the detectors skip it for that reason, not
/// this one.
fn functions_in<'a>(body: Node<'a>) -> Vec<Node<'a>> {
    let mut cursor = body.walk();
    body.named_children(&mut cursor)
        .filter(|c| matches!(c.kind(), "function_item" | "function_signature_item"))
        .collect()
}

/// Whether an `impl` target names `name`. Targets may be paths or generic, so
/// `crate::Foo<T>` and `Foo` both name `Foo`.
fn impl_targets(target: &str, name: &str) -> bool {
    let base = target.split('<').next().unwrap_or(target);
    let base = base.rsplit("::").next().unwrap_or(base);
    base.trim() == name
}
