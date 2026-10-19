//! Message Chains: report continuous getter chains of three or more calls.
//!
//! StaticNose's detector treats Java `get*` and `is*` calls as a chain and
//! ignores ordinary fluent APIs such as `StringBuilder` and streams.

use crate::finding::Finding;
use crate::lang::{SourceFile, model_for};
use crate::walk::descendants;

pub const SMELL_NAME: &str = "Message Chains";
pub const MINIMUM_CALLS: usize = 3;

pub fn detect(files: &[SourceFile]) -> Vec<Finding> {
    let mut findings = Vec::new();

    for file in files {
        let model = model_for(file.language);

        for ty in model.types(file) {
            let Some(class_name) = model.type_name(file, ty) else {
                continue;
            };

            let mut lines = Vec::new();
            for method in model.methods(file, ty) {
                for node in descendants(method) {
                    if model.accessor_chain_depth(file, node) >= MINIMUM_CALLS {
                        lines.push(file.line(node));
                    }
                }
            }
            lines.sort_unstable();
            lines.dedup();

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
