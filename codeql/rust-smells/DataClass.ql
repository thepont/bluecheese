/**
 * @name Data class
 * @description A Rust struct exposes public fields.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/rust/data-class
 * @precision medium
 * @tags maintainability
 */
import codeql.rust.elements.Struct
import codeql.rust.elements.StructField

from Struct type
where
  exists(StructField field |
    field.getParentNode().getParentNode() = type and
    field.getVisibility().toString().regexpMatch("^pub.*")
  )
select type, "This struct exposes public data fields."
