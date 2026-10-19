//! The Rust half of the smelly corpus: deliberately bad Rust, one concern per
//! file, frozen once written so it only ever grows.
//!
//! Unlike the Java half, there is no reference implementation behind these
//! expectations. They are reasoned from the fixture source and lock in
//! behaviour; they are not evidence that the metrics are right, and should not
//! be read as if they were.
//!
//! Where the two halves differ about a shape, they differ on purpose. The tests
//! that pin a difference say so, because a reader who has just learnt the Java
//! rule will otherwise assume it carries over.

mod common;

use common::{count_lines_starting_with, file, finding_keys_in};

#[test]
fn a_free_function_over_the_size_limit_is_reported() {
    // A free function belongs to no type, so it is reported under a pseudo-type
    // named after the file. 32 statements against a limit of 30, no branching.
    let expected = [(
        "long_by_size.rs".to_string(),
        "long_by_size".to_string(),
        vec![10],
    )];
    assert_eq!(
        finding_keys_in("smelly", "long_by_size.rs", "long-method"),
        expected.into_iter().collect()
    );
}

#[test]
fn an_expression_over_the_complexity_limit_is_reported() {
    // Fifteen `&&` and nothing else: complexity 16 against a limit of 15, in a
    // body holding a single expression. The Java metric counts a logical
    // operator only when an enclosing branch sits between it and the method;
    // this one counts it wherever it appears, which is the whole reason the
    // fixture has no `if` for a reader to misread as the cause.
    let expected = [(
        "long_by_branching.rs".to_string(),
        "long_by_branching".to_string(),
        vec![12],
    )];
    assert_eq!(
        finding_keys_in("smelly", "long_by_branching.rs", "long-method"),
        expected.into_iter().collect()
    );
}

#[test]
fn a_long_method_in_an_impl_block_is_reported_under_its_type() {
    // Rust keeps methods out of the type, in an `impl` block elsewhere in the
    // file, so the finding names `Accumulator` rather than `from_weights`. The
    // two short methods beside it stay out of the report.
    let expected = [(
        "long_impl.rs".to_string(),
        "Accumulator".to_string(),
        vec![24],
    )];
    assert_eq!(
        finding_keys_in("smelly", "long_impl.rs", "long-method"),
        expected.into_iter().collect()
    );
}

#[test]
fn a_long_method_on_an_enum_is_reported_under_the_enum() {
    // An enum is a type like any other for the impl lookup: the method is not
    // inside it, as it is in Java, but the finding still names the enum. The Java
    // corpus's EnumLong pins the same claim one AST level up.
    let expected = [("enum_long.rs".to_string(), "Rank".to_string(), vec![22])];
    assert_eq!(
        finding_keys_in("smelly", "enum_long.rs", "long-method"),
        expected.into_iter().collect()
    );
}

#[test]
fn a_long_method_called_new_is_reported() {
    // The deliberate opposite of the Java corpus's constructor trap. A Java
    // constructor is invisible however long it grows; Rust has no constructor in
    // that sense, so a long `fn new` is reported like any other method. Anyone
    // who carries the Java rule across would expect silence here.
    let expected = [("long_new.rs".to_string(), "Config".to_string(), vec![14])];
    assert_eq!(
        finding_keys_in("smelly", "long_new.rs", "long-method"),
        expected.into_iter().collect()
    );
}

#[test]
fn a_long_default_method_in_a_trait_is_reported_under_the_trait() {
    // A default method written in the trait is still a method of that trait.
    let expected = [(
        "long_trait_default.rs".to_string(),
        "Summarise".to_string(),
        vec![12],
    )];
    assert_eq!(
        finding_keys_in("smelly", "long_trait_default.rs", "long-method"),
        expected.into_iter().collect()
    );
}

#[test]
fn a_long_function_in_a_nested_module_is_invisible() {
    // The Rust reading of the Java nested-type trap: `types()` returns the file
    // and its top-level type items, and the file's free functions are its
    // *direct* children, so a function one module down is never reached.
    let file = file("smelly", "nested_module.rs");
    // Thirty increments, a declaration and a return: over the limit if it were
    // ever measured.
    assert_eq!(count_lines_starting_with(&file, "total +="), 30);

    assert!(finding_keys_in("smelly", "nested_module.rs", "long-method").is_empty());
}

#[test]
fn the_clean_control_stays_clean() {
    // Short in every shape the adapter has to map: an impl method, a trait
    // default method, a free function and a match.
    assert!(finding_keys_in("smelly", "clean.rs", "long-method").is_empty());
}
