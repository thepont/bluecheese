mod common;

use bluecheese::finding::Finding;
use common::parse;

fn scan(name: &str, source: &str) -> Vec<Finding> {
    let file = parse(name, source);
    bluecheese::detect(&[file], "switch-stmts").expect("run Switch Statements")
}

#[test]
fn javascript_and_typescript_switch_statements_are_reported() {
    for (name, source) in [
        ("choice.js", "class Choice { pick(value) { switch (value) { case 1: return 'one'; default: return 'other'; } } }"),
        ("choice.ts", "class Choice { pick(value: number) { switch (value) { case 1: return 'one'; default: return 'other'; } } }"),
    ] {
        let findings = scan(name, source);
        assert_eq!(findings.len(), 1, "{name}");
        assert_eq!(findings[0].class_name, "Choice", "{name}");
        assert_eq!(findings[0].line_numbers, [1], "{name}");
    }
}
