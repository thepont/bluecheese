//! bluecheese: Fowler code smell detection, ported from StaticNose.
//!
//! The Java tool is the reference implementation; its detectors are ported one
//! at a time and held to its output by `tests/parity.rs`. Detectors are written
//! against [`lang::LanguageModel`], not against a grammar, so the same detector
//! runs on any language that has an adapter.

pub mod detectors;
pub mod finding;
pub mod lang;
pub mod walk;

use anyhow::{Result, bail};

use finding::Finding;
use lang::SourceFile;

/// Run a single detector by its StaticNose label.
///
/// Fails when no detector has been ported for `smell`. The reference tool
/// exposes nineteen smells and this one exposes one of them, so an unknown label
/// is an ordinary mistake, not a bug: it must come back as a message rather than
/// as an empty report, which would read exactly like clean code.
pub fn detect(files: &[SourceFile], smell: &str) -> Result<Vec<Finding>> {
    match smell {
        "long-method" => Ok(detectors::long_method::detect(files)),
        other => bail!(
            "no detector for smell '{other}'; ported: {}",
            ported_smells().join(", ")
        ),
    }
}

/// The smell labels [`detect`] accepts, for diagnostics.
pub fn ported_smells() -> &'static [&'static str] {
    &["long-method"]
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
}
