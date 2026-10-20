mod common;

use bluecheese::finding::Finding;
use common::parse;

fn scan(name: &str, source: &str) -> Vec<Finding> {
    let file = parse(name, source);
    bluecheese::detect(&[file], "shotgun").expect("run Shotgun Surgery")
}

#[test]
fn rust_methods_called_from_two_other_types_are_reported() {
    let findings = scan(
        "calls.rs",
        "struct Shared;\nimpl Shared { fn work() {} }\nstruct First;\nimpl First { fn call() { Shared::work(); } }\nstruct Second;\nimpl Second { fn call() { Shared::work(); } }\n",
    );
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].class_name, "Shared");
    assert_eq!(findings[0].line_numbers, [2]);
}

#[test]
fn unknown_rust_receivers_are_not_joined_to_a_unique_method_name() {
    let findings = scan(
        "calls.rs",
        "struct Shared; impl Shared { fn work(&self) {} }
         struct First; impl First { fn call(value: Missing) { value.work(); } }
         struct Second; impl Second { fn call(value: Missing) { value.work(); } }",
    );
    assert!(findings.is_empty());
}

#[test]
fn unqualified_rust_calls_are_not_joined_to_associated_functions() {
    let findings = scan(
        "calls.rs",
        "struct Shared; impl Shared { fn work() {} }
         struct First; impl First { fn call() { work(); } }
         struct Second; impl Second { fn call() { work(); } }",
    );
    assert!(findings.is_empty());
}

#[test]
fn typed_rust_receivers_resolve_to_their_declared_type() {
    let findings = scan(
        "calls.rs",
        "struct Shared; impl Shared { fn work(&self) {} }
         struct Other; impl Other { fn work(&self) {} }
         struct First; impl First { fn call(value: &Shared) { value.work(); } }
         struct Second; impl Second { fn call(value: &Shared) { value.work(); } }",
    );
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].class_name, "Shared");
}

#[test]
fn rust_free_functions_still_count_resolved_calls_from_other_types() {
    let findings = scan(
        "calls.rs",
        "fn work() {}
         struct First; impl First { fn call() { work(); } }
         struct Second; impl Second { fn call() { work(); } }",
    );
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].class_name, "calls");
}

#[test]
fn a_rust_local_binding_shadows_a_typed_parameter() {
    for binding in ["let value = missing();", "let mut value = missing();"] {
        let findings = scan(
            "calls.rs",
            &format!(
                "struct Shared; impl Shared {{ fn work(&self) {{}} }}
                 struct First; impl First {{ fn call(value: &Shared) {{ {binding} value.work(); }} }}
                 struct Second; impl Second {{ fn call(value: &Shared) {{ value.work(); }} }}"
            ),
        );
        assert!(findings.is_empty(), "{binding}");
    }
}

#[test]
fn typed_rust_locals_resolve_without_leaking_between_blocks() {
    for statements in [
        "let value: &Shared = missing(); value.work();",
        "{ let value: Missing = missing(); } value.work();",
    ] {
        let findings = scan(
            "calls.rs",
            &format!(
                "struct Shared; impl Shared {{ fn work(&self) {{}} }}
                 struct First; impl First {{ fn call(value: &Shared) {{ {statements} }} }}
                 struct Second; impl Second {{ fn call(value: &Shared) {{ value.work(); }} }}"
            ),
        );
        assert_eq!(findings.len(), 1, "{statements}");
        assert_eq!(findings[0].class_name, "Shared");
    }
}
