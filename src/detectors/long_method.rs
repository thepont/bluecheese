//! Port of `edu.northeastern.smells.LongMethodDetector`.
//!
//! A method is long when its logical lines of code reach `MAX_LLOC` or its
//! cyclomatic complexity exceeds `MAX_COMPLEXITY`. The reported line is the
//! first line of the method body.

use crate::finding::Finding;
use crate::java::JavaFile;
use crate::metrics::{cyclomatic_complexity, lloc};
use crate::types;

pub const SMELL_NAME: &str = "Long Method";
pub const MAX_LLOC: usize = 30;
pub const MAX_COMPLEXITY: usize = 15;

pub fn detect(files: &[JavaFile]) -> Vec<Finding> {
    let mut findings = Vec::new();

    for file in files {
        for ty in types::all_types(file) {
            let Some(class_name) = types::simple_name(file, ty) else {
                continue;
            };

            let mut lines = Vec::new();
            for method in types::methods(ty) {
                let Some(body) = method.child_by_field_name("body") else {
                    continue;
                };

                let loc = lloc(file, body);
                let complexity = cyclomatic_complexity(file, method);

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
