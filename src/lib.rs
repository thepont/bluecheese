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

use finding::Finding;
use lang::SourceFile;

/// Run a single detector by its StaticNose label.
pub fn detect(files: &[SourceFile], smell: &str) -> Vec<Finding> {
    match smell {
        "long-method" => detectors::long_method::detect(files),
        other => panic!("no ported detector for smell '{other}'"),
    }
}
