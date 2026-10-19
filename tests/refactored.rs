//! The refactoring pairs: a smelly fixture beside the fixture that fixes it.
//!
//! Each pair is a refactoring from *Refactoring: Improving the Design of
//! Existing Code* applied deliberately, so the suite can ask the question a
//! corpus of smelly code alone cannot answer: does the detector track the smell,
//! or only the shape of the code it usually hides in? A finding that survives
//! its own refactoring is a finding that will keep being reported on code nobody
//! would call smelly.
//!
//! Behavioural equality of the two halves is asserted by construction, in the
//! fixtures' own comments, and not by execution: nothing here is compiled or
//! run, and no test below should be read as having checked it. What the tests do
//! check is the only thing this tool can be wrong about — what it reports.
//!
//! Each pair exists in both languages. The Java halves are held to the reference
//! tool's report by `parity.rs`; the Rust halves cannot be, because there is no
//! reference tool for Rust. Those expectations are read off the fixture source,
//! and lock in that a refactoring which works on Java works on the shapes Rust
//! writes as well — which is a weaker claim, and is labelled as one.

mod common;

use common::finding_keys_in;

#[test]
fn extract_method_removes_the_finding() {
    // The book's primary answer to Long Method. The work is identical on both
    // sides; only the division into methods changes.
    let expected = [(
        "ExtractMe.java".to_string(),
        "ExtractMe".to_string(),
        vec![14],
    )];
    assert_eq!(
        finding_keys_in("smelly", "ExtractMe.java"),
        expected.into_iter().collect()
    );

    assert!(
        finding_keys_in("refactored", "ExtractMe.java").is_empty(),
        "the same work, in methods that each say what they do, is not a smell"
    );
}

#[test]
fn replace_conditional_with_polymorphism_removes_the_finding() {
    // The book's answer to Switch Statements, which it also lists as a cause of
    // Long Method: the type code and the switch go, and each kind answers for
    // itself. The two halves add up to the same three numbers.
    let expected = [(
        "SwitchOnKind.java".to_string(),
        "SwitchOnKind".to_string(),
        vec![22],
    )];
    assert_eq!(
        finding_keys_in("smelly", "SwitchOnKind.java"),
        expected.into_iter().collect()
    );

    assert!(
        finding_keys_in("refactored", "SwitchOnKind.java").is_empty(),
        "a switch that has become dispatch is not a smell"
    );
}

#[test]
fn replace_method_with_method_object_moves_the_finding_instead() {
    // The book is explicit that this refactoring is a step and not a cure: the
    // locals become fields of a new object, and the long body moves with them.
    // It is still long, so it is still reported — now under the object that
    // holds it. A detector that went quiet here would be reporting the shape of
    // the class rather than the state of the code.
    let before = [(
        "ManyTemps.java".to_string(),
        "ManyTemps".to_string(),
        vec![11],
    )];
    assert_eq!(
        finding_keys_in("smelly", "ManyTemps.java"),
        before.into_iter().collect()
    );

    let after = [(
        "ManyTemps.java".to_string(),
        "ManyTempsCalculator".to_string(),
        vec![37],
    )];
    assert_eq!(
        finding_keys_in("refactored", "ManyTemps.java"),
        after.into_iter().collect(),
        "the method object has one short method and one that is still the old body"
    );
}

#[test]
fn extract_method_removes_the_finding_in_rust_as_well() {
    // The same refactoring on the Rust half of the corpus, where the two jobs are
    // an accumulation into a field and a rendering of labels. The Java test above
    // is evidence about the port; this one only locks in behaviour.
    let expected = [(
        "extract_me.rs".to_string(),
        "ExtractMe".to_string(),
        vec![15],
    )];
    assert_eq!(
        finding_keys_in("smelly", "extract_me.rs"),
        expected.into_iter().collect()
    );

    assert!(
        finding_keys_in("refactored", "extract_me.rs").is_empty(),
        "the same total and the same labels, in methods that each say what they do"
    );
}

#[test]
fn replace_conditional_with_polymorphism_removes_the_finding_in_rust_as_well() {
    // Rust writes this one differently: a trait and a type per variant, rather
    // than an abstract class and subclasses. The result is the same — no match,
    // and every method in the file short.
    let expected = [(
        "switch_on_kind.rs".to_string(),
        "SwitchOnKind".to_string(),
        vec![24],
    )];
    assert_eq!(
        finding_keys_in("smelly", "switch_on_kind.rs"),
        expected.into_iter().collect()
    );

    assert!(
        finding_keys_in("refactored", "switch_on_kind.rs").is_empty(),
        "a match that has become dispatch is not a smell"
    );
}

#[test]
fn replace_method_with_method_object_moves_the_finding_in_rust_as_well() {
    // The same step and not a cure: the locals become fields of a new type, and
    // the long body goes with them, still long, now reported under the object
    // that holds it.
    let before = [(
        "many_temps.rs".to_string(),
        "ManyTemps".to_string(),
        vec![13],
    )];
    assert_eq!(
        finding_keys_in("smelly", "many_temps.rs"),
        before.into_iter().collect()
    );

    let after = [(
        "many_temps.rs".to_string(),
        "ManyTempsCalculator".to_string(),
        vec![45],
    )];
    assert_eq!(
        finding_keys_in("refactored", "many_temps.rs"),
        after.into_iter().collect(),
        "the method object has one short method and one that is still the old body"
    );
}
