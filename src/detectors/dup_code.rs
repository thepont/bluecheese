//! Repeated executable fragments within language-model owners.

use std::collections::BTreeSet;

use crate::finding::Finding;
use crate::lang::{SourceFile, model_for};
use crate::walk::descendants;
use tree_sitter::Node;

pub const SMELL_NAME: &str = "Duplicate Code";
const MIN_SHARED_STATEMENTS: usize = 3;

pub fn detect(files: &[SourceFile]) -> Vec<Finding> {
    let mut findings: Vec<_> = files.iter().flat_map(file_findings).collect();
    findings.extend(super::cross_language_duplicates::detect(files));
    findings
}

fn file_findings(file: &SourceFile) -> Vec<Finding> {
    model_for(file.language)
        .method_comparison_owners(file)
        .into_iter()
        .filter_map(|ty| owner_finding(file, ty))
        .collect()
}

fn owner_finding(file: &SourceFile, ty: Node<'_>) -> Option<Finding> {
    let model = model_for(file.language);
    let class_name = model.type_name(file, ty)?;
    let methods: Vec<_> = model
        .methods(file, ty)
        .into_iter()
        .map(|method| MethodFingerprint::new(file, method))
        .collect();
    let lines = duplicate_lines(&methods);
    (!lines.is_empty()).then(|| Finding {
        smell_name: SMELL_NAME.to_string(),
        file_path: file.rel.clone(),
        class_name: class_name.to_string(),
        line_numbers: lines.into_iter().collect(),
        relative_file_path: file.rel.clone(),
        additional_info: String::new(),
    })
}

fn duplicate_lines(methods: &[MethodFingerprint]) -> BTreeSet<usize> {
    let mut lines = BTreeSet::new();
    for left in 0..methods.len() {
        for right in left + 1..methods.len() {
            if methods[left].duplicates(&methods[right]) {
                lines.insert(methods[left].line);
                lines.insert(methods[right].line);
            }
        }
    }
    lines
}

struct MethodFingerprint {
    line: usize,
    statements: BTreeSet<String>,
    arithmetic: BTreeSet<String>,
}

impl MethodFingerprint {
    fn new(file: &SourceFile, method: Node<'_>) -> Self {
        Self {
            line: model_for(file.language).decl_line(method),
            statements: statement_fingerprints(file, method),
            arithmetic: arithmetic_fingerprints(file, method),
        }
    }

    fn duplicates(&self, other: &Self) -> bool {
        self.statements.intersection(&other.statements).count() >= MIN_SHARED_STATEMENTS
            || self
                .arithmetic
                .intersection(&other.arithmetic)
                .next()
                .is_some()
    }
}

fn statement_fingerprints(file: &SourceFile, method: Node<'_>) -> BTreeSet<String> {
    let model = model_for(file.language);
    descendants(method)
        .into_iter()
        .filter(|node| model.is_statement(*node))
        .map(|node| normalize(file.text(node)))
        .collect()
}

fn arithmetic_fingerprints(file: &SourceFile, method: Node<'_>) -> BTreeSet<String> {
    descendants(method)
        .into_iter()
        .filter(|node| node.kind() == "binary_expression")
        .filter(|node| {
            let mut cursor = node.walk();
            node.children(&mut cursor)
                .any(|child| matches!(file.text(child), "+" | "-" | "*" | "/" | "%"))
        })
        .map(|node| normalize(file.text(node)))
        .collect()
}

fn normalize(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}
