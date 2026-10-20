//! Best-effort Primitive Obsession heuristic based on typed fields.
//!
//! A named Java class, Rust struct, TypeScript class, or TSX class is reported
//! when it declares at least four fields with built-in primitive types. This
//! does not infer domain meaning from names, and skips untyped JavaScript.

use crate::finding::Finding;
use crate::lang::{SourceFile, model_for};
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
        .filter(|ty| model.supports_primitive_fields(ty.kind()))
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

fn primitive_field_count(file: &SourceFile, ty: Node<'_>) -> usize {
    let model = model_for(file.language);
    model
        .fields(ty)
        .into_iter()
        .filter_map(|field| model.field_type(field))
        .filter(|kind| model.is_primitive_type(file.text(*kind)))
        .count()
}
