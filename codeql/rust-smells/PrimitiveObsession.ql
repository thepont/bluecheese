/**
 * @name Primitive obsession
 * @description A Rust struct declares at least four primitive-typed fields.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/rust/primitive-obsession
 * @precision medium
 * @tags maintainability
 */
import codeql.rust.elements.Struct
import codeql.rust.elements.StructField

from Struct type
where
  count(StructField field |
    field.getParentNode().getParentNode() = type and
    field.getTypeRepr().toString() in
      ["u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64", "i128", "isize", "f32", "f64", "bool", "char"]
  ) >= 4
select type, "This struct declares at least four primitive-typed fields."
