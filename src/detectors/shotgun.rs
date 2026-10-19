//! StaticNose's source-only incoming coupling heuristic. Shared services can
//! satisfy these thresholds even after responsibilities have been centralised.

use crate::finding::Finding;
use crate::lang::{SourceFile, coupling};

pub const SMELL_NAME: &str = "Shotgun Surgery";

pub fn detect(files: &[SourceFile]) -> Vec<Finding> {
    coupling::analyse(files)
        .into_iter()
        .filter(|method| method.counts.is_shared_across_classes())
        .map(|method| method.into_finding(SMELL_NAME))
        .collect()
}
