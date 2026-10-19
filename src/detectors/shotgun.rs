//! StaticNose's source-only incoming coupling heuristic. Shared services can
//! satisfy these thresholds even after responsibilities have been centralised.

use crate::finding::Finding;
use crate::lang::{SourceFile, coupling};

pub const SMELL_NAME: &str = "Shotgun Surgery";

pub fn detect(files: &[SourceFile]) -> Vec<Finding> {
    coupling::analyse(files)
        .into_iter()
        .filter(|method| method.callers >= 2 && method.calling_classes >= 2)
        .map(|method| Finding {
            smell_name: SMELL_NAME.into(),
            file_path: method.file_path.clone(),
            relative_file_path: method.file_path,
            class_name: method.class_name,
            line_numbers: vec![method.line],
            additional_info: format!(
                "Shotgun Surgery [CM={}, CC={}, Fan-Out={}]",
                method.callers, method.calling_classes, method.fan_out
            ),
        })
        .collect()
}
