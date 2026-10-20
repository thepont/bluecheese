/**
 * @name Message chains
 * @description A Rust function contains at least three nested getter calls.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/rust/message-chains
 * @precision medium
 * @tags maintainability
 */
import codeql.rust.elements.MethodCallExpr

predicate isAccessor(MethodCallExpr call) {
  call.getIdentifier().getText().regexpMatch("^(get|is).*")
}

from MethodCallExpr outer, MethodCallExpr middle, MethodCallExpr inner
where
  outer.getReceiver() = middle and
  middle.getReceiver() = inner and
  isAccessor(outer) and
  isAccessor(middle) and
  isAccessor(inner)
select outer, "This function contains a chain of at least three getter calls."
