// @tests bluecheese/rust/data-class
/** Test BlueCheese's Rust public-data-class rule. */
import codeql.rust.elements.Struct
import codeql.rust.elements.StructField

from Struct type
where
  type.getFile().getRelativePath() = "DataClass.rs" and
  exists(StructField field |
    field.getParentNode().getParentNode() = type and
    field.getVisibility().toString().regexpMatch("^pub.*")
  )
select type, "This struct exposes public data fields."
