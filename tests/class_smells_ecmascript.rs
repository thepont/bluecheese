mod common;

use common::parse;

fn names(extension: &str, smell: &str, source: &str) -> Vec<String> {
    let file = parse(&format!("example.{extension}"), source);
    assert!(!file.root().has_error(), "{}", file.root().to_sexp());
    bluecheese::detect(&[file], smell)
        .expect("detector supports ECMAScript")
        .into_iter()
        .map(|finding| finding.class_name)
        .collect()
}

#[test]
fn feature_envy_reads_other_objects_but_not_self_or_method_names() {
    for ext in ["js", "ts", "tsx"] {
        assert_eq!(
            names(
                ext,
                "feature-envy",
                r#"
            class Invoice { total(customer) { return customer.balance + customer.tax + customer.discount; } }
            class Own { total() { return this.balance + this.tax + this.discount; } }
            class Calls { run(other) { other.first(); other.second(); other.third(); } }
            class Small { total(c) { return c.balance + c.tax + c.tax; } }
        "#
            ),
            ["Invoice"]
        );
    }
}

#[test]
fn lazy_class_requires_one_forwarder_and_no_state() {
    for ext in ["js", "ts", "tsx"] {
        assert_eq!(
            names(
                ext,
                "lazy-class",
                r#"
            class Shell { run(value) { return worker.run(value); } }
            class Stateful { count = 0; run(value) { return worker.run(value); } }
            class Initialized { constructor() { this.count = 0; } run(value) { return worker.run(value); } }
            class Useful { run(value) { return worker.run(value); } other() { return 1; } }
            class Compute { run(value) { return worker.run(value) + 1; } }
        "#
            ),
            ["Shell"]
        );
    }
}

#[test]
fn alternative_classes_compare_two_bodies_and_keep_operators() {
    for ext in ["js", "ts", "tsx"] {
        assert_eq!(
            names(
                ext,
                "alt-classes",
                r#"
            class First { start(x) { return x + 1; } stop(x) { return x - 1; } }
            class Second { begin(y) { return y + 2; } end(y) { return y - 2; } }
            class Different { begin(y) { return y * 2; } end(y) { return y / 2; } }
            class Single { start(x) { return x + 1; } }
        "#
            ),
            ["First", "Second"]
        );
    }
}

#[test]
fn parallel_hierarchies_pair_subclasses_but_ignore_one_family() {
    for ext in ["js", "ts", "tsx"] {
        assert_eq!(
            names(
                ext,
                "parallel-hierarchy",
                r#"
            class Animal {} class Tool {}
            class Cat extends Animal {} class Dog extends Animal {}
            class CatTool extends Tool {} class DogTool extends Tool {}
        "#
            ),
            ["Cat", "Dog", "CatTool", "DogTool"]
        );
        assert!(
            names(
                ext,
                "parallel-hierarchy",
                r#"
            class Animal {} class Cat extends Animal {} class Dog extends Animal {}
        "#
            )
            .is_empty()
        );
    }
}

#[test]
fn divergent_change_recognizes_disjoint_state_and_repeated_decisions() {
    for ext in ["js", "ts", "tsx"] {
        assert_eq!(
            names(
                ext,
                "divergent-change",
                r#"
            class Disjoint {
                a = 1; b = 2; c = 3;
                first() { return this.a; } second() { return this.b; } third() { return this.c; }
            }
            class Decisions {
                first() { if (this.mode) return 1; }
                second() { if (this.mode) return 2; }
            }
            class Cohesive {
                value = 1;
                first() { return this.value; } second() { return this.value + 1; } third() { return this.value - 1; }
            }
        "#
            ),
            ["Disjoint", "Decisions"]
        );
    }
}

#[test]
fn temporary_fields_distinguish_conditional_state_from_local_variables() {
    for ext in ["js", "ts", "tsx"] {
        assert_eq!(
            names(
                ext,
                "temp-field",
                r#"
            class Scratch {
                scratch;
                run(enabled) { if (enabled) { this.scratch = 1; return this.scratch; } return 0; }
            }
            class Nullable {
                cache;
                run() { if (this.cache != null) { return this.cache.length; } }
            }
            class Always { total = 0; run() { this.total += 1; return this.total; } }
            class Local { scratch; run(enabled) { if (enabled) { let scratch = 1; return scratch; } } }
            class Other { scratch; run(other, enabled) { if (enabled) return other.scratch; } }
        "#
            ),
            ["Scratch", "Nullable"]
        );
    }
}

#[test]
fn every_migrated_detector_emits_positive_findings_in_every_cli_mode() {
    for ext in ["js", "ts", "tsx"] {
        for (smell, source) in [
            (
                "feature-envy",
                "class Invoice { run(c) { return c.a + c.b + c.c; } }",
            ),
            (
                "lazy-class",
                "class Shell { run() { return worker.run(); } }",
            ),
            (
                "alt-classes",
                "class A { a(x) { return x+1; } b(x) { return x-1; } } class B { c(y) { return y+2; } d(y) { return y-2; } }",
            ),
            (
                "parallel-hierarchy",
                "class Cat extends Animal {} class Dog extends Animal {} class CatTool extends Tool {} class DogTool extends Tool {}",
            ),
            (
                "divergent-change",
                "class Choices { first() { if (this.mode) return 1; } second() { if (this.mode) return 2; } }",
            ),
            (
                "temp-field",
                "class Scratch { scratch; run(ok) { if (ok) { this.scratch = 1; return this.scratch; } } }",
            ),
        ] {
            let tree = common::TempTree::new("migrated-cli");
            tree.write(&format!("example.{ext}"), source);
            for flags in [&[][..], &["--reference"][..], &["--source-only"][..]] {
                let output = std::process::Command::new(env!("CARGO_BIN_EXE_bluecheese"))
                    .arg("-f")
                    .arg(tree.path())
                    .args(["-s", smell])
                    .args(flags)
                    .output()
                    .unwrap();
                assert!(
                    output.status.success(),
                    "{smell} {ext}: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
                let findings: Vec<bluecheese::finding::Finding> =
                    serde_json::from_slice(&output.stdout).unwrap();
                assert!(!findings.is_empty(), "{smell} {ext} {flags:?}");
                assert!(!String::from_utf8_lossy(&output.stderr).contains("unsupported"));
            }
        }
    }
}

#[test]
fn typescript_annotations_private_fields_and_interfaces_are_understood() {
    for ext in ["ts", "tsx"] {
        assert_eq!(
            names(
                ext,
                "temp-field",
                "class Scratch { private scratch: number; run(ok: boolean) { if (ok) { this.scratch = 1; return this.scratch; } } }"
            ),
            ["Scratch"]
        );
        assert_eq!(
            names(
                ext,
                "feature-envy",
                "abstract class Invoice { total(c: Customer): number { return c.a + c.b + c.c; } }"
            ),
            ["Invoice"]
        );
        assert_eq!(
            names(
                ext,
                "parallel-hierarchy",
                "interface Animal {} interface Tool {} class Cat implements Animal {} class Dog implements Animal {} class CatTool implements Tool {} class DogTool implements Tool {}"
            ),
            ["Cat", "Dog", "CatTool", "DogTool"]
        );
    }
}
