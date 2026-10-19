//! Switch Statements: report switch-like control flow in methods.
//!
//! Java's grammar represents both switch statements and switch expressions as
//! `switch_expression`; Rust's corresponding construct is `match_expression`.
//! Findings point to the start of each construct and are grouped by type.
//! Java `if` / `else if` ladders that the reference detector also treats as
//! switches are not covered yet.

use crate::finding::Finding;
use crate::lang::{SourceFile, model_for};
use crate::walk::descendants;

pub const SMELL_NAME: &str = "Switch Statements";

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
                    if is_switch(file, node.kind()) {
                        lines.push(file.line(node));
                    }
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

fn is_switch(file: &SourceFile, kind: &str) -> bool {
    (file.language == crate::lang::LanguageId::Java && kind == "switch_expression")
        || (file.language == crate::lang::LanguageId::Rust && kind == "match_expression")
}
