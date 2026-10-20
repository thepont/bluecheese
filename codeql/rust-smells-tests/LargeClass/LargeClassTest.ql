/** Test the Rust large-class field-count rule. */
import codeql.rust.elements.Struct
import codeql.rust.elements.StructField

from Struct type
where
  type.getFile().getRelativePath() = "LargeClass.rs" and
  count(StructField field |
    field.getParentNode().getParentNode() = type
  ) >= 8
select type, "This struct declares at least eight fields."
