//! Best-effort Temporary Field heuristic for Java.
//!
//! A class field is a candidate when every read found in its methods is nested
//! inside an `if` consequence. Writes alone do not count as uses. This identifies
//! state used only for one conditional workflow; runtime call sequences are not
//! inferred.

use std::collections::BTreeSet;

use crate::finding::Finding;
use crate::lang::{LanguageId, LanguageModel, SourceFile, java};
use crate::walk::descendants;
use tree_sitter::Node;

pub const SMELL_NAME: &str = "Temporary Field";

struct DeclaredField<'a> {
    node: Node<'a>,
    name: String,
    primitive: bool,
}

pub fn detect(files: &[SourceFile]) -> Vec<Finding> {
    files
        .iter()
        .filter(|file| file.language == LanguageId::Java)
        .flat_map(file_findings)
        .collect()
}

fn file_findings(file: &SourceFile) -> Vec<Finding> {
    java::JAVA
        .types(file)
        .into_iter()
        .filter(|ty| ty.kind() == "class_declaration")
        .filter_map(|ty| {
            let methods = java::JAVA.methods(file, ty);
            let constructors = java::JAVA.constructors(file, ty);
            let declared_fields = fields(file, ty);
            let usage = FieldUsage {
                file,
                methods: &methods,
            };
            let mut lines = declared_fields
                .iter()
                .filter(|field| {
                    usage.is_nullable_guarded(field)
                        || (field.primitive
                            && usage.references_only_in_branches(&field.name)
                            && !usage.has_unconditional_write(&field.name)
                            && !usage.is_created_as_branch_local_object(&field.name))
                })
                .map(|field| file.line(field.node))
                .collect::<BTreeSet<_>>();
            if lines.is_empty() && constructors.len() >= 2 {
                lines.extend(
                    usage
                        .rarely_used_fields(&declared_fields)
                        .into_iter()
                        .map(|field| file.line(field)),
                );
            }
            (!lines.is_empty()).then(|| Finding {
                smell_name: SMELL_NAME.to_owned(),
                file_path: file.rel.clone(),
                class_name: java::JAVA
                    .type_name(file.at(ty))
                    .unwrap_or("<anonymous>")
                    .to_owned(),
                line_numbers: lines.into_iter().collect(),
                relative_file_path: file.rel.clone(),
                additional_info: String::new(),
            })
        })
        .collect()
}

fn fields<'a>(file: &SourceFile, class: Node<'a>) -> Vec<DeclaredField<'a>> {
    let Some(body) = class.child_by_field_name("body") else {
        return Vec::new();
    };
    let mut cursor = body.walk();
    body.named_children(&mut cursor)
        .filter(|child| child.kind() == "field_declaration")
        .flat_map(|declaration| {
            let mut cursor = declaration.walk();
            declaration
                .named_children(&mut cursor)
                .filter(|child| child.kind() == "variable_declarator")
                .filter_map(|field| {
                    let (Some(name), Some(ty)) = (
                        field.child_by_field_name("name"),
                        declaration.child_by_field_name("type"),
                    ) else {
                        return None;
                    };
                    Some((field, name, ty))
                })
                .collect::<Vec<_>>()
        })
        .map(|(field, name, ty)| {
            let spelling = file.text(ty).trim();
            let base = spelling.trim_end_matches("[]");
            DeclaredField {
                node: field,
                name: file.text(name).to_owned(),
                primitive: matches!(
                    base,
                    "byte" | "short" | "int" | "long" | "float" | "double" | "char" | "boolean"
                ),
            }
        })
        .collect()
}

fn is_write_target(node: Node<'_>) -> bool {
    assignment_for(node).is_some()
}

fn assignment_for(node: Node<'_>) -> Option<Node<'_>> {
    let field = if node
        .parent()
        .is_some_and(|parent| parent.kind() == "field_access")
    {
        node.parent().expect("checked above")
    } else {
        node
    };
    field.parent().filter(|parent| {
        parent.kind() == "assignment_expression"
            && parent.child_by_field_name("left").is_some_and(|left| {
                left.start_byte() <= field.start_byte() && left.end_byte() >= field.end_byte()
            })
    })
}

fn inside_if_consequence(mut node: Node<'_>, method: Node<'_>) -> bool {
    while let Some(parent) = node.parent() {
        if parent.kind() == "if_statement"
            && parent
                .child_by_field_name("consequence")
                .is_some_and(|consequence| {
                    consequence.start_byte() <= node.start_byte()
                        && consequence.end_byte() >= node.end_byte()
                })
        {
            return true;
        }
        if parent.id() == method.id() {
            break;
        }
        node = parent;
    }
    false
}

struct FieldUsage<'a> {
    file: &'a SourceFile,
    methods: &'a [Node<'a>],
}
impl<'a> FieldUsage<'a> {
    fn references_only_in_branches(&self, name: &str) -> bool {
        let mut saw_reference = false;
        for method in self.methods {
            let Some(body) = method.child_by_field_name("body") else {
                continue;
            };
            for reference in descendants(body)
                .into_iter()
                .filter(|node| node.kind() == "identifier" && self.file.text(*node) == name)
            {
                if is_write_target(reference) {
                    continue;
                }
                saw_reference = true;
                if !inside_if_consequence(reference, *method) {
                    return false;
                }
            }
        }
        saw_reference
    }

    fn has_unconditional_write(&self, name: &str) -> bool {
        self.methods.iter().any(|method| {
            descendants(*method).into_iter().any(|node| {
                node.kind() == "identifier"
                    && self.file.text(node) == name
                    && is_write_target(node)
                    && !inside_if_consequence(node, *method)
            })
        })
    }

    fn is_created_as_branch_local_object(&self, name: &str) -> bool {
        self.methods.iter().any(|method| {
            descendants(*method).into_iter().any(|node| {
                if node.kind() != "identifier"
                    || self.file.text(node) != name
                    || !is_write_target(node)
                {
                    return false;
                }
                let Some(assignment) = assignment_for(node) else {
                    return false;
                };
                let Some(value) = assignment.child_by_field_name("right") else {
                    return false;
                };
                inside_if_consequence(node, *method)
                    && descendants(value)
                        .into_iter()
                        .any(|child| child.kind() == "object_creation_expression")
            })
        })
    }

    fn is_nullable_guarded(&self, field: &DeclaredField<'_>) -> bool {
        let name = field.name.as_str();
        if field.node.child_by_field_name("value").is_some() {
            return false;
        }
        self.methods.iter().any(|method| {
            descendants(*method)
                .into_iter()
                .filter(|node| node.kind() == "if_statement")
                .any(|branch| {
                    let (Some(condition), Some(consequence)) = (
                        branch.child_by_field_name("condition"),
                        branch.child_by_field_name("consequence"),
                    ) else {
                        return false;
                    };
                    let condition_nodes = descendants(condition);
                    condition_nodes
                        .iter()
                        .any(|node| node.kind() == "identifier" && self.file.text(*node) == name)
                        && condition_nodes
                            .iter()
                            .any(|node| matches!(node.kind(), "null_literal" | "null"))
                        && descendants(consequence).iter().any(|node| {
                            node.kind() == "identifier" && self.file.text(*node) == name
                        })
                })
        })
    }

    fn rarely_used_fields(&self, fields: &[DeclaredField<'a>]) -> Vec<Node<'a>> {
        let use_counts = fields
            .iter()
            .map(|field| {
                let name = field.name.as_str();
                let methods_using_field = self
                    .methods
                    .iter()
                    .filter(|method| {
                        descendants(**method)
                            .into_iter()
                            .any(|node| node.kind() == "identifier" && self.file.text(node) == name)
                    })
                    .count();
                (name, methods_using_field)
            })
            .collect::<Vec<_>>();
        let common = use_counts
            .iter()
            .map(|(_, count)| *count)
            .max()
            .unwrap_or(0);
        if common < 2 {
            return Vec::new();
        }
        fields
            .iter()
            .filter(|field| {
                field.primitive
                    && use_counts
                        .iter()
                        .find(|(field_name, _)| *field_name == field.name.as_str())
                        .is_some_and(|(_, count)| *count > 0 && *count < common)
            })
            .map(|field| field.node)
            .collect()
    }
}
