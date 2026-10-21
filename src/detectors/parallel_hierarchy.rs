//! Best-effort Parallel Inheritance Hierarchies heuristic for Java and ECMAScript.
//!
//! Within one source file, two base types form a candidate pair when at least
//! two of their direct subclasses have corresponding names with a shared
//! prefix. This approximates repeated subclass creation across related roles.

use std::collections::{BTreeMap, BTreeSet};

use crate::finding::Finding;
use crate::lang::{LanguageId, SourceFile, SourceNode, model_for};
use tree_sitter::Node;

pub const SMELL_NAME: &str = "Parallel Inheritance Hierarchies";
const MIN_NAME_PREFIX_CHARS: usize = 3;

struct TypeInfo<'a> {
    node: Node<'a>,
    name: String,
    parents: Vec<String>,
}

pub fn detect(files: &[SourceFile]) -> Vec<Finding> {
    files
        .iter()
        .filter(|file| file.language != LanguageId::Rust)
        .flat_map(file_findings)
        .collect()
}

fn file_findings(file: &SourceFile) -> Vec<Finding> {
    let types = model_for(file.language)
        .types(file)
        .into_iter()
        .filter_map(|node| {
            let name = model_for(file.language)
                .type_name(file.at(node))?
                .to_owned();
            let parents = parent_types(file, node);
            (!parents.is_empty()).then_some(TypeInfo {
                node,
                name,
                parents,
            })
        })
        .collect::<Vec<_>>();
    let mut children = BTreeMap::<String, Vec<usize>>::new();
    for (index, ty) in types.iter().enumerate() {
        for parent in &ty.parents {
            children.entry(parent.clone()).or_default().push(index);
        }
    }

    let groups = children.into_iter().collect::<Vec<_>>();
    let mut reported = BTreeSet::new();
    for left in 0..groups.len() {
        for right in left + 1..groups.len() {
            let matches = paired_children(&groups[left].1, &groups[right].1, &types);
            if matches.len() >= 2 {
                for (first, second) in matches {
                    reported.insert(first);
                    reported.insert(second);
                }
            }
        }
    }

    reported
        .into_iter()
        .map(|index| {
            let ty = &types[index];
            Finding {
                smell_name: SMELL_NAME.to_owned(),
                file_path: file.rel.clone(),
                class_name: ty.name.clone(),
                line_numbers: vec![model_for(file.language).decl_line(ty.node)],
                relative_file_path: file.rel.clone(),
                additional_info: String::new(),
            }
        })
        .collect()
}

fn parent_types(file: &SourceFile, ty: Node<'_>) -> Vec<String> {
    if file.language != LanguageId::Java {
        let mut cursor = ty.walk();
        return ty
            .named_children(&mut cursor)
            .filter(|node| matches!(node.kind(), "class_heritage" | "extends_type_clause"))
            .flat_map(|heritage| ecmascript_parents(file.at(heritage)))
            .collect();
    }
    let mut parents = Vec::new();
    if let Some(superclass) = ty.child_by_field_name("superclass") {
        parents.extend(type_names(file, superclass));
    }
    if let Some(interfaces) = ty.child_by_field_name("interfaces") {
        parents.extend(type_names(file, interfaces));
    }
    parents
}

fn ecmascript_parents(source: SourceNode<'_>) -> Vec<String> {
    let file = source.file();
    let node = source.node();
    if matches!(
        node.kind(),
        "identifier" | "type_identifier" | "member_expression" | "nested_type_identifier"
    ) {
        return vec![file.text(node).to_owned()];
    }
    if matches!(node.kind(), "type_arguments" | "call_expression") {
        return Vec::new();
    }
    let mut cursor = node.walk();
    node.named_children(&mut cursor)
        .flat_map(|child| ecmascript_parents(file.at(child)))
        .collect()
}

fn type_names(file: &SourceFile, node: Node<'_>) -> Vec<String> {
    if matches!(node.kind(), "type_identifier" | "scoped_type_identifier") {
        return vec![
            file.text(node)
                .rsplit('.')
                .next()
                .unwrap_or(file.text(node))
                .to_owned(),
        ];
    }
    if node.kind() == "type_arguments" {
        return Vec::new();
    }
    let mut cursor = node.walk();
    node.named_children(&mut cursor)
        .flat_map(|child| type_names(file, child))
        .collect()
}

fn paired_children(left: &[usize], right: &[usize], types: &[TypeInfo<'_>]) -> Vec<(usize, usize)> {
    let mut candidates = left
        .iter()
        .flat_map(|&first| {
            right.iter().filter_map(move |&second| {
                let prefix = common_prefix_chars(&types[first].name, &types[second].name);
                (prefix >= MIN_NAME_PREFIX_CHARS).then_some((first, second, prefix))
            })
        })
        .collect::<Vec<_>>();
    candidates.sort_by_key(|(first, second, prefix)| (std::cmp::Reverse(*prefix), *first, *second));

    let mut used_left = BTreeSet::new();
    let mut used_right = BTreeSet::new();
    candidates
        .into_iter()
        .filter_map(|(first, second, _)| {
            if !used_left.contains(&first) && !used_right.contains(&second) {
                used_left.insert(first);
                used_right.insert(second);
                Some((first, second))
            } else {
                None
            }
        })
        .collect()
}

fn common_prefix_chars(left: &str, right: &str) -> usize {
    left.chars()
        .zip(right.chars())
        .take_while(|(left, right)| left == right)
        .count()
}
