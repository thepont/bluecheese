//! Whole-body structural clone candidates, reported only across languages.

use crate::finding::Finding;
use crate::lang::{
    SourceFile,
    structure::{self, Fingerprint, Function},
};
use std::collections::{BTreeMap, BTreeSet};

pub fn detect(files: &[SourceFile]) -> Vec<Finding> {
    let mut groups: BTreeMap<Fingerprint, Vec<Function>> = BTreeMap::new();
    for function in structure::functions(files) {
        groups
            .entry(function.fingerprint.clone())
            .or_default()
            .push(function);
    }
    groups
        .into_values()
        .filter(|group| {
            group
                .iter()
                .map(|function| function.language)
                .collect::<BTreeSet<_>>()
                .len()
                > 1
        })
        .flat_map(findings)
        .collect()
}

fn findings(group: Vec<Function>) -> Vec<Finding> {
    group.iter().map(|function| {
        let partners = function.cross_language_partners(&group);
        Finding { smell_name: super::dup_code::SMELL_NAME.into(), file_path: function.file_path.clone(),
            relative_file_path: function.file_path.clone(), class_name: function.class_name.clone(), line_numbers: vec![function.line],
            additional_info: format!("Cross-language structural duplicate: {partners}. Review numeric and runtime semantics.") }
    }).collect()
}
