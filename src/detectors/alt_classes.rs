//! Best-effort Alternative Classes with Different Interfaces heuristic.
//!
//! Within one Java file, report classes that have at least two methods with
//! matching body AST shapes. Identifier names and literal values are ignored,
//! while operators and control-flow structure remain part of the shape. This
//! catches similar implementations with different APIs without treating a
//! project-wide match on generic one-line methods as a pair.

use crate::finding::Finding;
use crate::lang::{LanguageId, LanguageModel, SourceFile, java};
use tree_sitter::Node;

pub const SMELL_NAME: &str = "Alternative Classes with Different Interfaces";

pub fn detect(files: &[SourceFile]) -> Vec<Finding> {
    files
        .iter()
        .filter(|file| file.language == LanguageId::Java)
        .flat_map(file_findings)
        .collect()
}

fn file_findings(file: &SourceFile) -> Vec<Finding> {
    let classes = java::JAVA
        .types(file)
        .into_iter()
        .filter(|ty| ty.kind() == "class_declaration")
        .filter_map(|ty| {
            let name = java::JAVA.type_name(file.at(ty))?.to_owned();
            let methods = java::JAVA
                .methods(file.at(ty))
                .into_iter()
                .filter_map(|method| {
                    method
                        .child_by_field_name("body")
                        .map(|body| body_shape(body))
                        .filter(|shape| !shape.is_empty())
                })
                .collect::<Vec<_>>();
            (methods.len() >= 2).then_some((ty, name, methods))
        })
        .collect::<Vec<_>>();

    let mut reported = vec![false; classes.len()];
    for left in 0..classes.len() {
        for right in left + 1..classes.len() {
            if common_method_shapes(&classes[left].2, &classes[right].2) >= 2 {
                reported[left] = true;
                reported[right] = true;
            }
        }
    }

    classes
        .into_iter()
        .zip(reported)
        .filter_map(|((ty, name, _), reported)| {
            reported.then(|| Finding {
                smell_name: SMELL_NAME.to_owned(),
                file_path: file.rel.clone(),
                class_name: name,
                line_numbers: vec![java::JAVA.decl_line(ty)],
                relative_file_path: file.rel.clone(),
                additional_info: String::new(),
            })
        })
        .collect()
}

fn common_method_shapes(left: &[Vec<String>], right: &[Vec<String>]) -> usize {
    let mut used = vec![false; right.len()];
    let mut matches = 0;
    for left_shape in left {
        if let Some(index) = right
            .iter()
            .enumerate()
            .position(|(index, right_shape)| !used[index] && left_shape == right_shape)
        {
            used[index] = true;
            matches += 1;
        }
    }
    matches
}

fn body_shape(body: Node<'_>) -> Vec<String> {
    let mut shape = Vec::new();
    let mut stack = vec![body];
    while let Some(node) = stack.pop() {
        let kind = node.kind();
        if kind == "identifier" || kind.ends_with("_identifier") {
            continue;
        }
        if kind.ends_with("_literal") || kind == "true" || kind == "false" || kind == "null_literal"
        {
            shape.push("literal".to_owned());
            continue;
        }
        shape.push(kind.to_owned());
        let mut cursor = node.walk();
        let children = node.children(&mut cursor).collect::<Vec<_>>();
        stack.extend(children.into_iter().rev());
    }
    shape
}
