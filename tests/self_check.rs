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

fn long_methods(project: Project) -> Vec<Finding> {
    bluecheese::detect(&project.files, "long-method").expect("run the long-method detector")
}

#[test]
fn the_projects_own_source_has_no_long_methods() {
    let project = load_project_with(&common::repo(), &shipped()).expect("scan the repository");

    // A scan that read nothing would pass this test. Ten is well under the
    // number of source files here and well over what a broken walk would find.
    assert!(
        project.files.len() >= 10,
        "the scan reached only {} files, so it proves nothing",
        project.files.len()
    );

    let findings = long_methods(project);
    assert!(
        findings.is_empty(),
        "a long method outside a fixture is not deliberate: {findings:#?}"
    );
}

#[test]
fn the_exclusion_is_what_keeps_the_corpus_out_of_the_self_scan() {
    // The same scan, without the scope. It must report the corpus, or the test
    // above is passing because the walk never reached the fixtures rather than
    // because of the exclusion — which is exactly the failure a reviewer cannot
    // see from the outside.
    let unscoped =
        load_project_with(&common::repo(), &Scope::default()).expect("scan the repository");
    let findings = long_methods(unscoped);

    assert!(
        findings.iter().any(|finding| finding
            .relative_file_path
            .starts_with("tests/fixtures/smelly/")),
        "the deliberate smells in the corpus must be findable, or the exclusion is untested: {findings:#?}"
    );
}
