mod common;

use bluecheese::finding::Finding;
use common::parse;

fn scan(name: &str, source: &str) -> Vec<Finding> {
    let file = parse(name, source);
    bluecheese::detect(&[file], "data-class").expect("run Data Class")
}

#[test]
fn rust_public_data_structs_are_reported() {
    let findings = scan("point.rs", "pub struct Point { pub x: f64, pub y: f64 }\n");
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].class_name, "Point");
    assert_eq!(findings[0].line_numbers, [1]);
}

#[test]
fn javascript_and_typescript_public_data_classes_are_reported() {
    for (name, source) in [
        ("point.js", "class Point { x; y; }"),
        ("point.ts", "class Point { x: number; y: number; }"),
    ] {
        let findings = scan(name, source);
        assert_eq!(findings.len(), 1, "{name}");
        assert_eq!(findings[0].class_name, "Point", "{name}");
        assert_eq!(findings[0].line_numbers, [1], "{name}");
    }
}

#[test]
fn accessor_only_data_structs_and_classes_are_reported() {
    for (name, source) in [
        (
            "point.rs",
            "pub struct Point { pub x: f64, pub y: f64 }\nimpl Point { fn get_x(&self) -> f64 { self.x } fn get_y(&self) -> f64 { self.y } }",
        ),
        (
            "point.js",
            "class Point { x; y; getX() { return this.x; } getY() { return this.y; } }",
        ),
        (
            "point.ts",
            "class Point { x: number; y: number; getX(): number { return this.x; } getY(): number { return this.y; } }",
        ),
    ] {
        let findings = scan(name, source);
        assert_eq!(findings.len(), 1, "{name}");
        assert_eq!(findings[0].class_name, "Point", "{name}");
    }
}

#[test]
fn accessor_only_setters_are_checked_against_a_single_field_assignment() {
    for (name, source) in [
        (
            "point.rs",
            "pub struct Point { pub x: f64 }\nimpl Point { fn set_x(&mut self, value: f64) { self.x = value; } }",
        ),
        (
            "point.js",
            "class Point { x; setX(value) { this.x = value; } }",
        ),
        (
            "point.ts",
            "class Point { x: number; setX(value: number) { this.x = value; } }",
        ),
    ] {
        let findings = scan(name, source);
        assert_eq!(findings.len(), 1, "{name}");
        assert_eq!(findings[0].class_name, "Point", "{name}");
    }
}

#[test]
fn private_and_static_fields_are_not_public_instance_data() {
    for (name, source) in [
        ("point.js", "class Point { #x = 1; }"),
        ("point.ts", "class Point { #x = 1; }"),
        ("point.ts", "class Point { private x = 1; }"),
        ("point.ts", "class Point { protected x = 1; }"),
        ("point.js", "class Point { static x = 1; }"),
        ("point.ts", "class Point { static x = 1; }"),
    ] {
        assert!(scan(name, source).is_empty(), "{name}: {source}");
    }
}

#[test]
fn field_names_and_initializers_are_not_modifiers() {
    for name in ["point.js", "point.ts"] {
        for source in [
            "class Point { x = 'private'; }",
            "class Point { x = 'protected'; }",
            "class Point { x = 'static'; }",
            "class Point { static = 1; }",
            "class Point { private = 1; }",
        ] {
            assert_eq!(scan(name, source).len(), 1, "{name}: {source}");
        }
    }
}
