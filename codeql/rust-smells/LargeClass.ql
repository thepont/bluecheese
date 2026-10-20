/**
 * @name Large class
 * @description Reports Rust structs with at least eight fields.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/rust/large-class
 * @precision medium
 * @tags maintainability
 */
import codeql.rust.elements.Struct
import codeql.rust.elements.StructField

from Struct type
where
  count(StructField field |
    field.getParentNode().getParentNode() = type
  ) >= 8
select type, "This struct declares at least eight fields."
