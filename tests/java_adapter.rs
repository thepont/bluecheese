//! Characterisation of the Java adapter: which declarations are types, which are
//! methods, the line a declaration is reported on, and what counts as a decision.
//!
//! SmellBench is real code, and `parity.rs` holds the metrics to Spoon's own
//! output over it. But it contains no nested type, no enum method, no record and
//! no annotated declaration, so those shapes are not reached there. These
//! expectations are reasoned from Spoon's behaviour (see `reference/README.md`)
//! and lock in what this adapter does; they are not evidence about Spoon.

mod common;

use std::collections::BTreeMap;

use bluecheese::lang::{SourceFile, model_for};
use common::{file, line_of};

/// Top-level type names, in source order, as `CtModel.getAllTypes()` returns
/// them: nested declarations never appear in their own right.
fn type_names(file: &SourceFile) -> Vec<String> {
    let model = model_for(file.language);
    model
        .types(file)
        .into_iter()
        .map(|ty| {
            model
                .type_name(file.at(ty))
                .expect("a top-level type has a name")
                .to_string()
        })
        .collect()
}

/// Method names per type, so a test can see exactly what a type is credited with.
fn methods_by_type(file: &SourceFile) -> BTreeMap<String, Vec<String>> {
    let model = model_for(file.language);
    let mut by_type = BTreeMap::new();
    for ty in model.types(file) {
        let type_name = model
            .type_name(file.at(ty))
            .expect("type has a name")
            .to_string();
        let methods = model
            .methods(file, ty)
            .into_iter()
            .map(|method| {
                let name = method
                    .child_by_field_name("name")
                    .expect("method has a name");
                file.text(name).to_string()
            })
            .collect();
        by_type.insert(type_name, methods);
    }
    by_type
}

/// Method names of a declaration that `types()` does not return, found by name.
/// A nested type is not a type of its own, but the adapter still has to read its
/// members: enum, record and nested-class members each sit somewhere different.
fn methods_of_declaration(file: &SourceFile, type_name: &str) -> Vec<String> {
    let model = model_for(file.language);
    let declaration = bluecheese::walk::descendants(file.root())
        .into_iter()
        .find(|node| {
            node.kind().ends_with("_declaration")
                && node.child_by_field_name("name").map(|name| file.text(name)) == Some(type_name)
        })
        .unwrap_or_else(|| panic!("no declaration named {type_name}"));
    model
        .methods(file, declaration)
        .into_iter()
        .map(|method| {
            let name = method
                .child_by_field_name("name")
                .expect("method has a name");
            file.text(name).to_string()
        })
        .collect()
}

/// Logical lines and cyclomatic complexity of the method called `method_name`.
fn metrics_of(file: &SourceFile, method_name: &str) -> (usize, usize) {
    let model = model_for(file.language);
    for ty in model.types(file) {
        for method in model.methods(file, ty) {
            let name = method
                .child_by_field_name("name")
                .expect("method has a name");
            if file.text(name) == method_name {
                let body = model.body(method).expect("method has a body");
                return (model.lloc(file, body), model.complexity(file, method));
            }
        }
    }
    panic!("no method named '{method_name}'");
}

#[test]
fn reports_top_level_types_only_and_in_source_order() {
    let shape = file("java-adapter", "Shape.java");

    // `Inner`, `Kind` and `Point` are nested, so they are absent by design.
    assert_eq!(
        type_names(&shape),
        ["Marker", "Shape", "Bare", "Measurable"]
    );
}

#[test]
fn does_not_mistake_constructors_or_nested_types_for_methods() {
    let shape = file("java-adapter", "Shape.java");
    let by_type = methods_by_type(&shape);

    // A constructor is not a method, and the nested class is not a method either.
    assert_eq!(by_type["Shape"], ["sides"]);
    // An annotation type declares no methods, and must not blow up.
    assert_eq!(by_type["Marker"], Vec::<String>::new());
    // An abstract method is listed, even though it has no body to measure.
    assert_eq!(by_type["Measurable"], ["size", "twice"]);

    // Top-level types are all `types()` returns, but the adapter still has to
    // read the members of the ones nested in them: an enum's methods sit under
    // `enum_body_declarations`, a record's and a class's one level up from there.
    assert_eq!(methods_of_declaration(&shape, "Kind"), ["isFlat"]);
    assert_eq!(methods_of_declaration(&shape, "Point"), ["sum"]);
    assert_eq!(methods_of_declaration(&shape, "Inner"), ["hide"]);
}

#[test]
fn reports_a_declaration_at_its_own_line_not_at_its_annotations() {
    let shape = file("java-adapter", "Shape.java");
    let model = model_for(shape.language);

    let annotated = model
        .types(&shape)
        .into_iter()
        .find(|ty| model.type_name(shape.at(*ty)) == Some("Shape"))
        .expect("Shape is a type");
    let shape_line = line_of(&shape, "public class Shape");
    assert_eq!(model.decl_line(annotated), shape_line);
    assert!(
        shape
            .source
            .lines()
            .nth(shape_line - 2)
            .unwrap()
            .contains("@Deprecated"),
        "the annotation sits on the line above; the declaration line must skip it"
    );

    // When the only modifier is an annotation there is no keyword to stop at, so
    // the position has to continue to the next child.
    let bare = model
        .types(&shape)
        .into_iter()
        .find(|ty| model.type_name(shape.at(*ty)) == Some("Bare"))
        .expect("Bare is a type");
    let bare_line = line_of(&shape, "class Bare");
    assert_eq!(model.decl_line(bare), bare_line);
    assert!(
        shape
            .source
            .lines()
            .nth(bare_line - 2)
            .unwrap()
            .contains("@Marker"),
        "Bare is annotated on the line above its declaration"
    );
}

#[test]
fn a_node_with_no_children_reports_its_own_line() {
    let shape = file("java-adapter", "Shape.java");
    let model = model_for(shape.language);

    // An identifier is a leaf: the search for something to stop at finds
    // nothing, so the node's own position has to be the answer.
    let identifier = bluecheese::walk::descendants(shape.root())
        .into_iter()
        .find(|node| node.kind() == "identifier")
        .expect("the fixture has identifiers");

    assert_eq!(model.decl_line(identifier), shape.line(identifier));
}

#[test]
fn a_logical_operator_is_a_decision_only_inside_a_branch() {
    let logic = file("java-adapter", "Logic.java");

    // `a > 0 && a < 10` with nothing enclosing it is a single comparison, not
    // three decisions. Getting this wrong would inflate complexity everywhere.
    assert_eq!(metrics_of(&logic, "bare").1, 1);
    // Inside an `if`, the `if` and both operators each count: 1 + 1 + 2.
    assert_eq!(metrics_of(&logic, "guarded").1, 4);
}
