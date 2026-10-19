mod common;

use bluecheese::finding::Finding;
use common::parse;

fn scan(name: &str, source: &str) -> Vec<Finding> {
    let file = parse(name, source);
    bluecheese::detect(&[file], "data-clumps").expect("run Data Clumps")
}

#[test]
fn rust_type_methods_with_repeated_parameter_groups_are_reported() {
    let findings = scan(
        "batch.rs",
        "struct Batch;\nimpl Batch {\n    fn create(&self, start: i32, end: i32) {}\n    fn update(&self, start: i32, end: i32) {}\n}\n",
    );
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].class_name, "Batch");
    assert_eq!(findings[0].line_numbers, [3, 4]);
}

#[test]
fn javascript_and_typescript_type_methods_with_repeated_groups_are_reported() {
    for (name, source) in [
        ("batch.js", "class Batch { create(start, end) {} update(start, end) {} }"),
        ("batch.ts", "class Batch { create(start: number, end: number) {} update(start: number, end: number) {} }"),
    ] {
        let findings = scan(name, source);
        assert_eq!(findings.len(), 1, "{name}");
        assert_eq!(findings[0].class_name, "Batch", "{name}");
        assert_eq!(findings[0].line_numbers, [1], "{name}");
    }
}
