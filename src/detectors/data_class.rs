//! Port of StaticNose's strict Data Class rule.

use crate::finding::Finding;
use crate::lang::{model_for, LanguageId, SourceFile};
use tree_sitter::Node;

pub const SMELL_NAME: &str = "Data Class";

pub fn detect(files: &[SourceFile]) -> Vec<Finding> {
    files.iter().flat_map(file_findings).collect()
}

fn file_findings(file: &SourceFile) -> Vec<Finding> {
    let model = model_for(file.language);
    model
        .types(file)
        .into_iter()
        .filter_map(|ty| {
            let name = model.type_name(file, ty)?;
            if file.language != LanguageId::Java {
                return non_java_data_class(file, ty, name);
            }
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

fn non_java_data_class(file: &SourceFile, ty: Node<'_>, name: &str) -> Option<Finding> {
    let model = model_for(file.language);
    let fields = match file.language {
        LanguageId::Rust => {
            if ty.kind() == "source_file" {
                return None;
            }
            if !matches!(ty.kind(), "struct_item" | "enum_item") {
                return None;
            }
            let body = ty.child_by_field_name("body")?;
            let mut cursor = body.walk();
            body.named_children(&mut cursor)
                .filter(|field| field.kind() == "field_declaration")
                .filter(|field| file.text(*field).trim_start().starts_with("pub "))
                .filter_map(|field| {
                    field
                        .child_by_field_name("name")
                        .map(|name| file.text(name).to_owned())
                })
                .collect::<Vec<_>>()
        }
        LanguageId::JavaScript | LanguageId::TypeScript | LanguageId::Tsx => {
            if !matches!(
                ty.kind(),
                "class_declaration" | "class" | "abstract_class_declaration"
            ) {
                return None;
            }
            let body = ty.child_by_field_name("body")?;
            let mut cursor = body.walk();
            body.named_children(&mut cursor)
                .filter(|field| {
                    matches!(
                        field.kind(),
                        "public_field_definition" | "field_definition" | "property_definition"
                    )
                })
                .filter(|field| {
                    !has_non_java_modifier(file.text(*field), "private")
                        && !has_non_java_modifier(file.text(*field), "protected")
                        && !has_non_java_modifier(file.text(*field), "static")
                })
                .filter_map(|field| {
                    field
                        .child_by_field_name("name")
                        .or_else(|| field.child_by_field_name("property"))
                        .map(|name| file.text(name).trim_start_matches('#').to_owned())
                })
                .collect::<Vec<_>>()
        }
        LanguageId::Java => return None,
    };
    if fields.is_empty() {
        return None;
    }
    let methods = model.methods(file, ty);
    if !methods.is_empty()
        && !methods
            .iter()
            .all(|method| is_non_java_accessor(file, *method, &fields))
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
}

fn has_non_java_modifier(text: &str, modifier: &str) -> bool {
    text.split(|ch: char| !ch.is_alphanumeric() && ch != '_')
        .any(|word| word == modifier)
}

fn is_non_java_accessor(file: &SourceFile, method: Node<'_>, fields: &[String]) -> bool {
    let Some(body) = method.child_by_field_name("body") else {
        return false;
    };
    let mut cursor = body.walk();
    let statements = body.named_children(&mut cursor).collect::<Vec<_>>();
    if statements.len() > 1 {
        return false;
    }
    let Some(statement) = statements.first().copied() else {
        return false;
    };
    let Some(function_name) = method.child_by_field_name("name").map(|n| file.text(n)) else {
        return false;
    };
    let statement_text = file.text(statement).replace(' ', "");
    let is_rust = file.language == LanguageId::Rust;
    let receiver_name = if is_rust { "self" } else { "this" };
    if function_name.starts_with("get") || function_name.starts_with("is") {
        return fields.iter().any(|field| {
            let expression = format!("{receiver_name}.{field}");
            if is_rust {
                statement_text.trim_end_matches(';') == expression
            } else {
                statement_text == format!("return{expression};")
                    || statement_text == format!("return{expression}")
            }
        });
    }
    if !(function_name.starts_with("set_") || (!is_rust && function_name.starts_with("set"))) {
        return false;
    }
    let Some(parameters) = method.child_by_field_name("parameters") else {
        return false;
    };
    let mut cursor = parameters.walk();
    let parameter_names: Vec<_> = parameters
        .named_children(&mut cursor)
        .filter(|parameter| parameter.kind() != "self_parameter")
        .filter_map(|parameter| {
            parameter
                .child_by_field_name("name")
                .or_else(|| parameter.child_by_field_name("pattern"))
                .or_else(|| (parameter.kind() == "identifier").then_some(parameter))
        })
        .map(|parameter| file.text(parameter).to_owned())
        .collect();
    if parameter_names.len() != 1 {
        return false;
    }
    let assignment = statement_text.trim_end_matches(';');
    fields
        .iter()
        .any(|field| assignment == format!("{receiver_name}.{field}={}", parameter_names[0]))
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
