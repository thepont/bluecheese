mod common;

use bluecheese::finding::Finding;
use common::parse;

fn scan(name: &str, source: &str) -> Vec<Finding> {
    let file = parse(name, source);
    bluecheese::detect(&[file], "large-class").expect("run Large Class")
}

#[test]
fn rust_types_with_many_fields_and_methods_are_reported() {
    let findings = scan(
        "record.rs",
        "struct Record { a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32 }\nimpl Record { fn total(&self) -> i32 { self.a + self.b } }",
    );
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].class_name, "Record");
    assert_eq!(findings[0].line_numbers, [1]);
}

#[test]
fn javascript_and_typescript_types_with_many_fields_and_methods_are_reported() {
    for (name, source) in [
        ("record.js", "class Record { a; b; c; d; e; f; g; h; total() { return this.a + this.b; } }"),
        ("record.ts", "class Record { a: number; b: number; c: number; d: number; e: number; f: number; g: number; h: number; total() { return this.a + this.b; } }"),
    ] {
        let findings = scan(name, source);
        assert_eq!(findings.len(), 1, "{name}");
        assert_eq!(findings[0].class_name, "Record", "{name}");
        assert_eq!(findings[0].line_numbers, [1], "{name}");
    }
}
