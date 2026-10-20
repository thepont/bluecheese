/** Test BlueCheese's JavaScript and TypeScript message-chains rule. */
import javascript

predicate isAccessor(MethodCallExpr call) {
  call.getMethodName().regexpMatch("^(get|is|has).*")
}

from MethodCallExpr outer, MethodCallExpr middle, MethodCallExpr inner
where
  outer.getFile().getRelativePath() = "MessageChains.ts" and
  outer.getReceiver() = middle and
  middle.getReceiver() = inner and
  isAccessor(outer) and
  isAccessor(middle) and
  isAccessor(inner)
select outer, "This function contains a chain of at least three accessor calls."
