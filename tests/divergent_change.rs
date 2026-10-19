mod common;

use bluecheese::finding::Finding;
use common::{file, parse};

fn scan(source: &str) -> Vec<Finding> {
    let file = parse("service.java", source);
    bluecheese::detect(&[file], "divergent-change").expect("run Divergent Change")
}

#[test]
fn reports_classes_with_methods_using_disjoint_state() {
    let findings = scan(
        "class Dashboard {\n\
            int width; int height; String title; boolean visible;\n\
            void resize() { this.width = 1; this.height = 1; }\n\
            void rename() { this.title = \"home\"; }\n\
            void show() { this.visible = true; }\n\
        }",
    );

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].class_name, "Dashboard");
}

#[test]
fn ignores_methods_that_share_the_same_state() {
    let findings = scan(
        "class Counter {\n\
            int count;\n\
            int current() { return this.count; }\n\
            int increment() { return ++this.count; }\n\
            int decrement() { return --this.count; }\n\
        }",
    );

    assert!(findings.is_empty());
}

#[test]
fn matches_smellbench_divergent_change_and_refactored_examples() {
    for (dir, name, class_name, expected) in [
        (
            "smellbench/main/java/divergentchange/case1",
            "DVCHBE1.java",
            "SquareMatrixOperationsBad",
            true,
        ),
        (
            "smellbench/main/java/divergentchange/case1",
            "DVCHGE1.java",
            "SquareMatrixOperationsGood",
            false,
        ),
        (
            "smellbench/main/java/divergentchange/case2",
            "DVCHBE2.java",
            "LucyChartService",
            true,
        ),
        (
            "smellbench/main/java/divergentchange/case2",
            "DVCHGE2.java",
            "LucyChartService",
            false,
        ),
        (
            "smellbench/main/java/divergentchange/case3",
            "DVCHBE3.java",
            "Connector",
            true,
        ),
        (
            "smellbench/main/java/divergentchange/case3",
            "DVCHGE3.java",
            "HTTPConnectorVariation",
            false,
        ),
    ] {
        let file = file(dir, name);
        let findings = bluecheese::detect(&[file], "divergent-change").expect("run detector");
        assert_eq!(
            findings
                .iter()
                .any(|finding| finding.class_name == class_name),
            expected,
            "{dir}/{name}"
        );
    }
}
