//! bluecheese: Fowler code smell detection, ported from StaticNose.
//!
//! The Java tool is the reference implementation; its detectors are ported one
//! at a time and held to its output by `tests/parity.rs`. Detectors are written
//! against [`lang::LanguageModel`], not against a grammar, so the same detector
//! runs on any language that has an adapter.

pub mod detectors;
pub mod finding;
pub mod lang;
pub mod smells;
pub mod walk;

use anyhow::{Result, anyhow};

use finding::Finding;
use lang::SourceFile;

/// Run a single detector by its smell label.
///
/// Fails when the smell is one the book names but no detector has been ported
/// for yet, and when the label is not a smell of the book at all. Both have to
/// come back as a message: an empty report reads exactly like clean code.
pub fn detect(files: &[SourceFile], smell: &str) -> Result<Vec<Finding>> {
    match smell {
        "long-method" => Ok(detectors::long_method::detect(files)),
        "long-params" => Ok(detectors::long_params::detect(files)),
        other => Err(refused(other)),
    }
}

/// Why `smell` cannot be run, as a message that says which of the two mistakes
/// it is. A smell waiting for its port wants a plan; anything else wants the
/// list of what the book actually names.
fn refused(smell: &str) -> anyhow::Error {
    match smells::by_label(smell) {
        Some(known) => anyhow!(
            "no detector for smell '{}' ({}) yet; ported: {}",
            known.label,
            known.name,
            ported_smells().join(", ")
        ),
        None => anyhow!(
            "unknown smell '{smell}'; this tool knows the {} smells of 'Bad Smells in Code': {}",
            smells::SMELLS.len(),
            smells::labels().join(", ")
        ),
    }
}

/// The smell labels [`detect`] accepts, for diagnostics.
pub fn ported_smells() -> &'static [&'static str] {
    &["long-method", "long-params"]
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
