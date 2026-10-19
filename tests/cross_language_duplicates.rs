//! Structural clone candidates across languages; numeric/runtime semantics
//! still require review. Ordinary Java-only parity remains unchanged.
mod common;
use bluecheese::finding::Finding;
use bluecheese::lang::load_project;
use common::TempTree;

const JAVA: &str = "class Calculator { int adjust(int input) { int doubled = input * 2; int result = doubled + 5; return result; } }";
const RUST: &str =
    "fn adjust(value: i32) -> i32 { let scaled = value * 2; let total = scaled + 5; total }";
const JS: &str =
    "export function adjust(n) { const twice = n * 2; const result = twice + 5; return result; }";
const TS: &str = "export function adjust(amount: number): number { const scaled = amount * 2; const answer = scaled + 5; return answer; }";

fn scan(files: &[(&str, &str)]) -> Vec<Finding> {
    let tree = TempTree::new("cross-language-duplicates");
    for (path, source) in files {
        tree.write(path, source);
    }
    let project = load_project(tree.path()).expect("load source");
    bluecheese::detect(&project.files, "dup-code").expect("duplicate detector")
}

#[test]
fn finds_one_clone_group_across_four_languages_in_one_scan() {
    let findings = scan(&[
        ("Calculator.java", JAVA),
        ("calculator.rs", RUST),
        ("calculator.js", JS),
        ("calculator.ts", TS),
    ]);
    assert_eq!(findings.len(), 4, "{findings:#?}");
    for finding in &findings {
        assert!(
            finding
                .additional_info
                .contains("Cross-language structural duplicate")
        );
        assert_eq!(finding.line_numbers, [1]);
    }
}

#[test]
fn same_constant_in_a_different_rule_is_not_a_clone() {
    assert!(scan(&[("Calculator.java", JAVA), ("different.ts", "function adjust(n: number): number { const twice = n * 3; const total = twice + 5; return total; }")]).is_empty());
}

#[test]
fn preserves_binding_identity_and_operand_order() {
    let a = "function f(a, b) { const x = a - b; const y = x * a; return y; }";
    let b = "fn g(a: i32, b: i32) -> i32 { let x = b - a; let y = x * a; y }";
    assert!(scan(&[("a.js", a), ("b.rs", b)]).is_empty());
}

#[test]
fn ignores_comments_types_and_formatting_but_keeps_the_operations() {
    let ts = "function renamed(value: number) { /* different formatting */\nlet other = (value * 2);\nconst finalValue = other + 5;\nreturn finalValue; }";
    let findings = scan(&[("a.js", JS), ("b.ts", ts)]);
    assert_eq!(findings.len(), 2);
}

#[test]
fn trivial_or_unsupported_bodies_do_not_generate_clone_claims() {
    assert!(
        scan(&[
            ("a.js", "function f(n) { return n * 2; }"),
            ("b.ts", "function f(n: number) { return n * 2; }")
        ])
        .is_empty()
    );
    assert!(
        scan(&[
            (
                "a.js",
                "function f(n) { const x = unknown(n); const y = x + 5; return y; }"
            ),
            (
                "b.ts",
                "function f(n: number) { const x = unknown(n); const y = x + 5; return y; }"
            )
        ])
        .is_empty()
    );
}

#[test]
fn same_language_methods_do_not_enter_the_cross_language_comparison() {
    assert!(scan(&[("a.js", JS), ("b.js", JS)]).is_empty());
    assert!(scan(&[("a.ts", TS), ("b.tsx", TS)]).is_empty());
}

#[test]
fn changed_return_expression_is_not_a_clone() {
    let ts = "function adjust(n: number) { const twice = n * 2; const result = twice + 5; return twice; }";
    assert!(scan(&[("a.js", JS), ("b.ts", ts)]).is_empty());
}

#[test]
fn reports_partner_locations_without_comparing_source_comments() {
    let findings = scan(&[("Calculator.java", JAVA), ("calculator.ts", TS)]);
    assert_eq!(findings.len(), 2);
    let java = findings
        .iter()
        .find(|f| f.file_path == "Calculator.java")
        .expect("Java side");
    assert!(java.additional_info.contains("calculator.ts:1"));
    let ts = findings
        .iter()
        .find(|f| f.file_path == "calculator.ts")
        .expect("TS side");
    assert!(ts.additional_info.contains("Calculator.java:1"));
}

#[test]
fn cli_finds_cross_language_issues_in_the_same_mixed_directory() {
    let root = common::fixtures().join("multilingual");
    let project = load_project(&root).expect("load mixed fixture");
    assert_eq!(project.files.len(), 5);
    assert!(project.files.iter().all(|file| !file.root().has_error()));
    let run = |smell| {
        let output = std::process::Command::new(env!("CARGO_BIN_EXE_bluecheese"))
            .args(["-f", root.to_str().unwrap(), "-s", smell])
            .output()
            .expect("run CLI");
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        serde_json::from_slice::<Vec<Finding>>(&output.stdout).expect("JSON findings")
    };
    let duplicates = run("dup-code");
    assert_eq!(duplicates.len(), 4);
    assert!(
        duplicates
            .iter()
            .all(|finding| finding.additional_info.contains("Cross-language"))
    );
    let coupling = run("shotgun");
    assert_eq!(coupling.len(), 1);
    assert_eq!(coupling[0].file_path, "policy.ts");
    assert_eq!(
        coupling[0].additional_info,
        "Shotgun Surgery [CM=2, CC=2, Fan-Out=0]"
    );
}

#[test]
fn shared_metrics_find_parameter_lists_in_four_languages_together() {
    let tree = TempTree::new("mixed-parameter-lists");
    tree.write(
        "Example.java",
        "class Example { void save(int a, int b, int c, int d) {} }",
    )
    .write("example.rs", "fn save(a: i32, b: i32, c: i32, d: i32) {}")
    .write("example.js", "function save(a, b, c, d) {}")
    .write(
        "example.ts",
        "function save(a: number, b: number, c: number, d: number) {}",
    );
    let project = load_project(tree.path()).expect("load mixed source");
    let findings = bluecheese::detect(&project.files, "long-params").expect("shared metric");
    let paths: std::collections::BTreeSet<_> = findings
        .iter()
        .map(|finding| finding.file_path.as_str())
        .collect();
    assert_eq!(
        paths,
        ["Example.java", "example.rs", "example.js", "example.ts"]
            .into_iter()
            .collect()
    );
}
