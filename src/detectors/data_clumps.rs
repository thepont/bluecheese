//! Java methods and constructors that repeatedly carry the same parameter group.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::finding::Finding;
use crate::lang::{LanguageId, SourceFile, model_for};
use crate::walk::descendants;
use tree_sitter::Node;

pub const SMELL_NAME: &str = "Data Clumps";
const MIN_SHARED_PARAMETERS: usize = 2;
type Parameters = BTreeMap<String, Option<String>>;

pub fn detect(files: &[SourceFile]) -> Vec<Finding> {
    files.iter().flat_map(detect_file).collect()
}

fn detect_file(file: &SourceFile) -> Vec<Finding> {
    let mut findings = type_findings(file);
    findings.extend(repeated_argument_groups(file));
    findings
}

fn type_findings(file: &SourceFile) -> Vec<Finding> {
    let model = model_for(file.language);
    model
        .method_comparison_owners(file)
        .into_iter()
        .filter_map(|ty| ParameterAnalysis { file, ty }.finding())
        .collect()
}

struct ParameterAnalysis<'a> {
    file: &'a SourceFile,
    ty: Node<'a>,
}

impl ParameterAnalysis<'_> {
    fn finding(&self) -> Option<Finding> {
        let model = model_for(self.file.language);
        let class_name = model.type_name(self.file.at(self.ty))?;
        let lines = self.repeated_parameter_lines();

        (!lines.is_empty()).then(|| Finding {
            smell_name: SMELL_NAME.to_string(),
            file_path: self.file.rel.clone(),
            class_name: class_name.to_string(),
            line_numbers: lines.into_iter().collect(),
            relative_file_path: self.file.rel.clone(),
            additional_info: String::new(),
        })
    }

    fn repeated_parameter_lines(&self) -> BTreeSet<usize> {
        let model = model_for(self.file.language);
        let groups: Vec<_> = model
            .methods(self.file.at(self.ty))
            .iter()
            .map(|method| {
                (
                    model.decl_line(*method),
                    parameter_names(*method, self.file),
                )
            })
            .collect();
        matching_method_lines(&groups)
    }
}

fn matching_method_lines(groups: &[(usize, Parameters)]) -> BTreeSet<usize> {
    let mut lines = BTreeSet::new();
    for left in 0..groups.len() {
        for right in left + 1..groups.len() {
            let shared = groups[left]
                .1
                .iter()
                .filter(|(name, kind)| {
                    groups[right]
                        .1
                        .get(*name)
                        .is_some_and(|other| kind.is_none() || other.is_none() || kind == &other)
                })
                .count();
            if shared >= MIN_SHARED_PARAMETERS {
                lines.insert(groups[left].0);
                lines.insert(groups[right].0);
            }
        }
    }
    lines
}

fn parameter_names(executable: Node<'_>, file: &SourceFile) -> Parameters {
    let Some(parameters) = executable.child_by_field_name("parameters") else {
        return Parameters::new();
    };
    let mut cursor = parameters.walk();
    parameters
        .named_children(&mut cursor)
        .filter_map(|parameter| {
            let name = (parameter.kind() == "identifier")
                .then_some(parameter)
                .or_else(|| {
                    parameter
                        .child_by_field_name("name")
                        .or_else(|| parameter.child_by_field_name("pattern"))
                })?;
            if !matches!(
                name.kind(),
                "identifier" | "shorthand_property_identifier_pattern"
            ) {
                return None;
            }
            Some((file.text(name).to_string(), parameter_type(file, parameter)))
        })
        .collect()
}

fn parameter_type(file: &SourceFile, parameter: Node<'_>) -> Option<String> {
    let ty = parameter.child_by_field_name("type")?;
    let lifetimes: Vec<_> = descendants(ty)
        .into_iter()
        .filter(|node| node.kind() == "lifetime")
        .map(|node| node.byte_range())
        .collect();
    Some(
        file.text(ty)
            .char_indices()
            .filter(|(offset, ch)| {
                !ch.is_whitespace()
                    && !lifetimes
                        .iter()
                        .any(|range| range.contains(&(ty.start_byte() + offset)))
            })
            .map(|(_, ch)| ch)
            .collect(),
    )
}

fn repeated_argument_groups(file: &SourceFile) -> Vec<Finding> {
    if file.language != LanguageId::Java {
        return Vec::new();
    }
    let declared = declared_methods(file);
    let (calls_by_executable, calls_by_type) = collect_argument_calls(file, &declared);

    let repeated = repeated_executable_groups(calls_by_executable);
    let repeated = repeated_type_groups(calls_by_type, repeated);
    findings_for_groups(file, repeated)
}

fn declared_methods(file: &SourceFile) -> BTreeSet<String> {
    model_for(file.language)
        .types(file)
        .into_iter()
        .flat_map(|ty| model_for(file.language).methods(file.at(ty)))
        .filter_map(|method| method.child_by_field_name("name"))
        .map(|name| file.text(name).to_string())
        .collect()
}

type ExecutableCalls = BTreeMap<(usize, Vec<String>), Vec<usize>>;
type TypeCalls = BTreeMap<(usize, Vec<String>), Vec<(usize, bool)>>;

fn collect_argument_calls(
    file: &SourceFile,
    declared_methods: &BTreeSet<String>,
) -> (ExecutableCalls, TypeCalls) {
    let mut calls_by_executable = ExecutableCalls::new();
    let mut calls_by_type = TypeCalls::new();
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
    (calls_by_executable, calls_by_type)
}

/// Repeated arguments in one executable are a direct data clump.
fn repeated_executable_groups(calls: ExecutableCalls) -> BTreeMap<Vec<String>, Vec<usize>> {
    let mut repeated = BTreeMap::<Vec<String>, Vec<usize>>::new();
    for ((_, variables), lines) in calls {
        if lines.len() >= 2 {
            repeated.insert(variables, ordered_lines(lines));
        }
    }
    repeated
}

/// A constructor forwarding a group and a method reusing it form a clump;
/// sibling constructors alone are ordinary inheritance plumbing.
fn repeated_type_groups(
    calls_by_type: TypeCalls,
    mut repeated: BTreeMap<Vec<String>, Vec<usize>>,
) -> BTreeMap<Vec<String>, Vec<usize>> {
    for ((_, variables), calls) in calls_by_type {
        let has_constructor = calls.iter().any(|(_, constructor)| *constructor);
        let has_method_call = calls.iter().any(|(_, constructor)| !constructor);
        if has_constructor && has_method_call {
            let lines: Vec<_> = calls.into_iter().map(|(line, _)| line).collect();
            repeated.insert(variables, ordered_lines(lines));
        }
    }
    repeated
}

fn ordered_lines(mut lines: Vec<usize>) -> Vec<usize> {
    lines.sort_unstable();
    lines.dedup();
    lines
}

fn findings_for_groups(
    file: &SourceFile,
    repeated: BTreeMap<Vec<String>, Vec<usize>>,
) -> Vec<Finding> {
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
