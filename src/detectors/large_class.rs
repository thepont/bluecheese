//! Port of StaticNose's WMC, TCC, and field-count rule.

use std::collections::{BTreeMap, BTreeSet};

use crate::finding::Finding;
use crate::lang::{LanguageId, SourceFile, model_for};
use crate::walk::descendants;
use tree_sitter::Node;

pub const SMELL_NAME: &str = "Large Class";
const WMC_THRESHOLD: usize = 47;
const TCC_THRESHOLD: f64 = 0.33;
const FIELD_THRESHOLD: usize = 8;

struct Analysis<'a> {
    file: &'a SourceFile,
    fields: Vec<Node<'a>>,
    methods: Vec<Node<'a>>,
}

pub fn detect(files: &[SourceFile]) -> Vec<Finding> {
    files
        .iter()
        .flat_map(|file| {
            if file.language == LanguageId::Java {
                file_findings(file)
            } else {
                non_java_findings(file)
            }
        })
        .collect()
}

fn non_java_findings(file: &SourceFile) -> Vec<Finding> {
    let model = model_for(file.language);
    model
        .types(file)
        .into_iter()
        .filter_map(|ty| {
            let is_type = match file.language {
                LanguageId::Rust => matches!(ty.kind(), "struct_item" | "enum_item"),
                LanguageId::JavaScript | LanguageId::TypeScript | LanguageId::Tsx => matches!(
                    ty.kind(),
                    "class_declaration" | "class" | "abstract_class_declaration"
                ),
                LanguageId::Java => false,
            };
            if !is_type {
                return None;
            }
            let name = model.type_name(file, ty)?;
            let field_names = non_java_fields(file, ty);
            let methods: Vec<_> = model
                .methods(file, ty)
                .into_iter()
                .filter(|method| model.body(*method).is_some())
                .collect();
            if methods.is_empty() {
                return None;
            }
            let wmc: usize = methods
                .iter()
                .map(|method| model.complexity(file, *method))
                .sum();
            let tcc = non_java_tcc(file, &field_names, &methods);
            if field_names.len() < FIELD_THRESHOLD && !(wmc >= WMC_THRESHOLD && tcc < TCC_THRESHOLD)
            {
                return None;
            }
            Some(Finding {
                smell_name: SMELL_NAME.to_string(),
                file_path: file.rel.clone(),
                class_name: name.to_owned(),
                line_numbers: vec![model.decl_line(ty)],
                relative_file_path: file.rel.clone(),
                additional_info: String::new(),
            })
        })
        .collect()
}

fn non_java_fields(file: &SourceFile, ty: Node<'_>) -> BTreeSet<String> {
    let Some(body) = ty.child_by_field_name("body") else {
        return BTreeSet::new();
    };
    let mut cursor = body.walk();
    let fields = body
        .named_children(&mut cursor)
        .filter(|field| match file.language {
            LanguageId::Rust => field.kind() == "field_declaration",
            LanguageId::JavaScript | LanguageId::TypeScript | LanguageId::Tsx => matches!(
                field.kind(),
                "public_field_definition" | "field_definition" | "property_definition"
            ),
            LanguageId::Java => false,
        });
    fields
        .filter_map(|field| {
            field
                .child_by_field_name("name")
                .or_else(|| field.child_by_field_name("property"))
                .map(|name| file.text(name).trim_start_matches('#').to_owned())
        })
        .collect()
}

fn non_java_tcc(file: &SourceFile, fields: &BTreeSet<String>, methods: &[Node<'_>]) -> f64 {
    if methods.len() < 2 {
        return 1.0;
    }
    let model = model_for(file.language);
    let usages: Vec<_> = methods
        .iter()
        .map(|method| {
            descendants(*method)
                .into_iter()
                .filter_map(|node| model.field_access_parts(node))
                .filter(|(receiver, _)| matches!(receiver.kind(), "self" | "this"))
                .map(|(_, field)| file.text(field).trim_start_matches('#').to_owned())
                .filter(|name| fields.contains(name))
                .collect::<BTreeSet<_>>()
        })
        .collect();
    connected_pair_ratio(&usages)
}

fn file_findings(file: &SourceFile) -> Vec<Finding> {
    let model = model_for(LanguageId::Java);
    model
        .types(file)
        .into_iter()
        .filter_map(|ty| {
            let name = model.type_name(file, ty)?;
            if matches!(
                ty.kind(),
                "interface_declaration" | "enum_declaration" | "annotation_type_declaration"
            ) {
                return None;
            }
            let fields = fields(ty);
            let all_methods = model.methods(file, ty);
            if all_methods.is_empty() {
                return None;
            }
            let analysis = Analysis {
                file,
                fields,
                methods: all_methods
                    .into_iter()
                    .filter(|method| {
                        method.child_by_field_name("body").is_some() && !is_accessor(file, *method)
                    })
                    .collect(),
            };
            let wmc: usize = model
                .methods(file, ty)
                .into_iter()
                .filter(|m| m.child_by_field_name("body").is_some())
                .map(|m| model.complexity(file, m))
                .sum();
            let tcc = tcc(&analysis);
            if (wmc < WMC_THRESHOLD || tcc >= TCC_THRESHOLD)
                && analysis.fields.len() < FIELD_THRESHOLD
            {
                return None;
            }
            Some(Finding {
                smell_name: SMELL_NAME.to_string(),
                file_path: file.rel.clone(),
                class_name: name.to_string(),
                line_numbers: vec![model.decl_line(ty)],
                relative_file_path: file.rel.clone(),
                additional_info: String::new(),
            })
        })
        .collect()
}

fn fields(ty: Node<'_>) -> Vec<Node<'_>> {
    let Some(body) = ty.child_by_field_name("body") else {
        return Vec::new();
    };
    let mut cursor = body.walk();
    body.named_children(&mut cursor)
        .filter(|node| node.kind() == "field_declaration")
        .flat_map(|declaration| {
            let mut cursor = declaration.walk();
            declaration
                .named_children(&mut cursor)
                .filter(|node| node.kind() == "variable_declarator")
                .collect::<Vec<_>>()
        })
        .collect()
}

fn is_accessor(file: &SourceFile, method: Node<'_>) -> bool {
    if has_modifier(file, method, "static") {
        return false;
    }
    let Some(name) = method.child_by_field_name("name") else {
        return false;
    };
    let method_name = file.text(name);
    let Some(params) = method.child_by_field_name("parameters") else {
        return false;
    };
    let param_count = params.named_child_count();
    let Some(result_type) = method.child_by_field_name("type") else {
        return false;
    };
    let is_void = file.text(result_type) == "void";
    let Some(body) = method.child_by_field_name("body") else {
        return false;
    };
    let statement_count = body.named_child_count();
    statement_count <= 2
        && ((!is_void
            && param_count == 0
            && (method_name.starts_with("get") || method_name.starts_with("is")))
            || (is_void && param_count == 1 && method_name.starts_with("set")))
}

fn tcc(analysis: &Analysis<'_>) -> f64 {
    let methods = &analysis.methods;
    if methods.len() < 2 {
        return 1.0;
    }
    let mut usage: BTreeMap<String, BTreeSet<String>> = methods
        .iter()
        .map(|method| (method_key(*method), direct_field_usage(analysis, *method)))
        .collect();
    let counts = field_counts(&usage, methods);
    let glue: BTreeSet<_> = counts
        .into_iter()
        .filter_map(|(name, count)| ((count as f64 / methods.len() as f64) >= 0.70).then_some(name))
        .collect();
    if !usage
        .values()
        .all(|fields| fields.iter().all(|field| glue.contains(field)))
    {
        for fields in usage.values_mut() {
            fields.retain(|field| !glue.contains(field));
        }
    }
    add_same_instance_usage(analysis, &mut usage);
    let sets: Vec<_> = methods
        .iter()
        .map(|method| usage.get(&method_key(*method)).cloned().unwrap_or_default())
        .collect();
    connected_pair_ratio(&sets)
}

fn connected_pair_ratio(usages: &[BTreeSet<String>]) -> f64 {
    if usages.len() < 2 {
        return 1.0;
    }
    let mut connected = 0usize;
    for left in 0..usages.len() {
        for right in left + 1..usages.len() {
            if usages[left]
                .iter()
                .any(|field| usages[right].contains(field))
            {
                connected += 1;
            }
        }
    }
    connected as f64 / (usages.len() * (usages.len() - 1) / 2) as f64
}

fn field_counts(
    usage: &BTreeMap<String, BTreeSet<String>>,
    methods: &[Node<'_>],
) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for method in methods {
        if let Some(fields) = usage.get(&method_key(*method)) {
            for field in fields {
                *counts.entry(field.clone()).or_insert(0) += 1;
            }
        }
    }
    counts
}

fn direct_field_usage(analysis: &Analysis<'_>, method: Node<'_>) -> BTreeSet<String> {
    let field_names: BTreeSet<_> = analysis
        .fields
        .iter()
        .filter_map(|field| {
            field
                .child_by_field_name("name")
                .map(|name| analysis.file.text(name))
        })
        .collect();
    descendants(method)
        .into_iter()
        .filter(|node| {
            node.kind() == "identifier" && field_names.contains(analysis.file.text(*node))
        })
        .map(|node| analysis.file.text(node).to_owned())
        .collect()
}

fn add_same_instance_usage(
    analysis: &Analysis<'_>,
    usage: &mut BTreeMap<String, BTreeSet<String>>,
) {
    let method_names: BTreeMap<_, _> = analysis
        .methods
        .iter()
        .filter_map(|method| {
            let name = method.child_by_field_name("name")?;
            Some((analysis.file.text(name).to_owned(), method_key(*method)))
        })
        .collect();
    let mut calls = Vec::new();
    for method in &analysis.methods {
        let mut callees = BTreeSet::new();
        for call in descendants(*method)
            .into_iter()
            .filter(|node| node.kind() == "method_invocation")
        {
            let same_instance = call
                .child_by_field_name("object")
                .is_none_or(|object| object.kind() == "this");
            if !same_instance {
                continue;
            }
            if let Some(name) = call
                .child_by_field_name("name")
                .and_then(|n| method_names.get(analysis.file.text(n)))
            {
                callees.insert(name.clone());
            }
        }
        calls.push((method_key(*method), callees));
    }
    loop {
        let mut changed = false;
        for (caller, callees) in &calls {
            let additions: Vec<_> = callees
                .iter()
                .flat_map(|callee| usage.get(callee).into_iter().flatten().cloned())
                .collect();
            if let Some(fields) = usage.get_mut(caller) {
                for field in additions {
                    changed |= fields.insert(field);
                }
            }
        }
        if !changed {
            break;
        }
    }
}

fn method_key(method: Node<'_>) -> String {
    format!("{}:{}", method.start_byte(), method.end_byte())
}

fn has_modifier(file: &SourceFile, node: Node<'_>, modifier: &str) -> bool {
    let mut cursor = node.walk();
    node.named_children(&mut cursor)
        .find(|child| child.kind() == "modifiers")
        .is_some_and(|modifiers| {
            file.text(modifiers)
                .split_whitespace()
                .any(|word| word == modifier)
        })
}
