mod common;

use bluecheese::finding::Finding;
use common::parse;

fn scan(name: &str, source: &str) -> Vec<Finding> {
    let file = parse(name, source);
    bluecheese::detect(&[file], "dup-code").expect("run Duplicate Code")
}

#[test]
fn rust_impl_methods_with_repeated_bodies_are_reported() {
    let findings = scan(
        "duplicate.rs",
        "struct Unit;\nimpl Unit {\n    fn first(&self, x: i32) -> i32 { let a = x + 1; let b = a * 2; b }\n    fn second(&self, x: i32) -> i32 { let a = x + 1; let b = a * 2; b }\n}\n",
    );
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].class_name, "Unit");
    assert_eq!(findings[0].line_numbers, [3, 4]);
}

#[test]
fn javascript_and_typescript_methods_with_repeated_bodies_are_reported() {
    for (name, source) in [
        ("duplicate.js", "class Unit { first(x) { const a = x + 1; const b = a * 2; return b; } second(x) { const a = x + 1; const b = a * 2; return b; } }"),
        ("duplicate.ts", "class Unit { first(x: number): number { const a = x + 1; const b = a * 2; return b; } second(x: number): number { const a = x + 1; const b = a * 2; return b; } }"),
    ] {
        let findings = scan(name, source);
        assert_eq!(findings.len(), 1, "{name}");
        assert_eq!(findings[0].class_name, "Unit", "{name}");
        assert_eq!(findings[0].line_numbers, [1], "{name}");
    }
}
