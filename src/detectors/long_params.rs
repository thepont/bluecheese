//! Port of `edu.northeastern.smells.LongParameterListDetector`.
//!
//! A method is reported when it takes more than `MAX_METHOD_PARAMS` parameters,
//! and a constructor when it takes more than `MAX_CONSTRUCTOR_PARAMS`: a
//! constructor is allowed two more, which is the reference tool's choice and not
//! a consequence of anything measurable.
//!
//! A method that only restates a signature it inherited is left out. Its
//! parameter list is fixed by the declaration it overrides, so the smell, and
//! the fix, belong there.
//!
//! The reported line is the declaration's own line, with leading annotations
//! excluded, as the reference tool reports it.

use crate::finding::Finding;
use crate::lang::{SourceFile, model_for};

pub const SMELL_NAME: &str = "Long Parameter List";
pub const MAX_METHOD_PARAMS: usize = 3;
pub const MAX_CONSTRUCTOR_PARAMS: usize = 5;

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
                if model.parameters(method) > MAX_METHOD_PARAMS && !model.is_override(file, method)
                {
                    lines.push(model.decl_line(method));
                }
            }

            for constructor in model.constructors(file, ty) {
                if model.parameters(constructor) > MAX_CONSTRUCTOR_PARAMS {
                    lines.push(model.decl_line(constructor));
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
