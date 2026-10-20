//! This repository's own source, held to the rule its corpus depends on: code
//! smells are deliberate, and live only where they are declared to.
//!
//! Without this the suite could pass while the code it describes rotted. The
//! exclusion it needs is also the one anyone writing a fixture needs: the
//! fixtures are tracked, so no ignore file will ever leave them out, and the
//! only thing that can is someone saying so.

mod common;

use bluecheese::finding::Finding;
use bluecheese::lang::{Project, Scope, load_project_with};

/// Everything this project ships: its source and its examples.
///
/// `tests/` holds fixtures that are smelly on purpose and `reference/` holds two
/// probes that are long on purpose, because Spoon is asked to read them. Both
/// are excluded here rather than waved through, and the second test below is what
/// stops this from being a scan that quietly found nothing.
fn shipped() -> Scope {
    Scope {
        include: Vec::new(),
        exclude: vec!["tests/".to_string(), "reference/".to_string()],
    }
}

fn source_findings(project: &Project, smell: &str) -> Vec<Finding> {
    bluecheese::detect(&project.files, smell).expect("run a ported detector")
}

#[test]
fn the_projects_own_source_is_free_of_every_smell_the_tool_can_detect() {
    let project = load_project_with(&common::repo(), &shipped()).expect("scan the repository");

    // A scan that read nothing would pass this test. Ten is well under the
    // number of source files here and well over what a broken walk would find.
    assert!(
        project.files.len() >= 10,
        "the scan reached only {} files, so it proves nothing",
        project.files.len()
    );

    // Every ported smell, not one of them: a detector is written and added to
    // the list, and the second port is where a scan pinned to the first would
    // have started missing things.
    for smell in bluecheese::ported_smells() {
        let findings = project.detect(smell).expect("run application analysis");
        assert!(
            findings.is_empty(),
            "'{smell}' outside a fixture is not deliberate: {findings:#?}"
        );
    }
}

#[test]
fn the_exclusion_is_what_keeps_the_corpus_out_of_source_only_scans() {
    // Source-only fixtures pin every detector independently of a repository's
    // history. The scoped application scan above uses the same file walker;
    // this control verifies that the deliberate corpus is reachable.
    let unscoped =
        load_project_with(&common::repo(), &Scope::default()).expect("scan the repository");

    for smell in bluecheese::ported_smells() {
        let findings = source_findings(&unscoped, smell);
        assert!(
            findings
                .iter()
                .any(|finding| finding.relative_file_path.starts_with("tests/fixtures/")),
            "'{smell}' has no deliberate smell in the corpus to find, so the exclusion is untested: {findings:#?}"
        );
    }
}
