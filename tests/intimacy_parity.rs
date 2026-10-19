use std::collections::BTreeSet;
use std::path::PathBuf;

use bluecheese::finding::Finding;
use bluecheese::lang::{LanguageId, load_project};

fn assert_parity(corpus: &str, golden_name: &str) {
    let fixtures = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let project = load_project(&fixtures.join(corpus)).expect("load fixtures");
    let java: Vec<_> = project
        .files
        .into_iter()
        .filter(|f| f.language == LanguageId::Java)
        .collect();
    let actual: BTreeSet<_> = bluecheese::detect(&java, "intimacy")
        .unwrap()
        .iter()
        .map(Finding::key)
        .collect();
    let expected: BTreeSet<_> = serde_json::from_str::<Vec<Finding>>(
        &std::fs::read_to_string(fixtures.join("golden").join(golden_name)).unwrap(),
    )
    .unwrap()
    .into_iter()
    .filter(|f| f.smell_name == "Inappropriate Intimacy")
    .map(|f| f.key())
    .collect();
    assert_eq!(actual, expected, "{corpus}/Inappropriate Intimacy parity");
}

#[test]
fn matches_reference_on_all_java_corpora() {
    assert_parity("smellbench", "smellbench.json");
    assert_parity("smelly", "smelly.json");
    assert_parity("refactored", "refactored.json");
}
