use std::collections::BTreeSet;
use std::path::PathBuf;

use bluecheese::finding::Finding;
use bluecheese::lang::{LanguageId, Project, load_project};

fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

fn golden(name: &str) -> Vec<Finding> {
    let path = fixtures().join("golden").join(name);
    serde_json::from_str(&std::fs::read_to_string(path).expect("read reference report"))
        .expect("golden report is valid JSON")
}

fn assert_parity(corpus: &str, golden_name: &str) {
    let Project { files, .. } = load_project(&fixtures().join(corpus)).expect("load fixtures");
    let java: Vec<_> = files
        .into_iter()
        .filter(|file| file.language == LanguageId::Java)
        .collect();
    let actual: BTreeSet<_> = bluecheese::detect(&java, "data-clumps")
        .expect("run data clumps detector")
        .iter()
        .map(Finding::key)
        .collect();
    let expected: BTreeSet<_> = golden(golden_name)
        .into_iter()
        .filter(|finding| finding.smell_name == "Data Clumps")
        .map(|finding| finding.key())
        .collect();

    assert_eq!(actual, expected, "{corpus}/Data Clumps differs from Java");
}

#[test]
fn matches_java_on_smellbench() {
    assert_parity("smellbench", "smellbench.json");
}

#[test]
fn matches_java_on_the_smelly_corpus() {
    assert_parity("smelly", "smelly.json");
}

#[test]
fn matches_java_on_the_refactored_corpus() {
    assert_parity("refactored", "refactored.json");
}
