//! Best-effort Primitive Obsession heuristic based on typed fields.
//!
//! A named Java class, Rust struct, TypeScript class, or TSX class is reported
//! when it declares at least four fields with built-in primitive types. This
//! does not infer domain meaning from names, and skips untyped JavaScript.

use crate::finding::Finding;
use crate::lang::{LanguageId, SourceFile, model_for};
use tree_sitter::Node;

pub const SMELL_NAME: &str = "Primitive Obsession";
const MIN_PRIMITIVE_FIELDS: usize = 4;

pub fn detect(files: &[SourceFile]) -> Vec<Finding> {
    files.iter().flat_map(file_findings).collect()
}

fn file_findings(file: &SourceFile) -> Vec<Finding> {
    let model = model_for(file.language);
    model
        .types(file)
        .into_iter()
        .filter(|ty| is_supported_type(file.language, ty.kind()))
        .filter(|ty| primitive_field_count(file, *ty) >= MIN_PRIMITIVE_FIELDS)
        .filter_map(|ty| {
            Some(Finding {
                smell_name: SMELL_NAME.to_owned(),
                file_path: file.rel.clone(),
                class_name: model.type_name(file, ty)?.to_owned(),
                line_numbers: vec![model.decl_line(ty)],
                relative_file_path: file.rel.clone(),
                additional_info: String::new(),
            })
        })
        .collect()
}

fn is_supported_type(language: LanguageId, kind: &str) -> bool {
    match language {
        LanguageId::Java => kind == "class_declaration",
        LanguageId::Rust => kind == "struct_item",
        LanguageId::TypeScript | LanguageId::Tsx => {
            matches!(
                kind,
                "class_declaration" | "class" | "abstract_class_declaration"
            )
        }
        LanguageId::JavaScript => false,
    }
}

fn primitive_field_count(file: &SourceFile, ty: Node<'_>) -> usize {
    let Some(mut body) = ty.child_by_field_name("body") else {
        return 0;
    };
    if file.language == LanguageId::Rust && body.kind() != "field_declaration_list" {
        let Some(fields) = body
            .named_child(0)
            .filter(|child| child.kind() == "field_declaration_list")
        else {
            return 0;
        };
        body = fields;
    }
    let mut cursor = body.walk();
    body.named_children(&mut cursor)
        .filter(|field| is_field_kind(file.language, field.kind()))
        .filter_map(|field| field_type_text(file, field).map(|text| (field, text)))
        .filter(|(_, field_type)| is_primitive_type(file.language, field_type))
        .map(|(field, _)| {
            if file.language == LanguageId::Java && field.kind() == "field_declaration" {
                let mut cursor = field.walk();
                field
                    .named_children(&mut cursor)
                    .filter(|child| child.kind() == "variable_declarator")
                    .count()
            } else {
                1
            }
        })
        .sum()
}

fn is_field_kind(language: LanguageId, kind: &str) -> bool {
    match language {
        LanguageId::Java | LanguageId::Rust => kind == "field_declaration",
        LanguageId::TypeScript | LanguageId::Tsx => matches!(
            kind,
            "field_definition" | "public_field_definition" | "property_definition"
        ),
        LanguageId::JavaScript => false,
    }
}

fn field_type_text<'a>(file: &'a SourceFile, field: Node<'a>) -> Option<&'a str> {
    field.child_by_field_name("type").map(|ty| file.text(ty))
}

fn is_primitive_type(language: LanguageId, raw: &str) -> bool {
    let raw = raw.trim();
    let text = raw
        .trim_start_matches(':')
        .trim()
        .trim_end_matches('?')
        .trim();
    let base = text
        .split('<')
        .next()
        .unwrap_or(text)
        .trim_end_matches("[]");
    match language {
        LanguageId::Java => matches!(
            base,
            "byte" | "short" | "int" | "long" | "float" | "double" | "char" | "boolean"
        ),
        LanguageId::Rust => matches!(
            base,
            "u8" | "u16"
                | "u32"
                | "u64"
                | "u128"
                | "usize"
                | "i8"
                | "i16"
                | "i32"
                | "i64"
                | "i128"
                | "isize"
                | "f32"
                | "f64"
                | "bool"
                | "char"
        ),
        LanguageId::TypeScript | LanguageId::Tsx => {
            matches!(base, "number" | "string" | "boolean" | "bigint" | "symbol")
        }
        LanguageId::JavaScript => false,
    }
}
