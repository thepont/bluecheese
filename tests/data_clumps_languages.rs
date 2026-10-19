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
        (
            "batch.js",
            "class Batch { create(start, end) {} update(start, end) {} }",
        ),
        (
            "batch.ts",
            "class Batch { create(start: number, end: number) {} update(start: number, end: number) {} }",
        ),
    ] {
        let findings = scan(name, source);
        assert_eq!(findings.len(), 1, "{name}");
        assert_eq!(findings[0].class_name, "Batch", "{name}");
        assert_eq!(findings[0].line_numbers, [1], "{name}");
    }
}

#[test]
fn identically_named_parameters_with_different_types_are_not_a_shared_group() {
    for (name, source) in [
        (
            "values.rs",
            "struct Values; impl Values { fn add(left: usize, right: usize) {} fn compare(left: &str, right: &str) {} }",
        ),
        (
            "Values.java",
            "class Values { void add(int left, int right) {} void compare(String left, String right) {} }",
        ),
        (
            "values.ts",
            "class Values { add(left: number, right: number) {} compare(left: string, right: string) {} }",
        ),
    ] {
        let findings = scan(name, source);
        assert!(findings.is_empty(), "{name}: {findings:#?}");
    }
}

#[test]
fn one_compatible_parameter_does_not_form_a_pair() {
    let findings = scan(
        "values.rs",
        "struct Values; impl Values { fn add(left: usize, right: usize) {} fn compare(left: usize, right: &str) {} }",
    );
    assert!(findings.is_empty(), "{findings:#?}");
}

#[test]
fn lifetimes_do_not_split_an_otherwise_identical_parameter_group() {
    let findings = scan(
        "values.rs",
        "struct Values; impl Values { fn add<'a>(left: &'a str, right: &'a str) {} fn compare(left: &str, right: &str) {} }",
    );
    assert_eq!(findings.len(), 1, "{findings:#?}");
}

#[test]
fn missing_annotations_do_not_disprove_a_shared_parameter_group() {
    let findings = scan(
        "values.ts",
        "class Values { add(left: number, right: number) {} compare(left, right) {} }",
    );
    assert_eq!(findings.len(), 1, "{findings:#?}");
}
