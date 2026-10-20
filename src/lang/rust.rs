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

use crate::lang::{LanguageId, LanguageModel, SourceFile, SourceNode};
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
    fn field_access_parts<'a>(&self, node: Node<'a>) -> Option<(Node<'a>, Node<'a>)> {
        if node.kind() != "field_expression" {
            return None;
        }
        Some((
            node.child_by_field_name("value")?,
            node.child_by_field_name("field")?,
        ))
    }

    fn is_statement(&self, node: Node<'_>) -> bool {
        if matches!(
            node.kind(),
            "return_expression" | "break_expression" | "continue_expression"
        ) {
            return node
                .parent()
                .is_none_or(|parent| parent.kind() != "expression_statement");
        }
        matches!(node.kind(), "let_declaration" | "expression_statement")
    }

    fn method_coupling(&self, files: &[SourceFile]) -> Vec<crate::lang::coupling::MethodCoupling> {
        crate::lang::rust_calls::coupling(files)
    }

    fn accessor_chain_depth(&self, source: SourceNode<'_>) -> usize {
        let SourceNode { file, node } = source;
        if node.kind() != "call_expression" {
            return 0;
        }
        crate::lang::accessors::RUST.depth(file, node)
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
        let mut out = vec![root];
        let mut cursor = root.walk();
        for child in root.named_children(&mut cursor) {
            if TYPE_KINDS.contains(&child.kind()) {
                out.push(child);
            }
        }
        out
    }

    fn type_name<'a>(&self, source: SourceNode<'a>) -> Option<&'a str> {
        let SourceNode { file, node: ty } = source;
        if ty.kind() == "source_file" {
            return file.path.file_stem().and_then(|s| s.to_str());
        }
        ty.child_by_field_name("name").map(|n| file.text(n))
    }

    fn fields<'a>(&self, ty: Node<'a>) -> Vec<Node<'a>> {
        self.body_children(ty)
            .unwrap_or_default()
            .into_iter()
            .filter(|field| field.kind() == "field_declaration")
            .collect()
    }

    fn supports_primitive_fields(&self, kind: &str) -> bool {
        kind == "struct_item"
    }

    fn is_primitive_type(&self, raw: &str) -> bool {
        matches!(
            super::primitive_type_name(raw),
            "u8" | "u16"
                | "u32"
                | "u64"
                | "u128"
                | "usize"
                | "i8"
                | "i16"
                | "i32"
                | "i64"
                | "i128"
                | "isize"
                | "f32"
                | "f64"
                | "bool"
                | "char"
        )
    }

    fn methods<'a>(&self, source: SourceNode<'a>) -> Vec<Node<'a>> {
        if source.node().kind() == "source_file" {
            return free_functions(source.node());
        }
        let mut methods = implementation_methods(source);
        methods.extend(trait_default_methods(source.node()));
        methods
    }

    /// Rust has no constructors: `fn new` is an ordinary function returning
    /// the type, and nothing in the grammar marks it as special. A type
    /// therefore has none, and the constructor threshold never applies.
    fn constructors<'a>(&self, _source: SourceNode<'a>) -> Vec<Node<'a>> {
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
    fn is_override(&self, source: SourceNode<'_>) -> bool {
        let SourceNode { file: _file, node } = source;
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
    fn lloc(&self, source: SourceNode<'_>) -> usize {
        let SourceNode {
            file: _file,
            node: body,
        } = source;
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
    fn complexity(&self, source: SourceNode<'_>) -> usize {
        let SourceNode { file, node: method } = source;
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

fn free_functions(root: Node<'_>) -> Vec<Node<'_>> {
    let mut cursor = root.walk();
    root.named_children(&mut cursor)
        .filter(|node| node.kind() == "function_item")
        .collect()
}

fn implementation_methods(source: SourceNode<'_>) -> Vec<Node<'_>> {
    let name = RUST
        .type_name(source)
        .expect("a type returned by types() has a name");
    descendants(source.file().root())
        .into_iter()
        .filter(|node| node.kind() == "impl_item")
        .filter_map(|node| implementation_body(source.file().at(node), name))
        .flat_map(functions_in)
        .collect()
}

/// An impl needs both its target and body; incomplete syntax contributes no methods.
fn implementation_body<'a>(source: SourceNode<'a>, name: &str) -> Option<Node<'a>> {
    let target = source.node().child_by_field_name("type")?;
    let body = source.node().child_by_field_name("body")?;
    impl_targets(source.file().text(target), name).then_some(body)
}

/// Traits always have a body; absence indicates a grammar contract failure.
fn trait_default_methods(ty: Node<'_>) -> Vec<Node<'_>> {
    if ty.kind() != "trait_item" {
        return Vec::new();
    }
    functions_in(
        ty.child_by_field_name("body")
            .expect("a trait_item has a body"),
    )
}
