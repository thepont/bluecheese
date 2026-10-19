//! Best-effort Feature Envy heuristic for Java methods.
//!
//! A method is flagged when it reads at least three distinct fields through
//! the same named receiver. The rule focuses on explicit field access and does
//! not infer ownership from unqualified names or accessor calls.

use std::collections::{BTreeMap, BTreeSet};

use crate::finding::Finding;
use crate::lang::{LanguageId, LanguageModel, SourceFile, java};
use crate::walk::descendants;

pub const SMELL_NAME: &str = "Feature Envy";
const MIN_FIELDS_ON_RECEIVER: usize = 3;

pub fn detect(files: &[SourceFile]) -> Vec<Finding> {
    files
        .iter()
        .filter(|file| file.language == LanguageId::Java)
        .flat_map(file_findings)
        .collect()
}

fn file_findings(file: &SourceFile) -> Vec<Finding> {
    java::JAVA
        .types(file)
        .into_iter()
        .filter(|ty| ty.kind() == "class_declaration")
        .flat_map(|ty| {
            let Some(class_name) = java::JAVA.type_name(file, ty).map(str::to_owned) else {
                return Vec::new();
            };
            java::JAVA
                .methods(file, ty)
                .into_iter()
                .filter(|method| is_envying(file, *method))
                .map(|method| Finding {
                    smell_name: SMELL_NAME.to_owned(),
                    file_path: file.rel.clone(),
                    class_name: class_name.clone(),
                    line_numbers: vec![java::JAVA.decl_line(method)],
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
    for access in descendants(body)
        .into_iter()
        .filter(|node| node.kind() == "field_access")
    {
        let (Some(receiver), Some(field)) = (
            access.child_by_field_name("object"),
            access.child_by_field_name("field"),
        ) else {
            continue;
        };
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
