//! bluecheese: Fowler code smell detection, ported from StaticNose.
//!
//! The Java tool is the reference implementation; its detectors are ported one
//! at a time and held to its output by `tests/parity.rs`. Detectors are written
//! against [`lang::LanguageModel`], not against a grammar, so the same detector
//! runs on any language that has an adapter.

pub mod detectors;
pub mod finding;
pub mod history;
pub mod lang;
pub mod smells;
pub mod walk;

use anyhow::{Result, anyhow};

use finding::Finding;
use lang::SourceFile;

type Detector = fn(&[SourceFile]) -> Vec<Finding>;

const DETECTORS: &[(&str, Detector)] = &[
    ("data-clumps", detectors::data_clumps::detect),
    ("alt-classes", detectors::alt_classes::detect),
    ("lazy-class", detectors::lazy_class::detect),
    ("prim-obsession", detectors::primitive_obsession::detect),
    ("feature-envy", detectors::feature_envy::detect),
    (
        "incomp-lib-class",
        detectors::incomplete_library_class::detect,
    ),
    ("temp-field", detectors::temp_field::detect),
    ("parallel-hierarchy", detectors::parallel_hierarchy::detect),
    ("divergent-change", detectors::divergent_change::detect),
    ("data-class", detectors::data_class::detect),
    ("middle-man", detectors::middle_man::detect),
    ("large-class", detectors::large_class::detect),
    ("intimacy", detectors::intimacy::detect),
    ("refused-bequest", detectors::refused_bequest::detect),
    ("dup-code", detectors::dup_code::detect),
    ("long-method", detectors::long_method::detect),
    ("long-params", detectors::long_params::detect),
    ("comments", detectors::comments::detect),
    ("shotgun", detectors::shotgun::detect),
    ("message-chains", detectors::message_chains::detect),
    ("switch-stmts", detectors::switch_stmts::detect),
    ("spec-gen", detectors::spec_gen::detect),
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
pub fn detect(files: &[SourceFile], smell: &str) -> Result<Vec<Finding>> {
    let detector = DETECTORS
        .iter()
        .find(|(label, _)| *label == smell)
        .map(|(_, detector)| detector)
        .ok_or_else(|| refused(smell))?;
    Ok(detector(files))
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
