//! bluecheese: Fowler code smell detection, ported from StaticNose.
//!
//! The Java tool is the reference implementation; its detectors are ported one
//! at a time and held to its output by `tests/parity.rs`. Detectors are written
//! against [`lang::LanguageModel`] where possible. Each registered detector
//! declares its supported languages; a parser alone does not imply support.

pub mod coverage;
pub mod detectors;
pub mod finding;
pub mod history;
pub mod lang;
pub mod smells;
pub mod walk;

use anyhow::{Result, anyhow};

use finding::Finding;
use lang::{LanguageId, SourceFile};

type Detector = fn(&[SourceFile]) -> Vec<Finding>;

type Registration = (&'static str, Detector, &'static [LanguageId]);

const ALL_LANGUAGES: &[LanguageId] = &[
    LanguageId::Java,
    LanguageId::Rust,
    LanguageId::JavaScript,
    LanguageId::TypeScript,
    LanguageId::Tsx,
];
const JAVA_ONLY: &[LanguageId] = &[LanguageId::Java];

const DETECTORS: &[Registration] = &[
    ("data-clumps", detectors::data_clumps::detect, ALL_LANGUAGES),
    ("alt-classes", detectors::alt_classes::detect, JAVA_ONLY),
    ("lazy-class", detectors::lazy_class::detect, JAVA_ONLY),
    (
        "prim-obsession",
        detectors::primitive_obsession::detect,
        ALL_LANGUAGES,
    ),
    ("feature-envy", detectors::feature_envy::detect, JAVA_ONLY),
    (
        "incomp-lib-class",
        detectors::incomplete_library_class::detect,
        ALL_LANGUAGES,
    ),
    ("temp-field", detectors::temp_field::detect, JAVA_ONLY),
    (
        "parallel-hierarchy",
        detectors::parallel_hierarchy::detect,
        JAVA_ONLY,
    ),
    (
        "divergent-change",
        detectors::divergent_change::detect,
        JAVA_ONLY,
    ),
    ("data-class", detectors::data_class::detect, ALL_LANGUAGES),
    ("middle-man", detectors::middle_man::detect, ALL_LANGUAGES),
    ("large-class", detectors::large_class::detect, ALL_LANGUAGES),
    ("intimacy", detectors::intimacy::detect, ALL_LANGUAGES),
    (
        "refused-bequest",
        detectors::refused_bequest::detect,
        ALL_LANGUAGES,
    ),
    ("dup-code", detectors::dup_code::detect, ALL_LANGUAGES),
    ("long-method", detectors::long_method::detect, ALL_LANGUAGES),
    ("long-params", detectors::long_params::detect, ALL_LANGUAGES),
    ("comments", detectors::comments::detect, ALL_LANGUAGES),
    ("shotgun", detectors::shotgun::detect, ALL_LANGUAGES),
    (
        "message-chains",
        detectors::message_chains::detect,
        ALL_LANGUAGES,
    ),
    (
        "switch-stmts",
        detectors::switch_stmts::detect,
        ALL_LANGUAGES,
    ),
    ("spec-gen", detectors::spec_gen::detect, ALL_LANGUAGES),
];

const fn detector_labels() -> [&'static str; DETECTORS.len()] {
    let mut labels = [""; DETECTORS.len()];
    let mut index = 0;
    while index < DETECTORS.len() {
        labels[index] = DETECTORS[index].0;
        index += 1;
    }
    labels
}

const PORTED_SMELLS: [&str; DETECTORS.len()] = detector_labels();

/// Run a single detector by its smell label.
/// This retains source-only reference heuristics for corpus comparisons. Use
/// [`lang::Project::detect`] for source-evidence reports with a history fallback.
///
/// Fails when the smell is one the book names but no detector has been ported
/// for yet, and when the label is not a smell of the book at all. Both have to
/// come back as a message: an empty report reads exactly like clean code.
/// Also rejects nonempty input containing no language supported by the detector.
/// For mixed input, use [`lang::Project::coverage`] to account for unsupported files.
pub fn detect(files: &[SourceFile], smell: &str) -> Result<Vec<Finding>> {
    let (_, detector, languages) = registration(smell)?;
    coverage::AnalysisCoverage::new(files, smell, languages)?;
    Ok(detector(files))
}

/// Languages implemented by this detector, independent of the loader's adapters.
pub fn detector_languages(smell: &str) -> Result<&'static [LanguageId]> {
    let (_, _, languages) = registration(smell)?;
    Ok(languages)
}

fn registration(smell: &str) -> Result<&'static Registration> {
    DETECTORS
        .iter()
        .find(|(label, _, _)| *label == smell)
        .ok_or_else(|| refused(smell))
}

/// Why `smell` cannot be run, as a message that says which of the two mistakes
/// it is. A smell waiting for its port wants a plan; anything else wants the
/// list of what the book actually names.
fn refused(smell: &str) -> anyhow::Error {
    if let Some(known) = smells::by_label(smell) {
        anyhow!(
            "no detector for smell {known} yet; ported: {}",
            ported_smells().join(", ")
        )
    } else {
        anyhow!(
            "unknown smell '{smell}'; this tool knows the {} smells of 'Bad Smells in Code': {}",
            smells::SMELLS.len(),
            smells::labels().join(", ")
        )
    }
}

/// The smell labels [`detect`] accepts, for diagnostics.
pub fn ported_smells() -> &'static [&'static str] {
    &PORTED_SMELLS
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The list and the dispatch are written separately, so this is what keeps
    /// them from drifting: a label offered here must actually be accepted.
    #[test]
    fn every_smell_offered_is_one_that_can_be_run() {
        for smell in ported_smells() {
            assert!(
                detect(&[], smell).is_ok(),
                "'{smell}' is offered but not dispatched"
            );
        }
    }

    /// And what is dispatched has to be a smell of the book, or the port is of
    /// something outside the catalogue it claims to be working through.
    #[test]
    fn every_ported_smell_is_one_the_book_names() {
        for smell in ported_smells() {
            assert!(
                smells::by_label(smell).is_some(),
                "'{smell}' is ported but not a smell of the book"
            );
        }
    }
}
