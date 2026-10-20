/**
 * @name Refused bequest
 * @description Reports Rust trait overrides that replace a default method with todo or unimplemented.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/rust/refused-bequest
 * @precision medium
 * @tags maintainability
 */
import codeql.rust.elements.AssocItemList
import codeql.rust.elements.Function
import codeql.rust.elements.Impl
import codeql.rust.elements.MacroCall
import codeql.rust.elements.Trait

from Impl impl, Trait trait, AssocItemList implItems, AssocItemList traitItems,
  Function method, Function defaultMethod
where
  impl.getTrait() = trait and
  impl.getAssocItemList() = implItems and
  trait.getAssocItemList() = traitItems and
  implItems.getAnAssocItem() = method and
  traitItems.getAnAssocItem() = defaultMethod and
  method.getName().toString() = defaultMethod.getName().toString() and
  exists(defaultMethod.getBody()) and
  exists(MacroCall refusal |
    refusal.getEnclosingCallable() = method and
    (
      refusal.getPath().toString() = "todo" or
      refusal.getPath().toString() = "unimplemented"
    )
  )
select method, "This trait implementation replaces a default method with a refusal stub."
