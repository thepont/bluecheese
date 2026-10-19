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
fn detects_parallel_factory_subclasses_in_a_smellbench_example() {
    let file = file(
        "smellbench/main/java/parallelhierarchy/case1",
        "PIHBE1.java",
    );
    let findings = bluecheese::detect(&[file], "parallel-hierarchy").expect("run detector");
    let names = findings
        .iter()
        .map(|finding| finding.class_name.as_str())
        .collect::<Vec<_>>();

    for expected in ["CarBad", "BikeBad", "CarFactoryBad", "BikeFactoryBad"] {
        assert!(names.contains(&expected), "missing {expected}: {names:?}");
    }
}
