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
