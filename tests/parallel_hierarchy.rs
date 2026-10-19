mod common;

use bluecheese::finding::Finding;
use common::{file, parse};

fn scan(source: &str) -> Vec<Finding> {
    let file = parse("hierarchies.java", source);
    bluecheese::detect(&[file], "parallel-hierarchy").expect("run Parallel Inheritance Hierarchies")
}

#[test]
fn reports_two_hierarchies_with_repeatedly_paired_subclass_names() {
    let findings = scan(
        "abstract class Animal {}\n\
        class Cat extends Animal {}\n\
        class Dog extends Animal {}\n\
        abstract class Tool {}\n\
        class CatTool extends Tool {}\n\
        class DogTool extends Tool {}",
    );

    assert_eq!(
        findings
            .iter()
            .map(|finding| finding.class_name.as_str())
            .collect::<Vec<_>>(),
        ["Cat", "Dog", "CatTool", "DogTool"]
    );
}

#[test]
fn ignores_one_hierarchy_and_unrelated_child_names() {
    let findings = scan(
        "abstract class Animal {}\n\
        class Cat extends Animal {}\n\
        class Dog extends Animal {}\n\
        class RemoteTool {}\n\
        class LocalTool {}",
    );

    assert!(findings.is_empty());
}

#[test]
fn detects_all_three_smellbench_hierarchy_examples() {
    for (dir, name, expected) in [
        (
            "smellbench/main/java/parallelhierarchy/case1",
            "PIHBE1.java",
            ["CarBad", "BikeBad", "CarFactoryBad", "BikeFactoryBad"].as_slice(),
        ),
        (
            "smellbench/main/java/parallelhierarchy/case2",
            "PIHBE2.java",
            [
                "iOSAbstractAlertButton",
                "AndroidAbstractAlertButton",
                "iOSAbstractAlertDialog",
                "AndroidAbstractAlertDialog",
            ]
            .as_slice(),
        ),
        (
            "smellbench/main/java/parallelhierarchy/case3",
            "PIHBE3.java",
            ["Vampire", "Ghoul", "VampireWeapon", "GhoulWeapon"].as_slice(),
        ),
    ] {
        let file = file(dir, name);
        let findings = bluecheese::detect(&[file], "parallel-hierarchy").expect("run detector");
        let names = findings
            .iter()
            .map(|finding| finding.class_name.as_str())
            .collect::<Vec<_>>();
        for expected in expected {
            assert!(names.contains(expected), "missing {expected}: {names:?}");
        }
    }
}

#[test]
fn keeps_all_three_refactored_smellbench_examples_clean() {
    for (dir, name) in [
        (
            "smellbench/main/java/parallelhierarchy/case1",
            "PIHGE1.java",
        ),
        (
            "smellbench/main/java/parallelhierarchy/case2",
            "PIHGE2.java",
        ),
        (
            "smellbench/main/java/parallelhierarchy/case3",
            "PIHGE3.java",
        ),
    ] {
        let file = file(dir, name);
        let findings = bluecheese::detect(&[file], "parallel-hierarchy").expect("run detector");
        assert!(findings.is_empty(), "{dir}/{name}: {findings:?}");
    }
}
