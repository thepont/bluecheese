/** Test BlueCheese's Rust primitive-obsession rule. */
import codeql.rust.elements.Struct
import codeql.rust.elements.StructField

from Struct type
where
  type.getFile().getRelativePath() = "PrimitiveObsession.rs" and
  count(StructField field |
    field.getParentNode().getParentNode() = type and
    field.getTypeRepr().toString() in
      ["u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64", "i128", "isize", "f32", "f64", "bool", "char"]
  ) >= 4
select type, "This struct declares at least four primitive-typed fields."
