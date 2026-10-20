/** Test BlueCheese's Java message-chains rule. */
import java

predicate isAccessor(MethodCall call) {
  call.getMethod().getName().regexpMatch("^(get|is).*")
}

from Method method, MethodCall outer, MethodCall middle, MethodCall inner
where
  method.getFile().getRelativePath() = "MessageChains.java" and
  outer.getEnclosingCallable() = method and
  outer.getQualifier() = middle and
  middle.getQualifier() = inner and
  isAccessor(outer) and
  isAccessor(middle) and
  isAccessor(inner)
select outer, "This method contains a chain of at least three getter calls."
