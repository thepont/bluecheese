// @tests bluecheese/rust/shotgun-surgery
/** Test the Rust shotgun-surgery rule. */
import codeql.rust.elements.AssocItemList
import codeql.rust.elements.Function
import codeql.rust.elements.Impl
import codeql.rust.elements.MethodCallExpr

predicate methodInImpl(Function method, Impl owner) {
  exists(AssocItemList items |
    owner.getAssocItemList() = items and
    items.getAnAssocItem() = method
  )
}

predicate externalCaller(Function target, Function caller, Impl targetImpl, Impl callerImpl) {
  methodInImpl(target, targetImpl) and
  methodInImpl(caller, callerImpl) and
  callerImpl != targetImpl and
  exists(MethodCallExpr call |
    call.getStaticTarget() = target and
    call.getEnclosingCallable() = caller
  )
}

from Function target, Impl owner
where
  target.getFile().getRelativePath() = "ShotgunSurgery.rs" and
  methodInImpl(target, owner) and
  count(Function caller |
    exists(Impl callerImpl | externalCaller(target, caller, owner, callerImpl))
  ) >= 2 and
  count(Impl callerImpl |
    exists(Function caller | externalCaller(target, caller, owner, callerImpl))
  ) >= 2
select target, "This method is called by multiple methods across multiple impl blocks."
