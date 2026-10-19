//! JS/TS characterisation: project-local static resolution, not runtime proof.
mod common;

use bluecheese::finding::Finding;
use bluecheese::lang::load_project;
use common::TempTree;

fn scan(files: &[(&str, &str)]) -> Vec<Finding> {
    let tree = TempTree::new("ecmascript");
    for (path, source) in files {
        tree.write(path, source);
    }
    let project = load_project(tree.path()).expect("load source");
    assert_eq!(
        project.files.len(),
        files.len(),
        "every requested extension must be supported"
    );
    for file in &project.files {
        assert!(
            !file.root().has_error(),
            "{}: {}",
            file.rel,
            file.root().to_sexp()
        );
    }
    bluecheese::detect(&project.files, "shotgun").expect("run shotgun")
}

#[test]
fn javascript_classes_resolve_inferred_instances_and_distinct_callers() {
    let findings = scan(&[(
        "example.js",
        "class Policy { check() {} } class A { run() { const p = new Policy(); p.check(); p.check(); } } class B { run() { const p = new Policy(); p.check(); } }",
    )]);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].class_name, "Policy");
    assert_eq!(
        findings[0].additional_info,
        "Shotgun Surgery [CM=2, CC=2, Fan-Out=0]"
    );
}

#[test]
fn typescript_classes_resolve_parameter_types() {
    let findings = scan(&[(
        "example.ts",
        "class Policy { check(): void {} } class A { run(p: Policy): void { p.check(); } } class B { run(p: Policy): void { p.check(); } }",
    )]);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].class_name, "Policy");
}

#[test]
fn javascript_and_typescript_imports_work_together() {
    let findings = scan(&[
        ("policy.ts", "export function check(): void {}"),
        (
            "a.js",
            "import {check as validate} from './policy.js'; export function run() { validate(); }",
        ),
        (
            "b.ts",
            "import {check} from './policy'; export const run = () => { check(); };",
        ),
    ]);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].file_path, "policy.ts");
    assert_eq!(
        findings[0].additional_info,
        "Shotgun Surgery [CM=2, CC=2, Fan-Out=0]"
    );
}

#[test]
fn default_and_namespace_imports_resolve_relative_modules() {
    let findings = scan(&[
        ("policy.mjs", "export default function check() {}"),
        (
            "a.mjs",
            "import validate from './policy.mjs'; export function run() { validate(); }",
        ),
        (
            "b.mjs",
            "import * as policy from './policy.mjs'; export function run() { policy.default(); }",
        ),
    ]);
    assert_eq!(findings.len(), 1);
}

#[test]
fn constructors_and_same_owner_calls_do_not_inflate_counts() {
    assert!(scan(&[("example.js", "class Policy { check() {} run() { this.check(); } } class A { constructor() { new Policy().check(); } run() { new Policy().check(); } again() { new Policy().check(); } }")]).is_empty());
}

#[test]
fn unknown_receivers_are_not_joined_by_method_name() {
    assert!(scan(&[("example.js", "class Policy { check() {} } class A { run(p) { p.check(); } } class B { run(p) { p.check(); } }")]).is_empty());
}

#[test]
fn commonjs_names_are_not_assumed_to_be_es_module_imports() {
    assert!(scan(&[("example.cjs", "class Policy { check() {} } class A { run() { const p = require('unknown'); p.check(); } } class B { run() { const p = require('unknown'); p.check(); } }")]).is_empty());
}

#[test]
fn shadowed_imports_and_sibling_block_locals_do_not_leak() {
    assert!(
        scan(&[
            ("policy.js", "export function check() {}"),
            (
                "a.js",
                "import {check} from './policy.js'; function run(check) { check(); }"
            ),
            (
                "b.js",
                "import {check} from './policy.js'; function run() { check(); }"
            ),
        ])
        .is_empty()
    );
    let findings = scan(&[(
        "blocks.js",
        "class Policy { check() {} } class Other { check() {} } class A { run() { const p = new Policy(); { const p = new Other(); p.check(); } p.check(); } } class B { run() { new Policy().check(); } }",
    )]);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].class_name, "Policy");
}

#[test]
fn inherited_interface_contracts_are_excluded_in_typescript() {
    assert!(scan(&[("contract.ts", "interface Root { check(): void; } interface Child extends Root {} class Policy implements Child { check(): void {} } class A { run(p: Policy) { p.check(); } } class B { run(p: Policy) { p.check(); } }")]).is_empty());
}

#[test]
fn field_initializers_and_constructor_parameter_properties_resolve() {
    let findings = scan(&[(
        "fields.ts",
        "class Policy { check() {} } class A { p = new Policy(); run() { this.p.check(); } } class B { constructor(private p: Policy) {} run() { this.p.check(); } }",
    )]);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].class_name, "Policy");
}

#[test]
fn jsx_and_tsx_are_parsed_with_their_own_grammars() {
    scan(&[
        ("view.jsx", "export const View = () => <div>Hello</div>;"),
        (
            "typed.tsx",
            "export const View = (props: {value: string}) => <div>{props.value}</div>;",
        ),
    ]);
}

#[test]
fn self_referential_initializers_and_inheritance_cycles_terminate() {
    assert!(scan(&[("cycles.js", "class A extends B { run() { const p = p; p.check(); } } class B extends A { run() { const p = p; p.check(); } }")]).is_empty());
}

#[test]
fn cli_accepts_a_mixed_javascript_typescript_project() {
    let tree = TempTree::new("ecmascript-cli");
    tree.write("policy.ts", "export class Policy { static check() {} }")
        .write(
            "a.js",
            "import {Policy} from './policy'; class A { run() { Policy.check(); } }",
        )
        .write(
            "b.ts",
            "import {Policy} from './policy'; class B { run() { Policy.check(); } }",
        );
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_bluecheese"))
        .args(["-f", tree.path().to_str().unwrap(), "-s", "shotgun"])
        .output()
        .expect("run CLI");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let findings: Vec<Finding> = serde_json::from_slice(&output.stdout).expect("JSON findings");
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].file_path, "policy.ts");
}

#[test]
fn adapter_distinguishes_constructors_from_ordinary_methods() {
    let file = common::parse(
        "parameters.ts",
        "class Example { constructor(a: number, b: number, c: number, d: number, e: number, f: number) {} sameLengthX() {} }",
    );
    let model = bluecheese::lang::model_for(file.language);
    let class = model
        .types(&file)
        .into_iter()
        .find(|ty| model.type_name(&file, *ty) == Some("Example"))
        .expect("class");
    assert_eq!(model.constructors(&file, class).len(), 1);
    assert_eq!(model.methods(&file, class).len(), 1);
    let findings = bluecheese::detect(&[file], "long-params").expect("long parameters");
    assert_eq!(findings.len(), 1);
}

#[test]
fn object_literal_services_and_class_field_arrows_are_resolved() {
    let findings = scan(&[(
        "objects.js",
        "const policy = { check() {} }; class A { run = () => { policy.check(); }; } class B { run() { policy.check(); } }",
    )]);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].class_name, "policy");
}

#[test]
fn javascript_constructor_field_initialization_resolves() {
    let findings = scan(&[(
        "fields.js",
        "class Policy { check() {} } class A { constructor() { this.p = new Policy(); } run() { this.p.check(); } } class B { run() { new Policy().check(); } }",
    )]);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].class_name, "Policy");
}
