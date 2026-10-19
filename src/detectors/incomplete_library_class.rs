//! Best-effort Incomplete Library Class heuristics for Java, Rust, JavaScript and TypeScript.
//!
//! Without library documentation or usage intent, missing API surface cannot
//! be established from source alone. Findings therefore require repeated,
//! non-trivial implementations around the same imported type.

use std::collections::{BTreeMap, BTreeSet};

use crate::finding::Finding;
use crate::lang::{LanguageId, LanguageModel, SourceFile, java, rust};
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
        .filter_map(|ty| java::JAVA.type_name(file, ty).map(str::to_owned))
        .collect::<BTreeSet<_>>();

    let mut groups = BTreeMap::<(String, Vec<String>), BTreeMap<String, usize>>::new();
    for ty in java::JAVA
        .types(file)
        .into_iter()
        .filter(|ty| ty.kind() == "class_declaration")
    {
        let Some(owner_name) = java::JAVA.type_name(file, ty).map(str::to_owned) else {
            continue;
        };
        for method in java::JAVA.methods(file, ty) {
            let (Some((type_name, receiver)), Some(body)) = (
                extension_parameter(file, method),
                method.child_by_field_name("body"),
            ) else {
                continue;
            };
            if !imported.contains(&type_name)
                || local.contains(&type_name)
                || !extends_receiver(file, method, &receiver)
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
        .collect()
}

fn rust_findings(file: &SourceFile) -> Vec<Finding> {
    let imported = rust_imported_types(file);
    let declarations = rust::RUST.types(file);
    let local_types = declarations
        .iter()
        .filter_map(|ty| rust::RUST.type_name(file, *ty).map(str::to_owned))
        .collect::<BTreeSet<_>>();
    let local_traits = declarations
        .iter()
        .filter(|ty| ty.kind() == "trait_item")
        .filter_map(|ty| rust::RUST.type_name(file, *ty).map(str::to_owned))
        .collect::<BTreeSet<_>>();

    let mut groups = BTreeMap::<(String, Vec<String>), BTreeMap<String, usize>>::new();
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
            if !rust_extension_method(file, method) {
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

    let mut findings = BTreeMap::<(String, String), usize>::new();
    for ((target_name, _), traits) in groups {
        if traits.len() < MIN_EXTENSIONS {
            continue;
        }
        for (trait_name, line) in traits {
            findings
                .entry((trait_name, target_name.clone()))
                .or_insert(line);
        }
    }
    findings
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
        .collect()
}

fn rust_imported_types(file: &SourceFile) -> BTreeSet<String> {
    descendants(file.root())
        .into_iter()
        .filter(|node| node.kind() == "use_declaration")
        .filter_map(|node| node.child_by_field_name("argument"))
        .filter_map(|argument| {
            let path = file.text(argument).trim();
            if path.contains('*') || path.contains(',') {
                return None;
            }
            path.rsplit("::")
                .next()
                .map(|leaf| leaf.trim_matches(|c: char| !c.is_alphanumeric() && c != '_'))
                .filter(|leaf| !leaf.is_empty())
                .map(str::to_owned)
        })
        .collect()
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

fn rust_extension_method(file: &SourceFile, method: Node<'_>) -> bool {
    let Some(body) = method.child_by_field_name("body") else {
        return false;
    };
    descendants(body).into_iter().any(|node| {
        node.kind() == "call_expression"
            && node
                .child_by_field_name("function")
                .is_some_and(|function| file.text(function).trim_start().starts_with("self."))
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

fn extension_parameter(file: &SourceFile, method: Node<'_>) -> Option<(String, String)> {
    if !is_static(file, method) {
        return None;
    }
    let parameters = method.child_by_field_name("parameters")?;
    let mut cursor = parameters.walk();
    let parameter = parameters
        .named_children(&mut cursor)
        .find(|node| node.kind() == "formal_parameter")?;
    let type_node = parameter.child_by_field_name("type")?;
    let name_node = parameter.child_by_field_name("name")?;
    let type_text = file.text(type_node);
    let type_name = type_text.rsplit('.').next()?.trim_end_matches("[]");
    Some((type_name.to_owned(), file.text(name_node).to_owned()))
}

fn is_static(file: &SourceFile, method: Node<'_>) -> bool {
    let mut cursor = method.walk();
    method
        .named_children(&mut cursor)
        .find(|child| child.kind() == "modifiers")
        .is_some_and(|modifiers| {
            file.text(modifiers)
                .split_whitespace()
                .any(|word| word == "static")
        })
}

fn extends_receiver(file: &SourceFile, method: Node<'_>, receiver_name: &str) -> bool {
    let Some(body) = method.child_by_field_name("body") else {
        return false;
    };
    descendants(body).into_iter().any(|node| {
        if node.kind() != "method_invocation" {
            return false;
        }
        node.child_by_field_name("object")
            .is_some_and(|object| file.text(object) == receiver_name)
    })
}
