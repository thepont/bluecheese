//! Java methods and constructors that repeatedly carry the same parameter group.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::finding::Finding;
use crate::lang::{LanguageId, SourceFile, model_for};
use crate::walk::descendants;
use tree_sitter::Node;

pub const SMELL_NAME: &str = "Data Clumps";
const MIN_SHARED_PARAMETERS: usize = 2;

pub fn detect(files: &[SourceFile]) -> Vec<Finding> {
    let mut findings = Vec::new();
    for file in files {
        if file.language != LanguageId::Java {
            continue;
        }
        let model = model_for(file.language);
        for ty in model.types(file) {
            if ty.kind() != "class_declaration" {
                continue;
            }
            let Some(class_name) = model.type_name(file, ty) else {
                continue;
            };
            let executables = model.methods(file, ty);
            let groups: Vec<_> = executables
                .iter()
                .map(|executable| {
                    (
                        model.decl_line(*executable),
                        parameter_names(*executable, file),
                    )
                })
                .collect();
            let mut lines = BTreeSet::new();
            for left in 0..groups.len() {
                for right in left + 1..groups.len() {
                    if groups[left].1.intersection(&groups[right].1).count()
                        >= MIN_SHARED_PARAMETERS
                    {
                        lines.insert(groups[left].0);
                        lines.insert(groups[right].0);
                    }
                }
            }
            if !lines.is_empty() {
                findings.push(Finding {
                    smell_name: SMELL_NAME.to_string(),
                    file_path: file.rel.clone(),
                    class_name: class_name.to_string(),
                    line_numbers: lines.into_iter().collect(),
                    relative_file_path: file.rel.clone(),
                    additional_info: String::new(),
                });
            }
        }
        findings.extend(repeated_argument_groups(file));
    }
    findings
}

fn parameter_names(executable: Node<'_>, file: &SourceFile) -> BTreeSet<String> {
    let Some(parameters) = executable.child_by_field_name("parameters") else {
        return BTreeSet::new();
    };
    let mut cursor = parameters.walk();
    parameters
        .named_children(&mut cursor)
        .filter_map(|parameter| parameter.child_by_field_name("name"))
        .map(|name| file.text(name).to_string())
        .collect()
}

fn repeated_argument_groups(file: &SourceFile) -> Vec<Finding> {
    let model = model_for(file.language);
    let declared_methods: BTreeSet<String> = model
        .types(file)
        .into_iter()
        .flat_map(|ty| model.methods(file, ty))
        .filter_map(|method| method.child_by_field_name("name"))
        .map(|name| file.text(name).to_string())
        .collect();
    let mut calls_by_executable: BTreeMap<(usize, Vec<String>), Vec<usize>> = BTreeMap::new();
    let mut calls_by_type: BTreeMap<(usize, Vec<String>), Vec<(usize, bool)>> = BTreeMap::new();
    for call in descendants(file.root()).into_iter().filter(|node| {
        matches!(
            node.kind(),
            "method_invocation" | "explicit_constructor_invocation"
        )
    }) {
        if call.kind() == "method_invocation"
            && call
                .child_by_field_name("name")
                .is_none_or(|name| !declared_methods.contains(file.text(name)))
        {
            continue;
        }
        let Some(arguments) = call.child_by_field_name("arguments") else {
            continue;
        };
        let mut cursor = arguments.walk();
        let variables: BTreeSet<String> = arguments
            .named_children(&mut cursor)
            .filter(|argument| argument.kind() == "identifier")
            .map(|argument| file.text(argument).to_string())
            .collect();
        if variables.len() >= MIN_SHARED_PARAMETERS {
            let variables: Vec<_> = variables.into_iter().collect();
            let is_constructor = call.kind() == "explicit_constructor_invocation";
            let line = file.line(call);
            if let Some(executable) =
                ancestor_of_kind(call, &["method_declaration", "constructor_declaration"])
            {
                calls_by_executable
                    .entry((executable.start_byte(), variables.clone()))
                    .or_default()
                    .push(line);
            }
            if let Some(ty) = ancestor_of_kind(call, &["class_declaration"]) {
                calls_by_type
                    .entry((ty.start_byte(), variables))
                    .or_default()
                    .push((line, is_constructor));
            }
        }
    }

    // Repeated arguments in one executable are a direct data clump. A
    // constructor forwarding the same group and a method using it again in
    // the same type is also a clump; sibling constructors alone are ordinary
    // inheritance plumbing.
    let mut repeated = BTreeMap::<Vec<String>, Vec<usize>>::new();
    for ((_, variables), mut lines) in calls_by_executable {
        if lines.len() >= 2 {
            lines.sort_unstable();
            lines.dedup();
            repeated.insert(variables, lines);
        }
    }
    for ((_, variables), calls) in calls_by_type {
        let has_constructor = calls.iter().any(|(_, constructor)| *constructor);
        let has_method_call = calls.iter().any(|(_, constructor)| !constructor);
        if has_constructor && has_method_call {
            let mut lines: Vec<_> = calls.into_iter().map(|(line, _)| line).collect();
            lines.sort_unstable();
            lines.dedup();
            repeated.insert(variables, lines);
        }
    }

    let file_class_name = Path::new(&file.rel)
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or_default();
    repeated
        .into_iter()
        .map(|(variables, lines)| Finding {
            smell_name: SMELL_NAME.to_string(),
            file_path: file.rel.clone(),
            class_name: file_class_name.to_string(),
            line_numbers: lines,
            relative_file_path: file.rel.clone(),
            additional_info: format!("Variables: {}", variables.join(", ")),
        })
        .collect()
}

fn ancestor_of_kind<'tree>(node: Node<'tree>, kinds: &[&str]) -> Option<Node<'tree>> {
    let mut parent = node.parent();
    while let Some(current) = parent {
        if kinds.contains(&current.kind()) {
            return Some(current);
        }
        parent = current.parent();
    }
    None
}
