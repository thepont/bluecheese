//! Characterisation of StaticNose's static coupling heuristic, not a semantic
//! oracle for whether a shared responsibility is misplaced.
mod common;

use bluecheese::finding::Finding;
use bluecheese::lang::load_project;
use common::{TempTree, parse};

fn scan(source: &str) -> Vec<Finding> {
    bluecheese::detect(&[parse("Example.java", source)], "shotgun").expect("shotgun is ported")
}

#[test]
fn two_distinct_external_classes_trigger_even_for_a_centralised_policy() {
    let findings = scan(
        "class Policy { void check() {} }\nclass A { void run(Policy p) { p.check(); p.check(); } }\nclass B { void run(Policy p) { p.check(); } }",
    );
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].class_name, "Policy");
    assert_eq!(findings[0].line_numbers, [1]);
    assert_eq!(
        findings[0].additional_info,
        "Shotgun Surgery [CM=2, CC=2, Fan-Out=0]"
    );
}

#[test]
fn duplicated_policy_without_a_shared_callee_is_invisible_to_static_coupling() {
    // Changing this policy needs two edits, but there is no incoming call
    // relationship for the heuristic to measure.
    assert!(scan("class A { boolean allowed(int balance) { return balance > 500; } } class B { boolean allowed(int balance) { return balance > 500; } }").is_empty());
}

#[test]
fn unresolved_receivers_and_ambiguous_overloads_are_not_joined_by_name() {
    assert!(scan("class Policy { void check(String x) {} void check(Helper x) {} } class Helper {} class A { void run(Policy p) { p.check(null); } } class B { void run(Policy p) { p.check(null); } } class C { void run(Missing p) { p.check(\"x\"); } }").is_empty());
}

#[test]
fn one_calling_class_and_self_calls_do_not_meet_the_threshold() {
    assert!(scan("class Policy { void check() {} void a() { check(); } void b() { check(); } } class A { void a(Policy p) { p.check(); } void b(Policy p) { p.check(); } }").is_empty());
}

#[test]
fn counts_distinct_calling_methods_and_distinct_outgoing_targets() {
    let findings = scan(
        "class Policy { void check(Helper h) { h.go(); h.go(); } } class Helper { void go() {} } class A { void a(Policy p, Helper h) { p.check(h); } void b(Policy p, Helper h) { p.check(h); } } class B { void run(Policy p, Helper h) { p.check(h); } }",
    );
    let policy = findings
        .iter()
        .find(|f| f.class_name == "Policy")
        .expect("shared policy");
    assert_eq!(
        policy.additional_info,
        "Shotgun Surgery [CM=3, CC=2, Fan-Out=1]"
    );
}

#[test]
fn constructor_calls_do_not_count_as_calling_methods() {
    assert!(scan("class Policy { void check() {} } class A { A(Policy p) { p.check(); } } class B { B(Policy p) { p.check(); } }").is_empty());
}

#[test]
fn overloads_are_not_combined_by_name_or_arity() {
    assert!(scan("class Policy { void check(int x) {} void check(String x) {} } class A { void run(Policy p) { p.check(1); } } class B { void run(Policy p) { p.check(\"x\"); } }").is_empty());
}

#[test]
fn interface_contracts_are_excluded_including_inherited_contracts() {
    assert!(scan("interface Root { void check(); } interface Child extends Root {} class Policy implements Child { public void check() {} } class A { void run(Policy p) { p.check(); } } class B { void run(Policy p) { p.check(); } }").is_empty());
}

#[test]
fn nested_class_methods_and_field_receivers_are_read() {
    let findings = scan(
        "class Outer { static class Policy { void check() {} } Policy p; void run() { this.p.check(); } } class B { Outer.Policy p; void run() { p.check(); } }",
    );
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].class_name, "Policy");
}

#[test]
fn qualified_types_keep_same_named_classes_in_different_packages_separate() {
    let tree = TempTree::new("shotgun-packages");
    tree.write(
        "one/Policy.java",
        "package one; public class Policy { public void check() {} }",
    );
    tree.write(
        "two/Policy.java",
        "package two; public class Policy { public void check() {} }",
    );
    tree.write(
        "A.java",
        "import one.Policy; class A { void run(Policy p) { p.check(); } }",
    );
    tree.write(
        "B.java",
        "import two.Policy; class B { void run(Policy p) { p.check(); } }",
    );
    let project = load_project(tree.path()).expect("load sources");
    assert!(
        bluecheese::detect(&project.files, "shotgun")
            .expect("ported")
            .is_empty()
    );
}

#[test]
fn fully_qualified_static_receivers_resolve_to_the_declared_method() {
    let findings = scan(
        "package example; class Policy { static void check() {} } class A { void run() { example.Policy.check(); } } class B { void run() { example.Policy.check(); } }",
    );
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].class_name, "Policy");
}

#[test]
fn block_local_receiver_does_not_leak_into_another_scope() {
    let findings = scan(
        "class Policy { void check() {} } class Other { void check() {} } class A { Policy p; void run() { { Other p = new Other(); p.check(); } p.check(); } } class B { void run(Policy p) { p.check(); } }",
    );
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].class_name, "Policy");
}

#[test]
fn shotgun_is_available_at_the_command_line() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_bluecheese"))
        .args([
            "-f",
            common::fixtures().join("smellbench").to_str().unwrap(),
            "-s",
            "shotgun",
            "--reference",
        ])
        .output()
        .expect("run CLI");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let findings: Vec<Finding> = serde_json::from_slice(&output.stdout).expect("JSON findings");
    assert!(
        findings
            .iter()
            .any(|f| f.class_name == "BankAccount" && f.line_numbers == [38])
    );
}

#[test]
fn all_reference_metrics_match_on_smellbench() {
    use std::collections::BTreeMap;
    let project = load_project(&common::fixtures().join("smellbench")).expect("load corpus");
    let actual: BTreeMap<_, _> = bluecheese::detect(&project.files, "shotgun")
        .expect("ported")
        .into_iter()
        .map(|finding| (finding.key(), finding.additional_info))
        .collect();
    let golden: Vec<Finding> = serde_json::from_slice(
        &std::fs::read(common::fixtures().join("golden/smellbench.json")).expect("golden exists"),
    )
    .expect("JSON");
    let expected: BTreeMap<_, _> = golden
        .into_iter()
        .filter(|finding| finding.smell_name == "Shotgun Surgery")
        .map(|finding| (finding.key(), finding.additional_info))
        .collect();
    assert_eq!(actual, expected);
}

#[test]
fn cyclic_inferred_receivers_do_not_overflow_the_stack() {
    assert!(scan("class A { void run() { var p = p; p.check(); } } class B { void run() { var p = p; p.check(); } }").is_empty());
}
