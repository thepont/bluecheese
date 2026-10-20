//! Source membership evidence and StaticNose's reference incoming coupling
//! metric. Shared services can satisfy the reference thresholds even after
//! responsibilities have been centralised; it is not application smell evidence.

use crate::finding::Finding;
use crate::lang::{SourceFile, coupling};

mod bundles;
mod source;
pub use bundles::BundleRules;
pub use source::{source_evidence, source_evidence_with};

pub const SMELL_NAME: &str = "Shotgun Surgery";

pub fn detect(files: &[SourceFile]) -> Vec<Finding> {
    coupling::analyse(files)
        .into_iter()
        .filter(|method| method.counts.is_shared_across_classes())
        .map(|method| method.into_finding(SMELL_NAME))
        .collect()
}
