//! Port of StaticNose's strict Data Class rule.

use crate::finding::Finding;
use crate::lang::{LanguageId, SourceFile, model_for};
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

            let fields = model.fields(ty);
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

            let accessors = Accessors {
                file,
                fields: &fields,
            };
            let smell = if methods.is_empty() {
                public_mutable_fields > 0
            } else {
                methods
                    .iter()
                    .all(|method| accessors.is_strict_accessor(*method))
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
            model
                .fields(ty)
                .into_iter()
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
            model
                .fields(ty)
                .into_iter()
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
    let accessors = Accessors {
        file,
        fields: &fields,
    };
    if !methods.is_empty()
        && !methods
            .iter()
            .all(|method| accessors.is_non_java_accessor(*method))
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

struct Accessors<'a, T> {
    file: &'a SourceFile,
    fields: &'a [T],
}

impl Accessors<'_, String> {
    fn is_non_java_accessor(&self, method: Node<'_>) -> bool {
        let Some(statements) = model_for(self.file.language).body_children(method) else {
            return false;
        };
        if statements.len() > 1 {
            return false;
        }
        let Some(statement) = statements.first().copied() else {
            return false;
        };
        let Some(function_name) = method
            .child_by_field_name("name")
            .map(|n| self.file.text(n))
        else {
            return false;
        };
        let statement_text = self.file.text(statement).replace(' ', "");
        let is_rust = self.file.language == LanguageId::Rust;
        let receiver_name = if is_rust { "self" } else { "this" };
        if function_name.starts_with("get") || function_name.starts_with("is") {
            return self.fields.iter().any(|field| {
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
        let Some(parameter_name) = self.single_accessor_parameter(method) else {
            return false;
        };
        let assignment = statement_text.trim_end_matches(';');
        self.fields
            .iter()
            .any(|field| assignment == format!("{receiver_name}.{field}={parameter_name}"))
    }

    fn single_accessor_parameter(&self, method: Node<'_>) -> Option<String> {
        let parameters = method.child_by_field_name("parameters")?;
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
            .map(|parameter| self.file.text(parameter).to_owned())
            .collect();
        let mut names = parameter_names.into_iter();
        let name = names.next()?;
        names.next().is_none().then_some(name)
    }
}

impl Accessors<'_, Node<'_>> {
    fn is_strict_accessor(&self, method: Node<'_>) -> bool {
        let Some(statements) = model_for(self.file.language).body_statements(method) else {
            return false;
        };
        if statements.len() != 1 {
            return false;
        }
        let statement = statements[0];

        let Some(result_type) = method.child_by_field_name("type") else {
            return false;
        };
        let is_void = self.file.text(result_type) == "void";
        let Some(parameters) = method.child_by_field_name("parameters") else {
            return false;
        };
        let mut cursor = parameters.walk();
        let parameter_names: Vec<_> = parameters
            .named_children(&mut cursor)
            .filter_map(|parameter| {
                parameter
                    .child_by_field_name("name")
                    .map(|name| self.file.text(name))
            })
            .collect();

        if parameter_names.is_empty() && !is_void && statement.kind() == "return_statement" {
            let mut cursor = statement.walk();
            return statement
                .named_children(&mut cursor)
                .find(|n| n.kind() != "line_comment" && n.kind() != "block_comment")
                .is_some_and(|expression| self.field_reference(expression));
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
            return self.field_reference(left)
                && right.kind() == "identifier"
                && self.file.text(right) == parameter_names[0];
        }
        false
    }

    fn field_reference(&self, expression: Node<'_>) -> bool {
        let field_name = if expression.kind() == "identifier" {
            self.file.text(expression)
        } else if expression.kind() == "field_access" {
            let Some(field) = expression.child_by_field_name("field") else {
                return false;
            };
            self.file.text(field)
        } else {
            return false;
        };
        self.fields.iter().any(|field| {
            field
                .child_by_field_name("name")
                .is_some_and(|name| self.file.text(name) == field_name)
        })
    }
}
