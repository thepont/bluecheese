/**
 * @name Inappropriate intimacy
 * @description A Rust method directly reads a field declared on another source struct.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/rust/inappropriate-intimacy
 * @precision medium
 * @tags maintainability
 */
import codeql.rust.elements.AssocItemList
import codeql.rust.elements.FieldExpr
import codeql.rust.elements.Function
import codeql.rust.elements.Impl
import codeql.rust.elements.Struct
import codeql.rust.elements.StructField

predicate methodInImpl(Function method, Impl owner) {
  exists(AssocItemList items |
    owner.getAssocItemList() = items and
    items.getAnAssocItem() = method
  )
}

from Function method, Impl owner, FieldExpr access,
  Struct target, StructField field
where
  methodInImpl(method, owner) and
  access.getEnclosingCallable() = method and
  access.getStructField() = field and
  field.getParentNode().getParentNode() = target and
  not owner.getSelf().toString().matches("%" + target.getName().toString())
select access, "This method reads a field from another source struct."
