mod common;

use bluecheese::finding::Finding;
use common::parse;

fn scan(name: &str, source: &str) -> Vec<Finding> {
    let file = parse(name, source);
    bluecheese::detect(&[file], "middle-man").expect("run Middle Man")
}

#[test]
fn rust_types_that_only_forward_to_a_project_field_are_reported() {
    let findings = scan(
        "service.rs",
        "struct Target;\nimpl Target { fn first(&self) {} fn second(&self) {} }\nstruct Service { target: Target }\nimpl Service { fn first(&self) { self.target.first(); } fn second(&self) { self.target.second(); } }\n",
    );
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].class_name, "Service");
    assert_eq!(findings[0].line_numbers, [3]);
}

#[test]
fn typescript_and_javascript_wrappers_are_reported() {
    for (name, source) in [
        (
            "service.ts",
            "class Target { first() {} second() {} } class Service { target: Target; first() { return this.target.first(); } second() { return this.target.second(); } }",
        ),
        (
            "service.js",
            "class Target { first() {} second() {} } class Service { constructor() { this.target = new Target(); } first() { return this.target.first(); } second() { return this.target.second(); } }",
        ),
    ] {
        let findings = scan(name, source);
        assert_eq!(findings.len(), 1, "{name}");
        assert_eq!(findings[0].class_name, "Service", "{name}");
    }
}
