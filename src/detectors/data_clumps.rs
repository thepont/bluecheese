//! Java methods and constructors that repeatedly carry the same parameter group.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::finding::Finding;
use crate::lang::{model_for, LanguageId, SourceFile};
use crate::walk::descendants;
use tree_sitter::Node;

pub const SMELL_NAME: &str = "Data Clumps";
const MIN_SHARED_PARAMETERS: usize = 2;

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
        .types(file)
        .into_iter()
        .filter(|ty| match file.language {
            LanguageId::Java => ty.kind() == "class_declaration",
            LanguageId::Rust => matches!(
                ty.kind(),
                "source_file" | "struct_item" | "enum_item" | "trait_item" | "union_item"
            ),
            LanguageId::JavaScript | LanguageId::TypeScript | LanguageId::Tsx => matches!(
                ty.kind(),
                "program"
                    | "class_declaration"
                    | "class"
                    | "abstract_class_declaration"
                    | "interface_declaration"
                    | "object"
            ),
        })
        .filter_map(|ty| type_finding(file, ty))
        .collect()
}

fn type_finding(file: &SourceFile, ty: Node<'_>) -> Option<Finding> {
    let model = model_for(file.language);
    let class_name = model.type_name(file, ty)?;
    let lines = repeated_parameter_lines(file, ty);

    (!lines.is_empty()).then(|| Finding {
        smell_name: SMELL_NAME.to_string(),
        file_path: file.rel.clone(),
        class_name: class_name.to_string(),
        line_numbers: lines.into_iter().collect(),
        relative_file_path: file.rel.clone(),
        additional_info: String::new(),
    })
}

fn repeated_parameter_lines(file: &SourceFile, ty: Node<'_>) -> BTreeSet<usize> {
    let model = model_for(file.language);
    let groups: Vec<_> = model
        .methods(file, ty)
        .iter()
        .map(|method| (model.decl_line(*method), parameter_names(*method, file)))
        .collect();
    matching_method_lines(&groups)
}

fn matching_method_lines(groups: &[(usize, BTreeSet<String>)]) -> BTreeSet<usize> {
    let mut lines = BTreeSet::new();
    for left in 0..groups.len() {
        for right in left + 1..groups.len() {
            if groups[left].1.intersection(&groups[right].1).count() >= MIN_SHARED_PARAMETERS {
                lines.insert(groups[left].0);
                lines.insert(groups[right].0);
            }
        }
    }
    lines
}

fn parameter_names(executable: Node<'_>, file: &SourceFile) -> BTreeSet<String> {
    let Some(parameters) = executable.child_by_field_name("parameters") else {
        return BTreeSet::new();
    };
    let mut cursor = parameters.walk();
    parameters
        .named_children(&mut cursor)
        .filter_map(|parameter| {
            (parameter.kind() == "identifier")
                .then_some(parameter)
                .or_else(|| {
                    parameter
                        .child_by_field_name("name")
                        .or_else(|| parameter.child_by_field_name("pattern"))
                })
        })
        .filter(|name| {
            matches!(
                name.kind(),
                "identifier" | "shorthand_property_identifier_pattern"
            )
        })
        .map(|name| file.text(name).to_string())
        .collect()
}

fn repeated_argument_groups(file: &SourceFile) -> Vec<Finding> {
    if file.language != LanguageId::Java {
        return Vec::new();
    }
    let declared = declared_methods(file);
    let (calls_by_executable, calls_by_type) = collect_argument_calls(file, &declared);

    // Repeated arguments in one executable are a direct data clump. A
    // constructor forwarding the same group and a method using it again in
    // the same type is also a clump; sibling constructors alone are ordinary
    // inheritance plumbing.
    let repeated = repeated_executable_groups(calls_by_executable);
    let repeated = repeated_type_groups(calls_by_type, repeated);
    findings_for_groups(file, repeated)
}

fn declared_methods(file: &SourceFile) -> BTreeSet<String> {
    model_for(file.language)
        .types(file)
        .into_iter()
        .flat_map(|ty| model_for(file.language).methods(file, ty))
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

fn repeated_executable_groups(calls: ExecutableCalls) -> BTreeMap<Vec<String>, Vec<usize>> {
    let mut repeated = BTreeMap::<Vec<String>, Vec<usize>>::new();
    for ((_, variables), mut lines) in calls {
        if lines.len() >= 2 {
            lines.sort_unstable();
            lines.dedup();
            repeated.insert(variables, lines);
        }
    }
    repeated
}

fn repeated_type_groups(
    calls_by_type: TypeCalls,
    mut repeated: BTreeMap<Vec<String>, Vec<usize>>,
) -> BTreeMap<Vec<String>, Vec<usize>> {
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
    repeated
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
