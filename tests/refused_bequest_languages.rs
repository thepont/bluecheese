mod common;

use bluecheese::finding::Finding;
use common::parse;

fn scan(name: &str, source: &str) -> Vec<Finding> {
    let file = parse(name, source);
    bluecheese::detect(&[file], "refused-bequest").expect("run Refused Bequest")
}

#[test]
fn javascript_and_typescript_empty_overrides_are_reported() {
    for (name, source) in [
        (
            "refusal.js",
            "class Base { open() { return 1; } } class Child extends Base { open() {} }",
        ),
        (
            "refusal.ts",
            "class Base { open(): number { return 1; } } class Child extends Base { open(): number {} }",
        ),
    ] {
        let findings = scan(name, source);
        assert_eq!(findings.len(), 1, "{name}");
        assert_eq!(findings[0].class_name, "Child", "{name}");
        assert_eq!(findings[0].line_numbers, [1], "{name}");
    }
}

#[test]
fn rust_trait_methods_explicitly_left_unimplemented_are_reported() {
    let findings = scan(
        "refusal.rs",
        "trait Base { fn open(&self) -> i32 { 1 } }\nstruct Child;\nimpl Base for Child { fn open(&self) -> i32 { unimplemented!() } }\n",
    );
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].class_name, "Child");
    assert_eq!(findings[0].line_numbers, [3]);
}
