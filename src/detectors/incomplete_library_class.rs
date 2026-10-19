//! Best-effort Incomplete Library Class heuristic for Java.
//!
//! Without library documentation or usage intent, missing API surface cannot
//! be established from source alone. This detector reports a narrower proxy:
//! a class with multiple static helper methods that extend the same imported
//! type through calls on their first parameter.

use std::collections::{BTreeMap, BTreeSet};

use crate::finding::Finding;
use crate::lang::{LanguageId, LanguageModel, SourceFile, java};
use crate::walk::descendants;
use tree_sitter::Node;

pub const SMELL_NAME: &str = "Incomplete Library Class";
const MIN_EXTENSIONS: usize = 2;

pub fn detect(files: &[SourceFile]) -> Vec<Finding> {
    files
        .iter()
        .filter(|file| file.language == LanguageId::Java)
        .flat_map(file_findings)
        .collect()
}

fn file_findings(file: &SourceFile) -> Vec<Finding> {
    let imported = imported_types(file);
    let local = java::JAVA
        .types(file)
        .into_iter()
        .filter_map(|ty| java::JAVA.type_name(file, ty).map(str::to_owned))
        .collect::<BTreeSet<_>>();

    java::JAVA
        .types(file)
        .into_iter()
        .filter(|ty| ty.kind() == "class_declaration")
        .filter_map(|ty| {
            let mut extensions = BTreeMap::<String, usize>::new();
            for method in java::JAVA.methods(file, ty) {
                let Some((type_name, receiver)) = extension_parameter(file, method) else {
                    continue;
                };
                if imported.contains(&type_name)
                    && !local.contains(&type_name)
                    && extends_receiver(file, method, &receiver)
                {
                    *extensions.entry(type_name).or_default() += 1;
                }
            }

            let (type_name, count) = extensions
                .into_iter()
                .filter(|(_, count)| *count >= MIN_EXTENSIONS)
                .max_by_key(|(_, count)| *count)?;
            Some(Finding {
                smell_name: SMELL_NAME.to_owned(),
                file_path: file.rel.clone(),
                class_name: java::JAVA.type_name(file, ty)?.to_owned(),
                line_numbers: vec![java::JAVA.decl_line(ty)],
                relative_file_path: file.rel.clone(),
                additional_info: format!(
                    "{} helper methods extend imported type {}",
                    count, type_name
                ),
            })
        })
        .collect()
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
