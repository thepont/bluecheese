//! Best-effort Feature Envy heuristic for Java and ECMAScript methods.
//!
//! A method is flagged when it reads at least three distinct fields through
//! the same named receiver. The rule focuses on explicit field access and does
//! not infer ownership from unqualified names or accessor calls.

use std::collections::{BTreeMap, BTreeSet};

use crate::finding::Finding;
use crate::lang::{LanguageId, SourceFile, model_for};
use crate::walk::descendants;

pub const SMELL_NAME: &str = "Feature Envy";
const MIN_FIELDS_ON_RECEIVER: usize = 3;

pub fn detect(files: &[SourceFile]) -> Vec<Finding> {
    files
        .iter()
        .filter(|file| file.language != LanguageId::Rust)
        .flat_map(file_findings)
        .collect()
}

fn file_findings(file: &SourceFile) -> Vec<Finding> {
    model_for(file.language)
        .types(file)
        .into_iter()
        .filter(|ty| model_for(file.language).is_data_type(ty.kind()))
        .flat_map(|ty| {
            let Some(class_name) = model_for(file.language)
                .type_name(file.at(ty))
                .map(str::to_owned)
            else {
                return Vec::new();
            };
            model_for(file.language)
                .methods(file.at(ty))
                .into_iter()
                .filter(|method| is_envying(file, *method))
                .map(|method| Finding {
                    smell_name: SMELL_NAME.to_owned(),
                    file_path: file.rel.clone(),
                    class_name: class_name.clone(),
                    line_numbers: vec![model_for(file.language).decl_line(method)],
                    relative_file_path: file.rel.clone(),
                    additional_info: String::new(),
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn is_envying(file: &SourceFile, method: tree_sitter::Node<'_>) -> bool {
    let Some(body) = method.child_by_field_name("body") else {
        return false;
    };
    let mut fields_by_receiver = BTreeMap::<String, BTreeSet<String>>::new();
    for access in descendants(body) {
        let Some((receiver, field)) = model_for(file.language).field_access_parts(access) else {
            continue;
        };
        if access.parent().is_some_and(|parent| {
            parent.kind() == "call_expression"
                && parent.child_by_field_name("function") == Some(access)
        }) {
            continue;
        }
        if receiver.kind() == "identifier" {
            fields_by_receiver
                .entry(file.text(receiver).to_owned())
                .or_default()
                .insert(file.text(field).to_owned());
        }
    }
    fields_by_receiver
        .values()
        .any(|fields| fields.len() >= MIN_FIELDS_ON_RECEIVER)
}
