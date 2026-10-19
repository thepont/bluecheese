use std::path::PathBuf;

use bluecheese::lang::load_project;

#[test]
fn detects_repeated_statements_in_the_same_class() {
    let project =
        load_project(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/smellbench"))
            .expect("load SmellBench");

    let findings = bluecheese::detect(&project.files, "dup-code").expect("run detector");

    assert!(findings.iter().any(|finding| {
        finding.class_name == "BankAccountBad"
            && finding.file_path.ends_with("dupcode/case1/DUPCBE1.java")
    }));
    assert!(findings.iter().any(|finding| {
        finding.class_name == "AnalyticEventVariation"
            && finding.file_path.ends_with("dupcode/case2/DUPCBE2.java")
    }));
}

#[test]
fn detects_the_same_expression_across_method_bodies() {
    let project =
        load_project(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/smellbench"))
            .expect("load SmellBench");

    let findings = bluecheese::detect(&project.files, "dup-code").expect("run detector");

    assert!(findings.iter().any(|finding| {
        finding.class_name == "SimpleCalculator"
            && finding.file_path.ends_with("dupcode/case3/DUPCBE3.java")
    }));
}

#[test]
fn clean_refactored_source_has_no_duplicate_code_findings() {
    let project =
        load_project(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/refactored"))
            .expect("load refactored fixtures");

    let findings = bluecheese::detect(&project.files, "dup-code").expect("run detector");

    assert!(findings.is_empty(), "unexpected findings: {findings:#?}");
}
