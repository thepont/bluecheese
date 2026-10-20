//! Port of `edu.northeastern.smells.LongMethodDetector`.
//!
//! A method is long when its logical lines of code reach `MAX_LLOC` or its
//! cyclomatic complexity exceeds `MAX_COMPLEXITY`. The reported line is the
//! first line of the method body.
//!
//! The thresholds are the Java tool's defaults, tuned on Java source. A
//! language adapter supplies the metrics; the thresholds may well need to
//! become per-language once there is a second adapter.

use crate::finding::Finding;
use crate::lang::{SourceFile, model_for};

pub const SMELL_NAME: &str = "Long Method";
pub const MAX_LLOC: usize = 30;
pub const MAX_COMPLEXITY: usize = 15;

pub fn detect(files: &[SourceFile]) -> Vec<Finding> {
    let mut findings = Vec::new();

    for file in files {
        let model = model_for(file.language);

        for ty in model.types(file) {
            let Some(class_name) = model.type_name(file.at(ty)) else {
                continue;
            };

            let mut lines = Vec::new();
            for method in model.methods(file.at(ty)) {
                let Some(body) = model.body(method) else {
                    continue;
                };

                let loc = model.lloc(file.at(body));
                let complexity = model.complexity(file.at(method));

                if loc >= MAX_LLOC || complexity > MAX_COMPLEXITY {
                    lines.push(file.line(body));
                }
            }

            if !lines.is_empty() {
                findings.push(Finding {
                    smell_name: SMELL_NAME.to_string(),
                    file_path: file.rel.clone(),
                    class_name: class_name.to_string(),
                    line_numbers: lines,
                    relative_file_path: file.rel.clone(),
                    additional_info: String::new(),
                });
            }
        }
    }

    findings
}
