/** Test BlueCheese's Rust data-clumps rule. */
import codeql.rust.elements.AssocItemList
import codeql.rust.elements.Function
import codeql.rust.elements.IdentPat
import codeql.rust.elements.Impl
import codeql.rust.elements.Param

predicate sameNamedParameter(Param left, Param right) {
  left.getPat().(IdentPat).getName().getText() =
    right.getPat().(IdentPat).getName().getText() and
  left.getTypeRepr().toString() = right.getTypeRepr().toString()
}

from Impl impl, AssocItemList items, Function method, Function other
where
  method.getFile().getRelativePath() = "DataClumps.rs" and
  impl.getAssocItemList() = items and
  items.getAnAssocItem() = method and
  items.getAnAssocItem() = other and
  method != other and
  count(Param parameter |
    parameter.getEnclosingCallable() = method and
    exists(Param matching |
      matching.getEnclosingCallable() = other and
      sameNamedParameter(parameter, matching)
    )
  ) >= 2
select method, "This method repeats a parameter group used by another method."
