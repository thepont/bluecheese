/**
 * @name Message chains
 * @description A JavaScript or TypeScript function contains at least three nested accessor calls.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/javascript/message-chains
 * @precision medium
 * @tags maintainability
 */
import javascript

predicate isAccessor(MethodCallExpr call) {
  call.getMethodName().regexpMatch("^(get|is|has).*")
}

from MethodCallExpr outer, MethodCallExpr middle, MethodCallExpr inner
where
  outer.getReceiver() = middle and
  middle.getReceiver() = inner and
  isAccessor(outer) and
  isAccessor(middle) and
  isAccessor(inner)
select outer, "This function contains a chain of at least three accessor calls."
