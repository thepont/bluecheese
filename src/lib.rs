//! bluecheese: Fowler code smell detection, ported from StaticNose.
//!
//! The Java tool is the reference implementation. Detectors here are ported
//! one at a time and held to the Java output by the parity tests in
//! `tests/parity.rs`.

pub mod detectors;
pub mod finding;
pub mod java;
pub mod metrics;
pub mod types;
pub mod walk;

use finding::Finding;
use java::JavaFile;

/// Run a single detector by its StaticNose label.
pub fn detect(files: &[JavaFile], smell: &str) -> Vec<Finding> {
    match smell {
        "long-method" => detectors::long_method::detect(files),
        other => panic!("no ported detector for smell '{other}'"),
    }
}
