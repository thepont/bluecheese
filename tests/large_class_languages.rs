mod common;

use bluecheese::finding::Finding;
use common::parse;

fn complex_class(name: &str, second_receiver: &str, second_field: &str) -> String {
    if name.ends_with(".rs") {
        let first = "if flag { self.left += 1; }".repeat(24);
        let second = format!("if flag {{ {second_receiver}.{second_field} += 1; }}").repeat(24);
        format!(
            "struct Service {{ left: i32, right: i32 }} impl Service {{ fn first(&mut self, flag: bool) {{ {first} }} fn second(&mut self, flag: bool, other: &mut Service) {{ let mut left = 0; {second} }} }}"
        )
    } else {
        let first = "if (flag) { this.left += 1; }".repeat(24);
        let second = format!("if (flag) {{ {second_receiver}.{second_field} += 1; }}").repeat(24);
        format!(
            "class Service {{ left = 0; right = 0; first(flag) {{ {first} }} second(flag, other) {{ let left = 0; {second} }} }}"
        )
    }
}

#[test]
fn complex_methods_using_the_same_instance_field_are_cohesive() {
    for (name, receiver) in [
        ("service.rs", "self"),
        ("service.js", "this"),
        ("service.ts", "this"),
    ] {
        let findings = scan(name, &complex_class(name, receiver, "left"));
        assert!(findings.is_empty(), "{name}: {findings:#?}");
    }
}

#[test]
fn complex_methods_using_separate_instance_fields_are_not_cohesive() {
    for (name, receiver) in [
        ("service.rs", "self"),
        ("service.js", "this"),
        ("service.ts", "this"),
    ] {
        let findings = scan(name, &complex_class(name, receiver, "right"));
        assert_eq!(findings.len(), 1, "{name}: {findings:#?}");
        assert_eq!(findings[0].class_name, "Service", "{name}");
    }
}

#[test]
fn accessing_another_instances_field_does_not_connect_own_methods() {
    for name in ["service.rs", "service.js", "service.ts"] {
        let findings = scan(name, &complex_class(name, "other", "left"));
        assert_eq!(findings.len(), 1, "{name}: {findings:#?}");
    }
}

#[test]
fn local_variables_named_like_fields_do_not_connect_methods() {
    for name in ["service.rs", "service.js", "service.ts"] {
        let source = complex_class(name, "other", "left").replace("other.left", "left");
        let findings = scan(name, &source);
        assert_eq!(findings.len(), 1, "{name}: {findings:#?}");
    }
}

#[test]
fn private_ecmascript_fields_contribute_to_cohesion() {
    for name in ["service.js", "service.ts", "service.tsx"] {
        let source = complex_class(name, "this", "left")
            .replace("class Service { left", "class Service { #left")
            .replace("this.left", "this.#left");
        let findings = scan(name, &source);
        assert!(findings.is_empty(), "{name}: {findings:#?}");
    }
}

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
        (
            "record.js",
            "class Record { a; b; c; d; e; f; g; h; total() { return this.a + this.b; } }",
        ),
        (
            "record.ts",
            "class Record { a: number; b: number; c: number; d: number; e: number; f: number; g: number; h: number; total() { return this.a + this.b; } }",
        ),
    ] {
        let findings = scan(name, source);
        assert_eq!(findings.len(), 1, "{name}");
        assert_eq!(findings[0].class_name, "Record", "{name}");
        assert_eq!(findings[0].line_numbers, [1], "{name}");
    }
}
