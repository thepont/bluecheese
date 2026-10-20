/**
 * @name Message chains
 * @description A Java method contains at least three nested getter calls.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/java/message-chains
 * @precision medium
 * @tags maintainability
 */
import java

predicate isAccessor(MethodCall call) {
  call.getMethod().getName().regexpMatch("^(get|is).*")
}

from Method method, MethodCall outer, MethodCall middle, MethodCall inner
where
  outer.getEnclosingCallable() = method and
  outer.getQualifier() = middle and
  middle.getQualifier() = inner and
  isAccessor(outer) and
  isAccessor(middle) and
  isAccessor(inner)
select outer, "This method contains a chain of at least three getter calls."
