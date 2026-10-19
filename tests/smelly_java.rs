//! The smelly corpus: deliberately bad Java, one concern per file, frozen once
//! written so it only ever grows.
//!
//! A corpus of only smelly code cannot tell a working detector from one that
//! reports everything, so `Clean.java` sits alongside it as the control, and the
//! traps below are code that *looks* reportable and must not be.
//!
//! Each test checks one file on its own. Adding an example means adding a test,
//! never rewriting the expectations an earlier example already pinned.

mod common;

use common::{count_lines_starting_with, file, finding_keys_in};

#[test]
fn a_method_over_the_size_limit_is_reported() {
    // `LongBySize.record` is 31 invocations in a row: over the limit on size
    // alone, with no branching at all.
    let expected = [(
        "LongBySize.java".to_string(),
        "LongBySize".to_string(),
        vec![12],
    )];
    assert_eq!(
        finding_keys_in("smelly", "LongBySize.java", "long-method"),
        expected.into_iter().collect()
    );
}

#[test]
fn one_file_can_trip_either_limit() {
    // `applyShipment` is over the size limit alone and `inRange` is over the
    // complexity limit alone; both are reported, from the same type.
    let expected = [(
        "LongMethod.java".to_string(),
        "LongMethod".to_string(),
        vec![23, 67],
    )];
    assert_eq!(
        finding_keys_in("smelly", "LongMethod.java", "long-method"),
        expected.into_iter().collect()
    );
}

#[test]
fn a_long_constructor_is_not_a_method() {
    // The constructor is 31 statements, comfortably over the limit, and must be
    // invisible: Spoon's getMethods() returns methods, not constructors.
    let file = file("smelly", "LongConstructor.java");
    assert!(count_lines_starting_with(&file, "this.total") > 30);

    assert!(finding_keys_in("smelly", "LongConstructor.java", "long-method").is_empty());
}

#[test]
fn a_long_method_in_a_nested_type_is_invisible() {
    // The inner method is 31 statements, and unattributable: Spoon's
    // getAllTypes() returns top-level types only, so its methods are never
    // examined. Were this adapter to walk types recursively it would report
    // line 20 and the parity harness would catch it on real code.
    let file = file("smelly", "NestedLong.java");
    // Thirty increments, plus a declaration and a return: the same body shape
    // measures 32 logical lines in `EnumLong.java`, so this really is smelly.
    assert_eq!(count_lines_starting_with(&file, "total +="), 30);

    assert!(finding_keys_in("smelly", "NestedLong.java", "long-method").is_empty());
}

#[test]
fn a_long_method_in_a_top_level_enum_is_reported() {
    // The counterpart to the nested case: an enum is a top-level type, so it is
    // reported even though its body sits one level deeper.
    let expected = [(
        "EnumLong.java".to_string(),
        "EnumLong".to_string(),
        vec![11],
    )];
    assert_eq!(
        finding_keys_in("smelly", "EnumLong.java", "long-method"),
        expected.into_iter().collect()
    );
}

#[test]
fn a_method_and_a_constructor_have_different_parameter_limits() {
    // Two limits, and the constructor's is the higher one: three parameters is
    // fine for a method and fine for a constructor, six is too many for either.
    // Lines 24 and 12 are the compliant pair and must stay out.
    //
    // The order is the reference tool's: every method the type declares, then
    // its constructors, which is not source order here.
    let expected = [(
        "LongParams.java".to_string(),
        "LongParams".to_string(),
        vec![28, 32, 20],
    )];
    assert_eq!(
        finding_keys_in("smelly", "LongParams.java", "long-params"),
        expected.into_iter().collect()
    );
}

#[test]
fn an_override_is_not_reported_where_the_declaration_it_restates_is() {
    // Line 16 restates `FourArgs.four` and is left out; line 27 is that
    // declaration, and line 20 is `own`, which chose its own parameter list.
    let expected = [
        (
            "OverrideParams.java".to_string(),
            "FourArgs".to_string(),
            vec![27],
        ),
        (
            "OverrideParams.java".to_string(),
            "OverrideParams".to_string(),
            vec![20],
        ),
    ];
    assert_eq!(
        finding_keys_in("smelly", "OverrideParams.java", "long-params"),
        expected.into_iter().collect()
    );
}

#[test]
fn an_enums_constructor_is_read_as_a_classs() {
    // Spoon asks a CtClass for constructors, and an enum is one.
    let expected = [(
        "ParamsInEnum.java".to_string(),
        "ParamsInEnum".to_string(),
        vec![12],
    )];
    assert_eq!(
        finding_keys_in("smelly", "ParamsInEnum.java", "long-params"),
        expected.into_iter().collect()
    );
}

#[test]
fn a_records_compact_constructor_takes_the_records_components() {
    // `RecordCtor` declares no parameters and is still reported: Java gives the
    // compact constructor the record's six components. `ImplicitCtor` declares
    // none either, and is not reported at all: Spoon's implicit canonical
    // constructor is not in the model with a position, so there is nothing to
    // measure. One of the two is a finding and the other is a trap.
    let expected = [(
        "RecordCtor.java".to_string(),
        "RecordCtor".to_string(),
        vec![12],
    )];
    assert_eq!(
        finding_keys_in("smelly", "RecordCtor.java", "long-params"),
        expected.into_iter().collect()
    );
}

#[test]
fn the_clean_control_stays_clean() {
    assert!(finding_keys_in("smelly", "Clean.java", "long-method").is_empty());
    assert!(finding_keys_in("smelly", "Clean.java", "long-params").is_empty());
}
