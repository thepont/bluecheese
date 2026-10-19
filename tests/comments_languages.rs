mod common;

use bluecheese::finding::Finding;
use common::parse;

fn scan(name: &str, source: &str) -> Vec<Finding> {
    let file = parse(name, source);
    bluecheese::detect(&[file], "comments").expect("run Comments")
}

#[test]
fn rust_reports_explanatory_module_and_type_comments_but_skips_doc_comments() {
    let findings = scan(
        "sample.rs",
        "/// This documents the public holder type for library users.\nstruct Holder {\n    // This explanation records why this value is retained.\n    value: i32,\n}\nfn run() {\n    // TODO\n    let _value = 1;\n}\n",
    );
    assert_eq!(findings.len(), 2);
    assert!(
        findings
            .iter()
            .any(|f| f.class_name == "Holder" && f.line_numbers == [3])
    );
    assert!(
        findings
            .iter()
            .any(|f| f.class_name == "sample" && f.line_numbers == [7])
    );
}

#[test]
fn javascript_and_typescript_report_explanatory_comments() {
    for (name, source) in [
        (
            "sample.js",
            "class Service { run() {\n  // This explains why the retry stays local.\n  return true;\n} }",
        ),
        (
            "sample.ts",
            "class Service { run(): boolean {\n  // This explains why the retry stays local.\n  return true;\n} }",
        ),
    ] {
        let findings = scan(name, source);
        assert_eq!(findings.len(), 1, "{name}");
        assert_eq!(findings[0].class_name, "Service", "{name}");
        assert_eq!(findings[0].line_numbers, [2], "{name}");
    }
}

#[test]
fn rust_documentation_is_not_an_explanatory_code_comment() {
    for documentation in [
        "//! This documents the module contract for library users.\n",
        "/*! This documents the module contract for library users. */\n",
        "/// This documents a field contract for library users.\n",
        "/** This documents a field contract for library users. */\n",
    ] {
        let source = if documentation.starts_with("//!") || documentation.starts_with("/*!") {
            format!("{documentation}pub const VALUE: i32 = 1;\n")
        } else {
            format!("struct Holder {{\n{documentation}value: i32,\n}}\n")
        };
        assert!(scan("sample.rs", &source).is_empty(), "{documentation}");
    }
}

#[test]
fn rust_ordinary_comments_that_resemble_doc_markers_still_report() {
    for comment in [
        "//// This explains why this value is retained.",
        "/*** This explains why this value is retained. */",
    ] {
        let source = format!("fn run() {{\n{comment}\nlet value = 1;\n}}\n");
        let findings = scan("sample.rs", &source);
        assert_eq!(findings.len(), 1, "{comment}");
        assert_eq!(findings[0].line_numbers, [2], "{comment}");
    }
}

#[test]
fn trailing_explanations_are_reported_even_before_a_declaration() {
    for (name, source) in [
        (
            "sample.rs",
            "const VALUE: i32 = 1; // This explains why this value is retained.\nfn run() {}\n",
        ),
        (
            "sample.js",
            "const value = 1; // This explains why this value is retained.\nfunction run() {}\n",
        ),
        (
            "sample.ts",
            "const value: number = 1; // This explains why this value is retained.\nfunction run() {}\n",
        ),
    ] {
        let findings = scan(name, source);
        assert_eq!(findings.len(), 1, "{name}");
        assert_eq!(findings[0].line_numbers, [1], "{name}");
    }
}
