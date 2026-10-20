//! Best-effort Incomplete Library Class heuristics for Java, Rust, JavaScript and TypeScript.
//!
//! Without library documentation or usage intent, missing API surface cannot
//! be established from source alone. Findings therefore require repeated,
//! non-trivial implementations around the same imported type.

use std::collections::{BTreeMap, BTreeSet};

use crate::finding::Finding;
use crate::lang::{LanguageId, LanguageModel, SourceFile, SourceNode, java, rust};
use crate::walk::descendants;
use tree_sitter::Node;

pub const SMELL_NAME: &str = "Incomplete Library Class";
const MIN_EXTENSIONS: usize = 2;
const MIN_BODY_SHAPE_NODES: usize = 8;

struct PrototypePatch {
    line: usize,
    shape: Vec<String>,
}

pub fn detect(files: &[SourceFile]) -> Vec<Finding> {
    files
        .iter()
        .flat_map(|file| match file.language {
            LanguageId::Java => java_findings(file),
            LanguageId::Rust => rust_findings(file),
            LanguageId::TypeScript | LanguageId::Tsx => typescript_findings(file),
            LanguageId::JavaScript => javascript_findings(file),
        })
        .collect()
}

fn java_findings(file: &SourceFile) -> Vec<Finding> {
    let imported = imported_types(file);
    let local = java::JAVA
        .types(file)
        .into_iter()
        .filter_map(|ty| java::JAVA.type_name(file.at(ty)).map(str::to_owned))
        .collect::<BTreeSet<_>>();

    let mut groups = ExtensionGroups::new();
    for ty in java::JAVA
        .types(file)
        .into_iter()
        .filter(|ty| ty.kind() == "class_declaration")
    {
        let Some(owner_name) = java::JAVA.type_name(file.at(ty)).map(str::to_owned) else {
            continue;
        };
        for method in java::JAVA.methods(file.at(ty)) {
            let (Some((type_name, receiver)), Some(body)) = (
                extension_parameter(file.at(method)),
                method.child_by_field_name("body"),
            ) else {
                continue;
            };
            if !imported.contains(&type_name)
                || local.contains(&type_name)
                || !extends_receiver(file.at(method), &receiver)
            {
                continue;
            }
            let shape = body_shape(body);
            if shape.len() >= MIN_BODY_SHAPE_NODES {
                groups
                    .entry((type_name, shape))
                    .or_default()
                    .entry(owner_name.clone())
                    .or_insert_with(|| java::JAVA.decl_line(ty));
            }
        }
    }

    let mut findings = repeated_extension_owners(groups)
        .into_iter()
        .map(|((class_name, type_name), line)| Finding {
            smell_name: SMELL_NAME.to_owned(),
            file_path: file.rel.clone(),
            class_name,
            line_numbers: vec![line],
            relative_file_path: file.rel.clone(),
            additional_info: format!(
                "repeated helper behavior extends imported type {}",
                type_name
            ),
        })
        .collect::<Vec<_>>();
    findings.extend(java_repeated_callsite_findings(file, &imported, &local));
    findings
}

fn java_repeated_callsite_findings(
    file: &SourceFile,
    imported: &BTreeSet<String>,
    local: &BTreeSet<String>,
) -> Vec<Finding> {
    let mut groups = ExtensionGroups::new();
    let mut completion_pipelines = ExtensionGroups::new();
    let native_types = java_native_boundary_types(file);
    for class in java::JAVA
        .types(file)
        .into_iter()
        .filter(|ty| ty.kind() == "class_declaration")
    {
        let Some(owner) = java::JAVA.type_name(file.at(class)).map(str::to_owned) else {
            continue;
        };
        for method in java::JAVA.methods(file.at(class)) {
            let Some(body) = method.child_by_field_name("body") else {
                continue;
            };
            let mut receivers = java_imported_parameters(file, method, imported, local);
            receivers.extend(
                imported
                    .iter()
                    .filter(|type_name| !local.contains(*type_name))
                    .map(|type_name| (type_name.clone(), type_name.clone())),
            );
            receivers.extend(
                native_types
                    .iter()
                    .map(|type_name| (type_name.clone(), type_name.clone())),
            );
            if receivers.is_empty() {
                continue;
            }
            if extension_parameter(file.at(method)).is_some_and(|(type_name, receiver)| {
                imported.contains(&type_name)
                    && !local.contains(&type_name)
                    && extends_receiver(file.at(method), &receiver)
            }) {
                continue;
            }

            for (type_name, receiver) in receivers {
                if !java_calls_receiver(file, body, &receiver) {
                    continue;
                }
                let shape = java_callsite_shape(file, body);
                if shape.len() >= MIN_BODY_SHAPE_NODES {
                    groups
                        .entry((type_name.clone(), shape))
                        .or_default()
                        .entry(owner.clone())
                        .or_insert(method.start_position().row + 1);
                }
                if let Some(shape) = java_completion_pipeline_shape(file, method, body, &receiver) {
                    completion_pipelines
                        .entry((type_name.clone(), shape))
                        .or_default()
                        .entry(owner.clone())
                        .or_insert(method.start_position().row + 1);
                }
            }
        }
    }

    let mut repeated = repeated_extension_owners(groups);
    for ((type_name, _), owners) in completion_pipelines {
        if owners.len() < MIN_EXTENSIONS {
            continue;
        }
        for (owner, line) in owners {
            repeated.entry((owner, type_name.clone())).or_insert(line);
        }
    }

    repeated
        .into_iter()
        .map(|((class_name, type_name), line)| Finding {
            smell_name: SMELL_NAME.to_owned(),
            file_path: file.rel.clone(),
            class_name,
            line_numbers: vec![line],
            relative_file_path: file.rel.clone(),
            additional_info: format!(
                "repeated call-site workaround around library boundary type {}",
                type_name
            ),
        })
        .collect()
}

fn java_completion_pipeline_shape(
    file: &SourceFile,
    method: Node<'_>,
    body: Node<'_>,
    receiver: &str,
) -> Option<Vec<String>> {
    let return_type = method.child_by_field_name("type")?;
    if file.text(return_type).trim() != "boolean" {
        return None;
    }

    let calls = descendants(body)
        .into_iter()
        .filter(|node| node.kind() == "method_invocation")
        .collect::<Vec<_>>();
    let has_lrs_query = calls
        .iter()
        .any(|call| java_is_lrs_query(file, *call, receiver));
    if !has_lrs_query
        || !calls.iter().any(|call| {
            call.child_by_field_name("name")
                .is_some_and(|name| file.text(name) == "result")
        })
        || !calls.iter().any(|call| {
            call.child_by_field_name("name")
                .is_some_and(|name| file.text(name) == "completion")
        })
        || !java_uses_any_completion_aggregation(file, body, &calls, receiver)
    {
        return None;
    }

    let mut shape = vec![
        "lrs-completion-projection".to_owned(),
        "return:boolean".to_owned(),
        "query:getStatements".to_owned(),
        "field:result.completion".to_owned(),
    ];
    shape.extend(java_parameter_types(file, method));
    Some(shape)
}

fn java_parameter_types(file: &SourceFile, method: Node<'_>) -> Vec<String> {
    let Some(parameters) = method.child_by_field_name("parameters") else {
        return Vec::new();
    };
    let mut cursor = parameters.walk();
    parameters
        .named_children(&mut cursor)
        .filter(|parameter| parameter.kind() == "formal_parameter")
        .filter_map(|parameter| parameter.child_by_field_name("type"))
        .map(|type_node| {
            let type_name = file.text(type_node).trim();
            format!(
                "param:{}",
                type_name.rsplit('.').next().unwrap_or(type_name)
            )
        })
        .collect()
}

fn java_uses_any_completion_aggregation(
    file: &SourceFile,
    body: Node<'_>,
    calls: &[Node<'_>],
    receiver: &str,
) -> bool {
    let stream_any_match = calls.iter().any(|call| {
        let is_any_match = call
            .child_by_field_name("name")
            .is_some_and(|name| file.text(name) == "anyMatch");
        let has_completion = call
            .child_by_field_name("arguments")
            .is_some_and(|arguments| java_has_call_named(file, arguments, "completion"));
        let query_is_streamed = call
            .child_by_field_name("object")
            .and_then(|stream| {
                (java_call_name(file, stream).as_deref() == Some("stream"))
                    .then(|| stream.child_by_field_name("object"))
                    .flatten()
            })
            .is_some_and(|query| java_is_lrs_query(file, query, receiver));
        is_any_match && has_completion && query_is_streamed
    });
    if stream_any_match {
        return true;
    }

    descendants(body)
        .into_iter()
        .filter(|node| matches!(node.kind(), "for_statement" | "enhanced_for_statement"))
        .any(|loop_node| {
            let Some(loop_body) = loop_node.child_by_field_name("body") else {
                return false;
            };
            let query_precedes_loop_body = descendants(loop_node).into_iter().any(|node| {
                java_is_lrs_query(file, node, receiver) && node.end_byte() <= loop_body.start_byte()
            });
            query_precedes_loop_body
                && descendants(loop_node)
                    .into_iter()
                    .filter(|node| node.kind() == "if_statement")
                    .any(|condition| {
                        let condition_nodes = descendants(condition);
                        let checks_completion = condition_nodes.iter().any(|node| {
                            java_call_name(file, *node).as_deref() == Some("completion")
                        });
                        let exits_with_true = condition_nodes.iter().any(|node| {
                            node.kind() == "return_statement" && file.text(*node).contains("true")
                        });
                        checks_completion && exits_with_true
                    })
        })
}

fn java_is_lrs_query(file: &SourceFile, node: Node<'_>, receiver: &str) -> bool {
    java_call_name(file, node).as_deref() == Some("getStatements")
        && node
            .child_by_field_name("object")
            .is_some_and(|object| file.text(object).trim() == receiver)
}

fn java_call_name(file: &SourceFile, node: Node<'_>) -> Option<String> {
    (node.kind() == "method_invocation")
        .then(|| node.child_by_field_name("name"))
        .flatten()
        .map(|name| file.text(name).to_owned())
}

fn java_has_call_named(file: &SourceFile, node: Node<'_>, name: &str) -> bool {
    descendants(node)
        .into_iter()
        .any(|child| java_call_name(file, child).as_deref() == Some(name))
}

fn java_native_boundary_types(file: &SourceFile) -> BTreeSet<String> {
    java::JAVA
        .types(file)
        .into_iter()
        .filter(|ty| ty.kind() == "class_declaration")
        .filter(|ty| {
            java::JAVA
                .methods(file.at(*ty))
                .into_iter()
                .any(|method| is_native_method(file.at(method)))
        })
        .filter_map(|ty| java::JAVA.type_name(file.at(ty)).map(str::to_owned))
        .collect()
}

fn is_native_method(source: SourceNode<'_>) -> bool {
    let mut cursor = source.node().walk();
    source
        .node()
        .named_children(&mut cursor)
        .find(|child| child.kind() == "modifiers")
        .is_some_and(|modifiers| {
            source
                .file()
                .text(modifiers)
                .split_whitespace()
                .any(|word| word == "native")
        })
}

fn java_imported_parameters(
    file: &SourceFile,
    method: Node<'_>,
    imported: &BTreeSet<String>,
    local: &BTreeSet<String>,
) -> Vec<(String, String)> {
    let Some(parameters) = method.child_by_field_name("parameters") else {
        return Vec::new();
    };
    let mut cursor = parameters.walk();
    parameters
        .named_children(&mut cursor)
        .filter(|parameter| parameter.kind() == "formal_parameter")
        .filter_map(|parameter| {
            let (Some(type_node), Some(name_node)) = (
                parameter.child_by_field_name("type"),
                parameter.child_by_field_name("name"),
            ) else {
                return None;
            };
            let type_name = file
                .text(type_node)
                .trim()
                .trim_end_matches("[]")
                .rsplit('.')
                .next()?
                .to_owned();
            (imported.contains(&type_name) && !local.contains(&type_name))
                .then(|| (type_name, file.text(name_node).to_owned()))
        })
        .collect()
}

fn java_calls_receiver(file: &SourceFile, body: Node<'_>, receiver: &str) -> bool {
    descendants(body).into_iter().any(|node| {
        node.kind() == "method_invocation"
            && node
                .child_by_field_name("object")
                .is_some_and(|object| file.text(object).trim() == receiver)
    })
}

fn java_callsite_shape(file: &SourceFile, body: Node<'_>) -> Vec<String> {
    let mut shape = Vec::new();
    let mut stack = vec![body];
    while let Some(node) = stack.pop() {
        let kind = node.kind();
        let method_name_id = if kind == "method_invocation" {
            if let Some(name) = node.child_by_field_name("name") {
                shape.push(format!("call:{}", file.text(name)));
                Some(name.id())
            } else {
                None
            }
        } else {
            None
        };
        if kind.ends_with("_literal") || matches!(kind, "true" | "false" | "null") {
            shape.push(format!("{}:{}", kind, file.text(node)));
            continue;
        }
        if kind == "identifier" || kind.ends_with("_identifier") || kind.ends_with("_name") {
            continue;
        }
        shape.push(kind.to_owned());
        let mut cursor = node.walk();
        let children = node
            .children(&mut cursor)
            .filter(|child| Some(child.id()) != method_name_id)
            .collect::<Vec<_>>();
        stack.extend(children.into_iter().rev());
    }
    shape
}

fn rust_findings(file: &SourceFile) -> Vec<Finding> {
    let imported = rust_imported_types(file);
    let declarations = rust::RUST.types(file);
    let local_types = declarations
        .iter()
        .filter_map(|ty| rust::RUST.type_name(file.at(*ty)).map(str::to_owned))
        .collect::<BTreeSet<_>>();
    let local_owner_types = descendants(file.root())
        .into_iter()
        .filter(|node| matches!(node.kind(), "struct_item" | "enum_item" | "union_item"))
        .filter_map(|ty| rust::RUST.type_name(file.at(ty)).map(str::to_owned))
        .collect::<BTreeSet<_>>();
    let local_traits = declarations
        .iter()
        .filter(|ty| ty.kind() == "trait_item")
        .filter_map(|ty| rust::RUST.type_name(file.at(*ty)).map(str::to_owned))
        .collect::<BTreeSet<_>>();

    let mut groups = ExtensionGroups::new();
    for implementation in descendants(file.root())
        .into_iter()
        .filter(|node| node.kind() == "impl_item")
    {
        let (Some(trait_node), Some(target_node), Some(body)) = (
            implementation.child_by_field_name("trait"),
            implementation.child_by_field_name("type"),
            implementation.child_by_field_name("body"),
        ) else {
            continue;
        };
        let (Some(trait_name), Some(target_name)) = (
            simple_rust_type(file.text(trait_node)),
            simple_rust_type(file.text(target_node)),
        ) else {
            continue;
        };
        if !local_traits.contains(&trait_name)
            || !imported.contains(&target_name)
            || local_types.contains(&target_name)
        {
            continue;
        }
        for method in direct_rust_functions(body) {
            if !rust_extension_method(file.at(method)) {
                continue;
            }
            let Some(method_body) = method.child_by_field_name("body") else {
                continue;
            };
            let shape = body_shape(method_body);
            if shape.len() >= MIN_BODY_SHAPE_NODES {
                groups
                    .entry((target_name.clone(), shape))
                    .or_default()
                    .entry(trait_name.clone())
                    .or_insert(implementation.start_position().row + 1);
            }
        }
    }

    let mut findings = repeated_extension_owners(groups)
        .into_iter()
        .map(|((class_name, target_name), line)| Finding {
            smell_name: SMELL_NAME.to_owned(),
            file_path: file.rel.clone(),
            class_name,
            line_numbers: vec![line],
            relative_file_path: file.rel.clone(),
            additional_info: format!(
                "repeated extension behavior for imported type {}",
                target_name
            ),
        })
        .collect::<Vec<_>>();
    findings.extend(rust_completion_pipeline_findings(
        file,
        &imported,
        &local_owner_types,
    ));
    findings
}

fn rust_completion_pipeline_findings(
    file: &SourceFile,
    imported: &BTreeSet<String>,
    local_types: &BTreeSet<String>,
) -> Vec<Finding> {
    let mut groups = ExtensionGroups::new();
    let mut candidates = Vec::new();
    for implementation in descendants(file.root())
        .into_iter()
        .filter(|node| node.kind() == "impl_item" && node.child_by_field_name("trait").is_none())
    {
        let (Some(owner_type), Some(body)) = (
            implementation.child_by_field_name("type"),
            implementation.child_by_field_name("body"),
        ) else {
            continue;
        };
        let Some(owner_name) = simple_rust_type(file.text(owner_type)) else {
            continue;
        };
        if !local_types.contains(&owner_name) {
            continue;
        }
        let owner_name = rust_module_path(file, implementation)
            .into_iter()
            .chain(std::iter::once(owner_name))
            .collect::<Vec<_>>()
            .join("::");
        for method in direct_rust_functions(body) {
            candidates.push((owner_name.clone(), method));
        }
    }
    for function in descendants(file.root()).into_iter().filter(|node| {
        node.kind() == "function_item" && !rust_function_belongs_to_impl_or_trait(*node)
    }) {
        if let Some(owner_name) = rust_free_function_owner(file, function) {
            candidates.push((owner_name, function));
        }
    }

    for (owner_name, function) in candidates {
        let Some(body) = function.child_by_field_name("body") else {
            continue;
        };
        let Some((client_type, shape)) =
            rust_completion_pipeline_shape(file, function, body, imported)
        else {
            continue;
        };
        groups
            .entry((client_type, shape))
            .or_default()
            .entry(owner_name)
            .or_insert(function.start_position().row + 1);
    }

    repeated_extension_owners(groups)
        .into_iter()
        .map(|((class_name, type_name), line)| Finding {
            smell_name: SMELL_NAME.to_owned(),
            file_path: file.rel.clone(),
            class_name,
            line_numbers: vec![line],
            relative_file_path: file.rel.clone(),
            additional_info: format!(
                "repeated LRS completion projection around imported type {}",
                type_name
            ),
        })
        .collect()
}

fn rust_function_belongs_to_impl_or_trait(function: Node<'_>) -> bool {
    let mut ancestor = function.parent();
    while let Some(node) = ancestor {
        if matches!(node.kind(), "impl_item" | "trait_item") {
            return true;
        }
        ancestor = node.parent();
    }
    false
}

fn rust_free_function_owner(file: &SourceFile, function: Node<'_>) -> Option<String> {
    let name = function.child_by_field_name("name")?;
    let mut scope_names = vec![file.path.file_stem()?.to_str()?.to_owned()];
    scope_names.extend(rust_module_path(file, function));
    scope_names.push(file.text(name).to_owned());
    Some(scope_names.join("::"))
}

fn rust_module_path(file: &SourceFile, node: Node<'_>) -> Vec<String> {
    let mut ancestor = node.parent();
    let mut modules = Vec::new();
    while let Some(ancestor_node) = ancestor {
        if ancestor_node.kind() == "mod_item"
            && let Some(module_name) = ancestor_node.child_by_field_name("name")
        {
            modules.push(file.text(module_name).to_owned());
        }
        ancestor = ancestor_node.parent();
    }
    modules.reverse();
    modules
}

fn rust_completion_pipeline_shape(
    file: &SourceFile,
    method: Node<'_>,
    body: Node<'_>,
    imported: &BTreeSet<String>,
) -> Option<(String, Vec<String>)> {
    let return_type = method.child_by_field_name("return_type")?;
    if file.text(return_type).trim() != "bool" {
        return None;
    }
    let parameters = method.child_by_field_name("parameters")?;
    let mut cursor = parameters.walk();
    let mut client_parameter = None;
    let mut parameter_shapes = Vec::new();
    for parameter in parameters
        .named_children(&mut cursor)
        .filter(|node| node.kind() == "parameter")
    {
        let Some(type_node) = parameter.child_by_field_name("type") else {
            continue;
        };
        let raw_type = file.text(type_node);
        parameter_shapes.push(format!("param:{}", raw_type.replace(' ', "")));
        let normalized_type = raw_type
            .trim()
            .trim_start_matches('&')
            .trim_start_matches("mut ");
        let Some(type_name) = simple_rust_type(normalized_type) else {
            continue;
        };
        if !imported.contains(&type_name) || client_parameter.is_some() {
            continue;
        }
        let Some(pattern) = parameter
            .child_by_field_name("pattern")
            .filter(|node| node.kind() == "identifier")
        else {
            continue;
        };
        client_parameter = Some((type_name, file.text(pattern).to_owned()));
    }
    let (client_type, receiver) = client_parameter?;

    let calls = descendants(body)
        .into_iter()
        .filter(|node| node.kind() == "call_expression")
        .collect::<Vec<_>>();
    let has_result_completion = calls.iter().any(|call| {
        rust_method_call_name(file, *call).as_deref() == Some("completion")
            && rust_method_receiver(*call).is_some_and(|result| {
                rust_method_call_name(file, result).as_deref() == Some("result")
            })
    });
    if !has_result_completion || !rust_uses_completion_aggregation(file, body, &calls, &receiver) {
        return None;
    }

    let mut shape = vec![
        "lrs-completion-projection".to_owned(),
        "return:bool".to_owned(),
        "query:get_statements".to_owned(),
        "method:result.completion".to_owned(),
    ];
    shape.extend(parameter_shapes);
    Some((client_type, shape))
}

fn rust_uses_completion_aggregation(
    file: &SourceFile,
    body: Node<'_>,
    calls: &[Node<'_>],
    receiver: &str,
) -> bool {
    let stream_any = calls.iter().any(|call| {
        if rust_method_call_name(file, *call).as_deref() != Some("any") {
            return false;
        }
        let Some(iter_call) = rust_method_receiver(*call) else {
            return false;
        };
        rust_method_call_name(file, iter_call).as_deref() == Some("iter")
            && rust_method_receiver(iter_call)
                .is_some_and(|query| rust_is_lrs_query(file, query, receiver))
            && rust_any_projects_completion_from_its_item(file, *call)
    });
    if stream_any {
        return true;
    }

    descendants(body)
        .into_iter()
        .filter(|node| node.kind() == "for_expression")
        .any(|loop_node| {
            let (Some(query), Some(loop_body)) = (
                loop_node.child_by_field_name("value"),
                loop_node.child_by_field_name("body"),
            ) else {
                return false;
            };
            if !rust_is_lrs_query(file, query, receiver) {
                return false;
            }
            let Some(item_name) = loop_node
                .child_by_field_name("pattern")
                .and_then(|pattern| rust_pattern_identifier(file, pattern))
            else {
                return false;
            };
            descendants(loop_body)
                .into_iter()
                .filter(|node| node.kind() == "if_expression")
                .any(|condition| {
                    let has_completion =
                        condition
                            .child_by_field_name("condition")
                            .is_some_and(|cond| {
                                descendants(cond).into_iter().any(|node| {
                                    node.kind() == "call_expression"
                                        && rust_method_call_name(file, node).as_deref()
                                            == Some("completion")
                                        && rust_completion_item_name(file, node).as_deref()
                                            == Some(item_name.as_str())
                                })
                            });
                    let returns_true = condition
                        .child_by_field_name("consequence")
                        .is_some_and(|branch| file.text(branch).contains("return true"));
                    has_completion && returns_true
                })
        })
}

fn rust_any_projects_completion_from_its_item(file: &SourceFile, any_call: Node<'_>) -> bool {
    let Some(arguments) = any_call.child_by_field_name("arguments") else {
        return false;
    };
    let mut cursor = arguments.walk();
    arguments
        .named_children(&mut cursor)
        .filter(|node| node.kind() == "closure_expression")
        .any(|closure| {
            let Some(parameters) = closure.child_by_field_name("parameters") else {
                return false;
            };
            let mut cursor = parameters.walk();
            let Some(item_name) = parameters
                .named_children(&mut cursor)
                .next()
                .and_then(|pattern| rust_pattern_identifier(file, pattern))
            else {
                return false;
            };
            let Some(closure_body) = closure.child_by_field_name("body") else {
                return false;
            };
            descendants(closure_body).into_iter().any(|node| {
                node.kind() == "call_expression"
                    && rust_method_call_name(file, node).as_deref() == Some("completion")
                    && rust_completion_item_name(file, node).as_deref() == Some(item_name.as_str())
            })
        })
}

fn rust_completion_item_name(file: &SourceFile, completion: Node<'_>) -> Option<String> {
    let result = rust_method_receiver(completion)?;
    if rust_method_call_name(file, result).as_deref() != Some("result") {
        return None;
    }
    let item = rust_method_receiver(result)?;
    (item.kind() == "identifier").then(|| file.text(item).to_owned())
}

fn rust_pattern_identifier(file: &SourceFile, pattern: Node<'_>) -> Option<String> {
    (pattern.kind() == "identifier").then(|| file.text(pattern).to_owned())
}

fn rust_is_lrs_query(file: &SourceFile, call: Node<'_>, receiver: &str) -> bool {
    rust_method_call_name(file, call).as_deref() == Some("get_statements")
        && rust_method_receiver(call).is_some_and(|value| file.text(value).trim() == receiver)
}

fn rust_method_call_name<'a>(file: &SourceFile, call: Node<'a>) -> Option<String> {
    let function = call.child_by_field_name("function")?;
    (function.kind() == "field_expression")
        .then(|| function.child_by_field_name("field"))
        .flatten()
        .map(|field| file.text(field).to_owned())
}

fn rust_method_receiver<'a>(call: Node<'a>) -> Option<Node<'a>> {
    call.child_by_field_name("function")?
        .child_by_field_name("value")
}

fn rust_imported_types(file: &SourceFile) -> BTreeSet<String> {
    let mut imported = BTreeSet::new();
    for declaration in descendants(file.root())
        .into_iter()
        .filter(|node| node.kind() == "use_declaration")
    {
        if let Some(argument) = declaration.child_by_field_name("argument") {
            collect_rust_import_names(file, argument, &mut imported);
        }
    }
    imported
}

fn collect_rust_import_names(file: &SourceFile, node: Node<'_>, imported: &mut BTreeSet<String>) {
    match node.kind() {
        "identifier" => {
            imported.insert(file.text(node).to_owned());
        }
        "scoped_identifier" => {
            if let Some(name) = node.child_by_field_name("name") {
                imported.insert(file.text(name).to_owned());
            }
        }
        "use_as_clause" => {
            if let Some(alias) = node.child_by_field_name("alias") {
                imported.insert(file.text(alias).to_owned());
            }
        }
        "scoped_use_list" => {
            if let Some(list) = node.child_by_field_name("list") {
                collect_rust_import_names(file, list, imported);
            }
        }
        "use_list" => {
            let mut cursor = node.walk();
            for child in node.named_children(&mut cursor) {
                collect_rust_import_names(file, child, imported);
            }
        }
        _ => {}
    }
}

fn simple_rust_type(text: &str) -> Option<String> {
    let base = text.split('<').next()?.trim();
    base.rsplit("::")
        .next()
        .map(|name| name.trim().to_owned())
        .filter(|name| !name.is_empty())
}

fn direct_rust_functions(body: Node<'_>) -> Vec<Node<'_>> {
    let mut cursor = body.walk();
    body.named_children(&mut cursor)
        .filter(|node| node.kind() == "function_item")
        .collect()
}

fn rust_extension_method(source: SourceNode<'_>) -> bool {
    let Some(body) = source.node().child_by_field_name("body") else {
        return false;
    };
    descendants(body).into_iter().any(|node| {
        node.kind() == "call_expression"
            && node
                .child_by_field_name("function")
                .is_some_and(|function| {
                    source
                        .file()
                        .text(function)
                        .trim_start()
                        .starts_with("self.")
                })
    })
}

fn typescript_findings(file: &SourceFile) -> Vec<Finding> {
    let imports = ecma_named_imports(file);
    let runtime_patches = ecma_prototype_patches(file);
    descendants(file.root())
        .into_iter()
        .filter(|node| node.kind() == "ambient_declaration")
        .flat_map(|ambient| {
            let Some(module) = descendants(ambient)
                .into_iter()
                .find(|node| node.kind() == "module")
            else {
                return Vec::new();
            };
            let Some(module_name) = module
                .child_by_field_name("name")
                .map(|name| unquote(file.text(name)).to_owned())
            else {
                return Vec::new();
            };
            descendants(module)
                .into_iter()
                .filter(|node| node.kind() == "interface_declaration")
                .filter_map(|interface| {
                    let name = interface
                        .child_by_field_name("name")
                        .map(|n| file.text(n))?;
                    let bindings = imports.get(&module_name)?;
                    let local_name = bindings
                        .iter()
                        .find(|(imported, local)| {
                            imported == name && runtime_patches.contains_key(local)
                        })
                        .map(|(_, local)| local)?;
                    let signatures = descendants(interface)
                        .into_iter()
                        .filter(|node| node.kind() == "method_signature")
                        .count();
                    let patches = runtime_patches.get(local_name)?;
                    let duplicate_lines = duplicated_patch_lines(patches);
                    if signatures < MIN_EXTENSIONS || duplicate_lines.len() < MIN_EXTENSIONS {
                        return None;
                    }
                    Some(Finding {
                        smell_name: SMELL_NAME.to_owned(),
                        file_path: file.rel.clone(),
                        class_name: name.to_owned(),
                        line_numbers: duplicate_lines,
                        relative_file_path: file.rel.clone(),
                        additional_info: format!(
                            "repeated runtime implementation of module-augmented type {} from {}",
                            name, module_name
                        ),
                    })
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn javascript_findings(file: &SourceFile) -> Vec<Finding> {
    let imports = ecma_named_imports(file);
    let patches = ecma_prototype_patches(file);
    let mut findings = Vec::new();
    for (module, bindings) in imports {
        for (name, local_name) in bindings {
            let Some(methods) = patches.get(&local_name) else {
                continue;
            };
            let duplicate_lines = duplicated_patch_lines(methods);
            if duplicate_lines.len() < MIN_EXTENSIONS {
                continue;
            }
            findings.push(Finding {
                smell_name: SMELL_NAME.to_owned(),
                file_path: file.rel.clone(),
                class_name: name.clone(),
                line_numbers: duplicate_lines,
                relative_file_path: file.rel.clone(),
                additional_info: format!(
                    "repeated prototype implementation for imported type {} from {}",
                    name, module
                ),
            });
        }
    }
    findings
}

fn ecma_named_imports(file: &SourceFile) -> BTreeMap<String, Vec<(String, String)>> {
    let mut imports = BTreeMap::<String, Vec<(String, String)>>::new();
    for statement in descendants(file.root())
        .into_iter()
        .filter(|node| node.kind() == "import_statement")
    {
        let Some(source) = statement.child_by_field_name("source") else {
            continue;
        };
        let module = unquote(file.text(source)).to_owned();
        for specifier in descendants(statement)
            .into_iter()
            .filter(|node| node.kind() == "import_specifier")
        {
            let (Some(imported), Some(local)) = (
                specifier.child_by_field_name("name"),
                specifier
                    .child_by_field_name("alias")
                    .or_else(|| specifier.child_by_field_name("name")),
            ) else {
                continue;
            };
            imports
                .entry(module.clone())
                .or_default()
                .push((file.text(imported).to_owned(), file.text(local).to_owned()));
        }
    }
    imports
}

fn ecma_prototype_patches(file: &SourceFile) -> BTreeMap<String, BTreeMap<String, PrototypePatch>> {
    let mut patches = BTreeMap::<String, BTreeMap<String, PrototypePatch>>::new();
    for assignment in descendants(file.root())
        .into_iter()
        .filter(|node| node.kind() == "assignment_expression")
    {
        let (Some(left), Some(right)) = (
            assignment.child_by_field_name("left"),
            assignment.child_by_field_name("right"),
        ) else {
            continue;
        };
        if !matches!(right.kind(), "function_expression" | "arrow_function")
            || left.kind() != "member_expression"
        {
            continue;
        }
        let (Some(method), Some(prototype)) = (
            left.child_by_field_name("property"),
            left.child_by_field_name("object"),
        ) else {
            continue;
        };
        if prototype.kind() != "member_expression"
            || prototype
                .child_by_field_name("property")
                .is_none_or(|property| file.text(property) != "prototype")
        {
            continue;
        }
        let Some(receiver) = prototype.child_by_field_name("object") else {
            continue;
        };
        if receiver.kind() != "identifier" {
            continue;
        }
        patches
            .entry(file.text(receiver).to_owned())
            .or_default()
            .entry(file.text(method).to_owned())
            .or_insert_with(|| PrototypePatch {
                line: assignment.start_position().row + 1,
                shape: body_shape(right.child_by_field_name("body").unwrap_or(right)),
            });
    }
    patches
}

fn duplicated_patch_lines(patches: &BTreeMap<String, PrototypePatch>) -> Vec<usize> {
    let mut groups = BTreeMap::<Vec<String>, BTreeMap<String, usize>>::new();
    for (method, patch) in patches {
        if patch.shape.len() >= MIN_BODY_SHAPE_NODES {
            groups
                .entry(patch.shape.clone())
                .or_default()
                .insert(method.clone(), patch.line);
        }
    }
    let mut lines = groups
        .into_values()
        .filter(|methods| methods.len() >= MIN_EXTENSIONS)
        .flat_map(BTreeMap::into_values)
        .collect::<Vec<_>>();
    lines.sort_unstable();
    lines.dedup();
    lines
}

fn unquote(text: &str) -> &str {
    text.strip_prefix(['\'', '"', '`'])
        .and_then(|text| text.strip_suffix(['\'', '"', '`']))
        .unwrap_or(text)
}

fn body_shape(body: Node<'_>) -> Vec<String> {
    let mut shape = Vec::new();
    let mut stack = vec![body];
    while let Some(node) = stack.pop() {
        let kind = node.kind();
        if kind == "identifier" || kind.ends_with("_identifier") || kind.ends_with("_name") {
            continue;
        }
        if kind.ends_with("_literal") || matches!(kind, "true" | "false" | "null") {
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

fn imported_types(file: &SourceFile) -> BTreeSet<String> {
    descendants(file.root())
        .into_iter()
        .filter(|node| node.kind() == "import_declaration")
        .filter_map(|node| {
            let declaration = file.text(node).trim();
            if declaration.contains(".*") || declaration.starts_with("import static") {
                return None;
            }
            declaration
                .trim_start_matches("import")
                .trim()
                .trim_end_matches(';')
                .trim()
                .rsplit('.')
                .next()
                .map(str::to_owned)
        })
        .collect()
}

fn extension_parameter(source: SourceNode<'_>) -> Option<(String, String)> {
    if !is_static(source) {
        return None;
    }
    let parameters = source.node().child_by_field_name("parameters")?;
    let mut cursor = parameters.walk();
    let parameter = parameters
        .named_children(&mut cursor)
        .find(|node| node.kind() == "formal_parameter")?;
    let type_node = parameter.child_by_field_name("type")?;
    let name_node = parameter.child_by_field_name("name")?;
    let type_text = source.file().text(type_node);
    let type_name = type_text.rsplit('.').next()?.trim_end_matches("[]");
    Some((
        type_name.to_owned(),
        source.file().text(name_node).to_owned(),
    ))
}

fn is_static(source: SourceNode<'_>) -> bool {
    let mut cursor = source.node().walk();
    source
        .node()
        .named_children(&mut cursor)
        .find(|child| child.kind() == "modifiers")
        .is_some_and(|modifiers| {
            source
                .file()
                .text(modifiers)
                .split_whitespace()
                .any(|word| word == "static")
        })
}

fn extends_receiver(source: SourceNode<'_>, receiver_name: &str) -> bool {
    let Some(body) = source.node().child_by_field_name("body") else {
        return false;
    };
    descendants(body).into_iter().any(|node| {
        if node.kind() != "method_invocation" {
            return false;
        }
        node.child_by_field_name("object")
            .is_some_and(|object| source.file().text(object) == receiver_name)
    })
}

type ExtensionGroups = BTreeMap<(String, Vec<String>), BTreeMap<String, usize>>;

fn repeated_extension_owners(groups: ExtensionGroups) -> BTreeMap<(String, String), usize> {
    let mut findings = BTreeMap::<(String, String), usize>::new();
    for ((type_name, _), owners) in groups {
        if owners.len() < MIN_EXTENSIONS {
            continue;
        }
        for (owner, line) in owners {
            findings.entry((owner, type_name.clone())).or_insert(line);
        }
    }
    findings
}
