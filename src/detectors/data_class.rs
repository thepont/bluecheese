//! Port of StaticNose's strict Data Class rule.

use crate::finding::Finding;
use crate::lang::{LanguageId, SourceFile, model_for};
use tree_sitter::Node;

pub const SMELL_NAME: &str = "Data Class";

pub fn detect(files: &[SourceFile]) -> Vec<Finding> {
    files
        .iter()
        .filter(|file| file.language == LanguageId::Java)
        .flat_map(file_findings)
        .collect()
}

fn file_findings(file: &SourceFile) -> Vec<Finding> {
    let model = model_for(LanguageId::Java);
    model
        .types(file)
        .into_iter()
        .filter_map(|ty| {
            let name = model.type_name(file, ty)?;
            if ty.kind() == "record_declaration" {
                return (ty
                    .child_by_field_name("parameters")
                    .is_some_and(|p| p.named_child_count() > 0))
                .then(|| Finding {
                    smell_name: SMELL_NAME.to_string(),
                    file_path: file.rel.clone(),
                    class_name: name.to_string(),
                    line_numbers: vec![model.decl_line(ty)],
                    relative_file_path: file.rel.clone(),
                    additional_info: String::new(),
                });
            }
            if ty.kind() != "class_declaration" || has_modifier(file, ty, "abstract") {
                return None;
            }

            let constructors = model.constructors(file, ty);
            if !constructors.is_empty()
                && constructors
                    .iter()
                    .all(|ctor| has_modifier(file, *ctor, "private"))
            {
                return None;
            }

            let fields = fields(ty);
            let methods = model.methods(file, ty);
            let public_mutable_fields = fields
                .iter()
                .filter(|field| {
                    let Some(declaration) = field.parent() else {
                        return false;
                    };
                    has_modifier(file, declaration, "public")
                        && !has_modifier(file, declaration, "static")
                        && !has_modifier(file, declaration, "final")
                })
                .count();

            let smell = if methods.is_empty() {
                public_mutable_fields > 0
            } else {
                methods
                    .iter()
                    .all(|method| is_strict_accessor(file, *method, &fields))
            };
            smell.then(|| Finding {
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

fn is_strict_accessor(file: &SourceFile, method: Node<'_>, fields: &[Node<'_>]) -> bool {
    let Some(body) = method.child_by_field_name("body") else {
        return false;
    };
    let mut cursor = body.walk();
    let statements: Vec<_> = body
        .named_children(&mut cursor)
        .filter(|node| node.kind() != "line_comment" && node.kind() != "block_comment")
        .collect();
    if statements.len() != 1 {
        return false;
    }
    let statement = statements[0];

    let Some(result_type) = method.child_by_field_name("type") else {
        return false;
    };
    let is_void = file.text(result_type) == "void";
    let Some(parameters) = method.child_by_field_name("parameters") else {
        return false;
    };
    let mut cursor = parameters.walk();
    let parameter_names: Vec<_> = parameters
        .named_children(&mut cursor)
        .filter_map(|parameter| {
            parameter
                .child_by_field_name("name")
                .map(|name| file.text(name))
        })
        .collect();

    if parameter_names.is_empty() && !is_void && statement.kind() == "return_statement" {
        let mut cursor = statement.walk();
        return statement
            .named_children(&mut cursor)
            .find(|n| n.kind() != "line_comment" && n.kind() != "block_comment")
            .is_some_and(|expression| field_reference(file, expression, fields));
    }

    if parameter_names.len() == 1 && is_void && statement.kind() == "expression_statement" {
        let mut cursor = statement.walk();
        let Some(assignment) = statement
            .named_children(&mut cursor)
            .find(|n| n.kind() == "assignment_expression")
        else {
            return false;
        };
        let Some(left) = assignment.child_by_field_name("left") else {
            return false;
        };
        let Some(right) = assignment.child_by_field_name("right") else {
            return false;
        };
        return field_reference(file, left, fields)
            && right.kind() == "identifier"
            && file.text(right) == parameter_names[0];
    }
    false
}

fn field_reference(file: &SourceFile, expression: Node<'_>, fields: &[Node<'_>]) -> bool {
    let field_name = if expression.kind() == "identifier" {
        file.text(expression)
    } else if expression.kind() == "field_access" {
        let Some(field) = expression.child_by_field_name("field") else {
            return false;
        };
        file.text(field)
    } else {
        return false;
    };
    fields.iter().any(|field| {
        field
            .child_by_field_name("name")
            .is_some_and(|name| file.text(name) == field_name)
    })
}

fn has_modifier(file: &SourceFile, node: Node<'_>, modifier: &str) -> bool {
    let Some(modifiers) = node.child_by_field_name("modifiers") else {
        let mut cursor = node.walk();
        let Some(modifiers) = node
            .named_children(&mut cursor)
            .find(|child| child.kind() == "modifiers")
        else {
            return false;
        };
        return file
            .text(modifiers)
            .split_whitespace()
            .any(|word| word == modifier);
    };
    file.text(modifiers)
        .split_whitespace()
        .any(|word| word == modifier)
}
