mod common;

use std::process::Command;

use bluecheese::finding::Finding;
use bluecheese::lang::load_project;
use common::TempTree;

fn tree(sources: &[(&str, &str)]) -> TempTree {
    let tree = TempTree::new("shotgun-declarations");
    for (path, source) in sources {
        tree.write(path, source);
    }
    tree
}

fn scan(sources: &[(&str, &str)]) -> Vec<Finding> {
    let tree = tree(sources);
    let output = Command::new(env!("CARGO_BIN_EXE_bluecheese"))
        .arg("-f")
        .arg(tree.path())
        .args(["-s", "shotgun", "--source-only"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

#[test]
fn parallel_enums_are_found_without_calls_in_java_rust_and_typescript() {
    for (extension, source) in [
        ("java", "enum Tenant { ACME, GLOBEX }"),
        ("rs", "enum Tenant { ACME, GLOBEX }"),
        ("ts", "export enum Tenant { ACME, GLOBEX }"),
    ] {
        let first = format!("first/Tenant.{extension}");
        let second = format!("second/Tenant.{extension}");
        let findings = scan(&[(&first, source), (&second, source)]);
        assert_eq!(findings.len(), 2, "{extension}");
        assert!(
            findings
                .iter()
                .all(|finding| finding.additional_info.contains(&first)
                    && finding.additional_info.contains(&second)),
            "{extension}"
        );
    }
}

#[test]
fn discordjs_runtime_catalogue_and_numeric_enum_are_duplicate_declarations() {
    let findings = scan(&[
        (
            "Constants.js",
            include_str!("fixtures/shotgun-discordjs/Constants.js"),
        ),
        (
            "enums.d.ts",
            include_str!("fixtures/shotgun-discordjs/enums.d.ts"),
        ),
    ]);
    assert_eq!(findings.len(), 2);
    assert!(
        findings
            .iter()
            .all(|finding| finding.class_name == "MessageTypes")
    );
    for finding in findings {
        assert!(finding.additional_info.contains("Constants.js"));
        assert!(finding.additional_info.contains("enums.d.ts"));
        assert!(
            finding
                .additional_info
                .contains("ROLE_SUBSCRIPTION_PURCHASE")
        );
    }
}

#[test]
fn numeric_catalogue_matches_preserve_reserved_slots_and_values() {
    let enumeration = "export const enum MessageTypes { DEFAULT, ADDED = 2, REMOVED }";
    assert_eq!(
        scan(&[
            ("types.ts", enumeration),
            (
                "runtime.js",
                "module.exports.MessageTypes = ['DEFAULT', null, 'ADDED', 'REMOVED'];"
            ),
        ])
        .len(),
        2
    );
    for runtime in [
        "exports.MessageTypes = ['DEFAULT', 'ADDED', 'REMOVED'];",
        "exports.MessageTypes = ['DEFAULT', null, 'REMOVED', 'ADDED'];",
        "exports.MessageTypes = [null, 'DEFAULT', 'ADDED', 'REMOVED'];",
        "exports.MessageTypes = ['DEFAULT', , 'ADDED', 'REMOVED'];",
        "exports.MessageTypes = ['DEFAULT', ...configured, 'ADDED', 'REMOVED'];",
        "exports.MessageTypes = ['DEFAULT', null, 'ADDED', 'ADDED', 'REMOVED'];",
    ] {
        assert!(
            scan(&[("types.ts", enumeration), ("runtime.js", runtime)]).is_empty(),
            "{runtime}"
        );
    }
    assert!(
        scan(&[
            (
                "first.ts",
                "enum MessageTypes { DEFAULT, ADDED = 2, REMOVED }"
            ),
            ("second.ts", "enum MessageTypes { DEFAULT, ADDED, REMOVED }"),
        ])
        .is_empty()
    );
}

#[test]
fn numeric_enum_values_are_compared_without_executing_expressions() {
    assert_eq!(scan(&[
        ("first.ts", "enum Codes { NEGATIVE = -1, ZERO, TWO = 0x2, FOUR = 0b100, EIGHT = 0o10, MANY = 1_000 }"),
        ("second.ts", "enum Codes { NEGATIVE = -1, ZERO = 0, TWO = 2, FOUR = 4, EIGHT = 8, MANY = 1000 }"),
    ]).len(), 2);
    for first in [
        "enum Codes { A = choose(), B }",
        "enum Codes { A = 9007199254740992, B }",
        "enum Codes { A = 9007199254740991, B }",
    ] {
        assert!(
            scan(&[("first.ts", first), ("second.ts", first)]).is_empty(),
            "{first}"
        );
    }
}

#[test]
fn instance_assignments_and_shadowed_commonjs_names_are_not_module_catalogues() {
    for runtime in [
        "const store = {}; store.MessageTypes = ['DEFAULT', 'ADDED'];",
        "function configure(exports) { exports.MessageTypes = ['DEFAULT', 'ADDED']; }",
        "const exports = {}; exports.MessageTypes = ['DEFAULT', 'ADDED'];",
        "const module = {exports: {}}; module.exports.MessageTypes = ['DEFAULT', 'ADDED'];",
        "function exports() {} exports.MessageTypes = ['DEFAULT', 'ADDED'];",
        "import exports from './other'; exports.MessageTypes = ['DEFAULT', 'ADDED'];",
        "import { catalogue as exports } from './other'; exports.MessageTypes = ['DEFAULT', 'ADDED'];",
        "import * as module from './other'; module.exports.MessageTypes = ['DEFAULT', 'ADDED'];",
        "const { exports } = other; exports.MessageTypes = ['DEFAULT', 'ADDED'];",
    ] {
        assert!(
            scan(&[
                ("types.ts", "enum MessageTypes { DEFAULT, ADDED }"),
                ("runtime.js", runtime),
            ])
            .is_empty(),
            "{runtime}"
        );
    }
}

#[test]
fn a_tenant_enum_and_independent_name_lists_are_linked_across_languages() {
    let findings = scan(&[
        ("tenant.rs", "enum Tenant { ACME, GLOBEX }"),
        (
            "registration.ts",
            "export const TENANTS = ['ACME', 'GLOBEX'] as const;",
        ),
        ("routing.js", "export const tenants = ['GLOBEX', 'ACME'];"),
    ]);
    assert_eq!(findings.len(), 3);
    for finding in findings {
        assert!(finding.additional_info.contains("Source evidence"));
        assert!(finding.additional_info.contains("tenant.rs:1"));
        assert!(finding.additional_info.contains("registration.ts:1"));
        assert!(finding.additional_info.contains("routing.js:1"));
    }
}

#[test]
fn normal_application_reports_source_evidence_without_a_git_repository() {
    let tree = tree(&[
        ("first.js", "const tenants = ['ACME', 'GLOBEX'];"),
        ("second.js", "const tenants = ['ACME', 'GLOBEX'];"),
    ]);
    let project = load_project(tree.path()).unwrap();
    let findings = project
        .detect("shotgun")
        .expect("source evidence needs no Git history");
    assert_eq!(findings.len(), 2);
}

#[test]
fn unrelated_repeated_strings_and_differently_named_catalogues_are_not_linked() {
    assert!(
        scan(&[
            (
                "payment.ts",
                "enum PaymentState { PENDING, ACTIVE } const label = 'ACME';"
            ),
            (
                "account.ts",
                "enum AccountState { PENDING, ACTIVE } const label = 'ACME';"
            ),
        ])
        .is_empty()
    );
}

#[test]
fn explicit_domains_and_schema_versions_keep_independent_enums_separate() {
    for (left, right) in [("Billing", "Identity"), ("v1", "v2")] {
        let first = format!("namespace {left} {{ export enum State {{ PENDING, ACTIVE }} }}");
        let second = format!("namespace {right} {{ export enum State {{ PENDING, ACTIVE }} }}");
        assert!(scan(&[("first.ts", &first), ("second.ts", &second)]).is_empty());
    }
}

#[test]
fn generated_copies_do_not_create_multiple_manual_authorities() {
    let findings = scan(&[
        ("tenant.ts", "export enum Tenant { ACME, GLOBEX }"),
        (
            "client.ts",
            "// Code generated by schema compiler. DO NOT EDIT.\nexport enum Tenant { ACME, GLOBEX }",
        ),
        (
            "other.ts",
            "// @generated by schema compiler\nexport enum Tenant { ACME, GLOBEX }",
        ),
    ]);
    assert!(findings.is_empty());
}

#[test]
fn a_marker_inside_a_string_does_not_make_the_file_generated() {
    assert_eq!(
        scan(&[
            (
                "first.ts",
                "const notice = 'Code generated. DO NOT EDIT.'; enum Tenant { ACME, GLOBEX }"
            ),
            ("second.ts", "enum Tenant { ACME, GLOBEX }"),
        ])
        .len(),
        2
    );
}

#[test]
fn different_members_dynamic_lists_and_a_single_literal_do_not_establish_parallel_sets() {
    for other in [
        "const tenants = ['ACME', 'INITECH'];",
        "const tenants = ['ACME', ...configured];",
        "const tenants = ['ACME'];",
        "const tenants = [Tenant.ACME, Tenant.GLOBEX];",
        "const tenants = ['ACME', , 'GLOBEX'];",
        "const [tenants] = ['ACME', 'GLOBEX'];",
    ] {
        assert!(
            scan(&[
                ("first.js", "const tenants = ['ACME', 'GLOBEX'];"),
                ("second.js", other)
            ])
            .is_empty(),
            "{other}"
        );
    }
}

#[test]
fn different_encoded_enum_values_are_not_treated_as_the_same_set() {
    assert!(
        scan(&[
            ("first.ts", "enum State { A = 'old-a', B = 'old-b' }"),
            ("second.ts", "enum State { A = 'new-a', B = 'new-b' }"),
        ])
        .is_empty()
    );
}

#[test]
fn repeated_uses_in_one_file_are_not_independent_declarations() {
    assert!(
        scan(&[(
            "tenant.ts",
            "enum Tenant { ACME, GLOBEX }
        const tenants = [Tenant.ACME, Tenant.GLOBEX];"
        )])
        .is_empty()
    );
}

#[test]
fn literal_lists_and_explicit_string_enums_share_membership() {
    let findings = scan(&[
        (
            "first.rs",
            "const TENANTS: [&str; 2] = [\"acme\", \"globex\"];",
        ),
        (
            "second.java",
            "class Registration { String[] tenants = {\"globex\", \"acme\"}; }",
        ),
        (
            "third.ts",
            "enum Tenant { ACME = 'acme', GLOBEX = 'globex' }",
        ),
    ]);
    assert_eq!(findings.len(), 3);
}

#[test]
fn explicit_java_packages_and_rust_modules_preserve_domains() {
    for sources in [
        [
            (
                "first.java",
                "package billing; enum State { PENDING, ACTIVE }",
            ),
            (
                "second.java",
                "package identity; enum State { PENDING, ACTIVE }",
            ),
        ],
        [
            ("first.rs", "mod billing { enum State { Pending, Active } }"),
            (
                "second.rs",
                "mod identity { enum State { Pending, Active } }",
            ),
        ],
    ] {
        assert!(scan(&sources).is_empty());
    }
}

#[test]
fn incomplete_source_is_an_error_and_cli_modes_are_exclusive() {
    let malformed = tree(&[("broken.ts", "enum Tenant { ACME,")]);
    assert!(
        load_project(malformed.path())
            .unwrap()
            .detect_source("shotgun")
            .is_err()
    );
    let output = Command::new(env!("CARGO_BIN_EXE_bluecheese"))
        .arg("-f")
        .arg(malformed.path())
        .args(["-s", "shotgun", "--source-only", "--reference"])
        .output()
        .unwrap();
    assert!(!output.status.success());
}
