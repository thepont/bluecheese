/**
 * @name Shotgun surgery
 * @description Reports Java methods called from at least two methods in at least two other classes.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/java/shotgun-surgery
 * @precision medium
 * @tags maintainability
 */
import java

predicate hasExternalCaller(Method target, Method caller) {
  exists(MethodCall call |
    call.getMethod() = target and
    call.getEnclosingCallable() = caller and
    caller.getDeclaringType() != target.getDeclaringType()
  )
}

from Method target, Class owner
where
  target.getDeclaringType() = owner and
  owner.fromSource() and
  exists(target.getBody()) and
  count(Method caller | hasExternalCaller(target, caller)) >= 2 and
  count(Class callerClass |
    exists(Method caller |
      hasExternalCaller(target, caller) and
      caller.getDeclaringType() = callerClass
    )
  ) >= 2
select target, "This method is called by multiple methods across multiple classes."
