mod common;

use bluecheese::finding::Finding;
use bluecheese::lang::load_project;
use common::TempTree;

const CUSTOMER: &str = "export interface Customer { name: string; email: string; city: string; phone: string; country: string; id: string; }";

fn scan(consumers: &[&str]) -> Vec<Finding> {
    let tree = TempTree::new("member-bundles");
    tree.write("customer.ts", CUSTOMER);
    for (index, body) in consumers.iter().enumerate() {
        tree.write(&format!("consumer{index}.ts"), &format!(
            "import {{ Customer }} from './customer'; export function render(c: Customer) {{ {body} }}"
        ));
    }
    load_project(tree.path())
        .unwrap()
        .detect_source("shotgun")
        .unwrap()
}

#[test]
fn overlapping_partial_representations_of_one_type_are_candidates() {
    let findings = scan(&[
        "return { name: c.name, email: c.email, city: c.city, phone: c.phone };",
        "return [c.name, c.email, c.city, c.country];",
        "return `${c.name}: ${c.email}, ${c.city} (${c.id})`;",
        "return c.id;",
    ]);
    assert_eq!(findings.len(), 3);
    for finding in findings {
        assert!(finding.additional_info.contains("name"));
        assert!(finding.additional_info.contains("email"));
        assert!(finding.additional_info.contains("city"));
        for index in 0..3 {
            assert!(
                finding
                    .additional_info
                    .contains(&format!("consumer{index}.ts:1"))
            );
        }
        assert!(!finding.additional_info.contains("consumer3.ts"));
    }
}

#[test]
fn one_or_two_fields_remain_legitimate_even_across_ten_consumers() {
    for body in [
        "return [c.name];",
        "return { name: c.name, email: c.email };",
        "return [c.name, c.name, c.name, c.email, c.email];",
    ] {
        assert!(scan(&[body; 10]).is_empty(), "{body}");
    }
}

#[test]
fn small_complete_types_are_not_automatically_suspicious() {
    let tree = TempTree::new("small-projections");
    tree.write(
        "pair.ts",
        "interface Pair { x: number; y: number; }
        function a(p: Pair) { return [p.x, p.y]; }
        function b(p: Pair) { return [p.x, p.y]; }
        function c(p: Pair) { return [p.x, p.y]; }",
    );
    assert!(
        load_project(tree.path())
            .unwrap()
            .detect_source("shotgun")
            .unwrap()
            .is_empty()
    );
}

#[test]
fn access_without_representation_and_insufficient_overlap_are_not_candidates() {
    assert!(scan(&["log(c.name); log(c.email); log(c.city);"; 5]).is_empty());
    assert!(
        scan(&[
            "return [c.name, c.email, c.city];",
            "return [c.name, c.phone, c.country];",
            "return [c.id, c.email, c.country];",
        ])
        .is_empty()
    );
    assert!(scan(&["return [c.name, c.email, c.city];"; 2]).is_empty());
}

#[test]
fn separate_expressions_or_instances_do_not_accumulate_a_bundle() {
    assert!(scan(&["const a = [c.name]; const b = [c.email]; return [c.city];"; 5]).is_empty());
    let tree = TempTree::new("separate-instances");
    tree.write("customer.ts", CUSTOMER);
    for index in 0..4 {
        tree.write(&format!("render{index}.ts"),
            "import { Customer } from './customer'; function render(a: Customer, b: Customer, c: Customer) { return [a.name, b.email, c.city]; }");
    }
    assert!(
        load_project(tree.path())
            .unwrap()
            .detect_source("shotgun")
            .unwrap()
            .is_empty()
    );
}

#[test]
fn repetition_within_one_consumer_is_one_maintenance_location() {
    assert!(scan(&["const a = [c.name, c.email, c.city]; const b = [c.name, c.email, c.city]; return [c.name, c.email, c.city];"]).is_empty());
}

#[test]
fn matching_fields_on_unrelated_types_and_unknown_receivers_are_not_joined() {
    let tree = TempTree::new("unrelated-bundles");
    for index in 0..4 {
        tree.write(&format!("other{index}.ts"), &format!(
            "interface Record{index} {{ name: string; email: string; city: string; }} function render(c: Record{index}) {{ return [c.name, c.email, c.city]; }}"
        ));
    }
    assert!(
        load_project(tree.path())
            .unwrap()
            .detect_source("shotgun")
            .unwrap()
            .is_empty()
    );
    assert!(scan(&["return [unknown.name, unknown.email, unknown.city];"; 4]).is_empty());
}

#[test]
fn imported_getters_are_resolved_across_javascript_and_typescript() {
    let tree = TempTree::new("mixed-getter-projections");
    tree.write("customer.ts", "export class Customer { getName() { return 'n'; } getEmail() { return 'e'; } getCity() { return 'c'; } getPhone() { return 'p'; } }");
    for (index, extension) in ["js", "ts", "tsx"].iter().enumerate() {
        tree.write(&format!("render{index}.{extension}"),
            "import { Customer as Person } from './customer'; export function render() { const c = new Person(); return [c.getName(), c.getEmail(), c.getCity()]; }");
    }
    assert_eq!(
        load_project(tree.path())
            .unwrap()
            .detect_source("shotgun")
            .unwrap()
            .len(),
        3
    );
}

#[test]
fn ordinary_zero_argument_commands_are_not_getters() {
    let tree = TempTree::new("commands-not-getters");
    tree.write(
        "device.ts",
        "export class Device { start() {} stop() {} reset() {} }",
    );
    for index in 0..4 {
        tree.write(&format!("run{index}.ts"), "import { Device } from './device'; function run(d: Device) { return [d.start(), d.stop(), d.reset()]; }");
    }
    assert!(
        load_project(tree.path())
            .unwrap()
            .detect_source("shotgun")
            .unwrap()
            .is_empty()
    );
}

#[test]
fn java_getter_projections_use_resolved_declaring_types() {
    let tree = TempTree::new("java-getter-projections");
    tree.write("Customer.java", "class Customer { String getName() { return name; } String getEmail() { return email; } String getCity() { return city; } }");
    for index in 0..3 {
        tree.write(&format!("Consumer{index}.java"), &format!(
            "class Consumer{index} {{ String[] render(Customer c) {{ return new String[] {{c.getName(), c.getEmail(), c.getCity()}}; }} }}"
        ));
    }
    assert_eq!(
        load_project(tree.path())
            .unwrap()
            .detect_source("shotgun")
            .unwrap()
            .len(),
        3
    );
}

#[test]
fn groups_share_a_real_core_and_do_not_rely_on_transitive_similarity() {
    let findings = scan(&[
        "return [c.name, c.email, c.city, c.phone, c.country];",
        "return [c.name, c.email, c.city, c.phone, c.id];",
        "return [c.name, c.email, c.city, c.country, c.id];",
    ]);
    assert_eq!(findings.len(), 3);
    assert!(
        scan(&[
            "return [c.name, c.email, c.city];",
            "return [c.name, c.email, c.city, c.phone];",
            "return [c.email, c.city, c.phone];",
        ])
        .is_empty()
    );
}

#[test]
fn validated_rules_control_breadth_overlap_and_consumer_count() {
    use bluecheese::detectors::shotgun::{BundleRules, source_evidence_with};
    for (members, consumers, overlap) in [(2, 3, 60), (3, 1, 60), (3, 3, 0), (3, 3, 101)] {
        assert!(BundleRules::new(members, consumers, overlap).is_err());
    }
    let tree = TempTree::new("projection-thresholds");
    tree.write("record.ts", "interface Record { a: string; b: string; c: string; d: string; e: string; f: string; g: string; h: string; i: string; j: string; k: string; l: string; }
        function one(r: Record) { return [r.a, r.b, r.c, r.d, r.e, r.f]; }
        function two(r: Record) { return [r.a, r.b, r.c, r.g, r.h, r.i]; }
        function three(r: Record) { return [r.a, r.b, r.c, r.j, r.k, r.l]; }");
    let project = load_project(tree.path()).unwrap();
    for (rules, expected) in [
        (BundleRules::default(), 0),
        (BundleRules::new(3, 3, 50).unwrap(), 3),
        (BundleRules::new(4, 3, 50).unwrap(), 0),
        (BundleRules::new(3, 4, 50).unwrap(), 0),
    ] {
        assert_eq!(
            source_evidence_with(&project.files, &rules).unwrap().len(),
            expected
        );
    }
}

#[test]
fn one_or_two_getters_do_not_become_a_bundle_and_generated_consumers_do_not_count() {
    for expression in [
        "[c.getName()]",
        "[c.getName(), c.getEmail()]",
        "[c.getName(), c.getName(), c.getEmail()]",
    ] {
        let tree = TempTree::new("legitimate-getters");
        tree.write(
            "customer.ts",
            "export class Customer { getName() { return 'n'; } getEmail() { return 'e'; } }",
        );
        for index in 0..10 {
            tree.write(&format!("view{index}.ts"), &format!("import {{ Customer }} from './customer'; function view(c: Customer) {{ return {expression}; }}"));
        }
        assert!(
            load_project(tree.path())
                .unwrap()
                .detect_source("shotgun")
                .unwrap()
                .is_empty()
        );
    }
    let tree = TempTree::new("generated-projections");
    tree.write("customer.ts", CUSTOMER);
    for index in 0..5 {
        let header = if index < 2 { "" } else { "// @generated\n" };
        tree.write(&format!("view{index}.ts"), &format!("{header}import {{ Customer }} from './customer'; function view(c: Customer) {{ return [c.name, c.email, c.city]; }}"));
    }
    assert!(
        load_project(tree.path())
            .unwrap()
            .detect_source("shotgun")
            .unwrap()
            .is_empty()
    );
}

#[test]
fn local_object_projections_and_default_cli_work_without_git() {
    let tree = TempTree::new("object-bundles");
    tree.write(
        "record.js",
        "const c = { name: 'a', email: 'b', city: 'c' };
        function first() { return [c.name, c.email, c.city]; }
        function second() { return { name: c.name, email: c.email, city: c.city }; }
        function third() { return `${c.name}, ${c.email}, ${c.city}`; }",
    );
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_bluecheese"))
        .arg("-f")
        .arg(tree.path())
        .args(["-s", "shotgun"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        serde_json::from_slice::<Vec<Finding>>(&output.stdout)
            .unwrap()
            .len(),
        3
    );
}

#[test]
fn combinatorial_input_returns_an_explicit_incomplete_analysis_error() {
    let tree = TempTree::new("bounded-common-bundles");
    let mut source = String::from("interface Record { ");
    for field in 0..13 {
        source.push_str(&format!("p{field}: string; "));
    }
    source.push_str("}\n");
    for omitted in 0..13 {
        let values = (0..13)
            .filter(|field| *field != omitted)
            .map(|field| format!("r.p{field}"))
            .collect::<Vec<_>>()
            .join(",");
        source.push_str(&format!(
            "function view{omitted}(r: Record) {{ return [{values}]; }}\n"
        ));
    }
    tree.write("record.ts", &source);
    let result = load_project(tree.path()).unwrap().detect_source("shotgun");
    assert!(result.is_err(), "partial analysis must not look complete");
}

#[test]
fn writes_are_not_counted_as_reading_a_representation() {
    for body in [
        "return [c.name = 'n', c.email = 'e', c.city = 'c'];",
        "return [c.name += 'n', c.email += 'e', c.city += 'c'];",
    ] {
        assert!(scan(&[body; 3]).is_empty());
    }
}
