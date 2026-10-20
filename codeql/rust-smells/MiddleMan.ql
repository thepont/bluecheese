/**
 * @name Middle man
 * @description Reports Rust impls where at least half of their methods are single-call delegations through source-defined struct fields.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/rust/middle-man
 * @precision medium
 * @tags maintainability
 */
import rust
import codeql.rust.elements.AssocItemList
import codeql.rust.elements.FieldExpr
import codeql.rust.elements.Function
import codeql.rust.elements.Impl
import codeql.rust.elements.MethodCallExpr
import codeql.rust.elements.Struct
import codeql.rust.elements.StructField

predicate methodInImpl(Function method, Impl owner) {
  exists(AssocItemList items |
    owner.getAssocItemList() = items and
    items.getAnAssocItem() = method
  )
}

predicate delegatesThroughProjectField(Function method) {
  count(MethodCallExpr call | call.getEnclosingCallable() = method) = 1 and
  exists(MethodCallExpr call, FieldExpr receiver, StructField field, Struct target |
    call.getEnclosingCallable() = method and
    call.getReceiver() = receiver and
    receiver.getStructField() = field and
    field.getTypeRepr().toString().regexpMatch(".*" + target.getName().toString() + ".*")
  )
}

from Impl owner
where
  count(Function method | methodInImpl(method, owner)) >= 2 and
  count(Function method |
    methodInImpl(method, owner) and
    delegatesThroughProjectField(method)
  ) >= 2 and
  2 * count(Function method |
    methodInImpl(method, owner) and
    delegatesThroughProjectField(method)
  ) >= count(Function method | methodInImpl(method, owner))
select owner, "At least half of this impl's methods delegate through a source-defined field."
