//! Abstract Java types with no references beyond their own inheritance tree.

use crate::finding::Finding;
use crate::lang::{LanguageId, SourceFile, model_for};
use crate::walk::descendants;
use tree_sitter::Node;

pub const SMELL_NAME: &str = "Speculative Generality";
const MAX_INTERNAL_REFERENCES: usize = 2;

pub fn detect(files: &[SourceFile]) -> Vec<Finding> {
    let java_files: Vec<_> = files
        .iter()
        .filter(|file| file.language == LanguageId::Java)
        .collect();
    let mut findings = Vec::new();

    for file in &java_files {
        let model = model_for(file.language);
        for ty in model.types(file) {
            if !is_abstract_type(file, ty) {
                continue;
            }
            let Some(name_node) = ty.child_by_field_name("name") else {
                continue;
            };
            let name = file.text(name_node);
            let package = package_name(file);
            let package_files: Vec<_> = java_files
                .iter()
                .filter(|source| package_name(source) == package)
                .collect();
            let uses: usize = package_files
                .iter()
                .map(|source| {
                    descendants(source.root())
                        .into_iter()
                        .filter(|node| is_identifier(*node) && source.text(*node) == name)
                        .count()
                })
                .sum();
            if uses <= MAX_INTERNAL_REFERENCES {
                findings.push(Finding {
                    smell_name: SMELL_NAME.to_string(),
                    file_path: file.rel.clone(),
                    class_name: name.to_string(),
                    line_numbers: vec![file.line(ty)],
                    relative_file_path: file.rel.clone(),
                    additional_info: String::new(),
                });
            }
        }
    }
    findings
}

fn is_abstract_type(file: &SourceFile, ty: Node<'_>) -> bool {
    if ty.kind() == "interface_declaration" {
        return true;
    }
    let mut cursor = ty.walk();
    ty.children(&mut cursor)
        .find(|child| child.kind() == "modifiers")
        .is_some_and(|modifiers| file.text(modifiers).contains("abstract"))
}

fn is_identifier(node: Node<'_>) -> bool {
    matches!(node.kind(), "identifier" | "type_identifier")
}

fn package_name(file: &SourceFile) -> String {
    let mut cursor = file.root().walk();
    file.root()
        .named_children(&mut cursor)
        .find(|node| node.kind() == "package_declaration")
        .map(|node| {
            file.text(node)
                .trim_start_matches("package")
                .trim()
                .trim_end_matches(';')
                .trim()
                .to_string()
        })
        .unwrap_or_default()
}
