use std::path::PathBuf;

use bluecheese::lang::load_project;

fn project(name: &str) -> bluecheese::lang::Project {
    load_project(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures")
            .join(name),
    )
    .expect("load fixture project")
}

#[test]
fn detects_unreferenced_abstract_roots_in_the_specgen_examples() {
    let findings =
        bluecheese::detect(&project("smellbench").files, "spec-gen").expect("run detector");

    for (class_name, suffix) in [
        ("AccomodationBad", "specgen/case1/SGBE1.java"),
        ("MembershipInterface", "specgen/case2/SGBE2.java"),
        ("Shape", "specgen/case3/SGBE3.java"),
    ] {
        assert!(
            findings.iter().any(|finding| {
                finding.class_name == class_name && finding.file_path.ends_with(suffix)
            }),
            "missing {class_name}: {findings:#?}"
        );
    }
}

#[test]
fn clean_refactored_source_has_no_speculative_generality_findings() {
    let findings =
        bluecheese::detect(&project("refactored").files, "spec-gen").expect("run detector");

    assert!(findings.is_empty(), "unexpected findings: {findings:#?}");
}
