mod common;

use bluecheese::finding::Finding;
use common::parse;

fn scan(name: &str, source: &str) -> Vec<Finding> {
    let file = parse(name, source);
    bluecheese::detect(&[file], "prim-obsession").expect("run Primitive Obsession")
}

#[test]
fn reports_types_with_many_primitive_fields_in_java_rust_and_typescript() {
    for (name, source, expected) in [
        (
            "record.java",
            "class HealthRecord { int age; double height; double weight; boolean active; }",
            "HealthRecord",
        ),
        (
            "record.rs",
            "struct HealthRecord { age: u32, height: f64, weight: f64, active: bool }",
            "HealthRecord",
        ),
        (
            "record.ts",
            "class HealthRecord { age: number; height: number; weight: number; active: boolean; }",
            "HealthRecord",
        ),
    ] {
        let findings = scan(name, source);
        assert_eq!(findings.len(), 1, "{name}");
        assert_eq!(findings[0].class_name, expected, "{name}");
    }
}

#[test]
fn ignores_small_primitive_sets_and_custom_value_types() {
    for (name, source) in [
        ("point.java", "class Point { int x; int y; }"),
        (
            "person.rs",
            "struct Person { address: Address, account: Account, id: Identity }",
        ),
        (
            "person.ts",
            "class Person { address: Address; account: Account; id: Identity; }",
        ),
    ] {
        assert!(scan(name, source).is_empty(), "{name}");
    }
}
