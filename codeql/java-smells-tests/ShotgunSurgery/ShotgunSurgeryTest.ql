// @tests bluecheese/java/shotgun-surgery
/** Test BlueCheese's Java Shotgun Surgery coupling rule. */
import java

predicate hasExternalCallerForTest(Method target, Method caller) {
  exists(MethodCall call |
    call.getMethod() = target and
    call.getEnclosingCallable() = caller and
    caller.getDeclaringType() != target.getDeclaringType()
  )
}

from Method target, Class owner
where
  target.getFile().getRelativePath() = "ShotgunSurgery.java" and
  target.getDeclaringType() = owner and
  exists(target.getBody()) and
  count(Method caller | hasExternalCallerForTest(target, caller)) >= 2 and
  count(Class callerClass |
    exists(Method caller |
      hasExternalCallerForTest(target, caller) and
      caller.getDeclaringType() = callerClass
    )
  ) >= 2
select target, "This method is called by multiple methods across multiple classes."
