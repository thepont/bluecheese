/**
 * @name Shotgun surgery
 * @description Reports methods called from multiple methods across multiple classes.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/javascript/shotgun-surgery
 * @precision medium
 * @tags maintainability
 */
import javascript

predicate externalCaller(MethodDefinition target, MethodDefinition caller) {
  exists(MethodCallExpr call |
    caller.getAChild*() = call and
    call.getMethodName() = target.getName() and
    caller.getDeclaringType() != target.getDeclaringType()
  )
}

from MethodDefinition target, ClassDefinition owner
where
  target.getDeclaringType() = owner and
  count(MethodDefinition caller | externalCaller(target, caller)) >= 2 and
  count(ClassDefinition callerClass |
    exists(MethodDefinition caller |
      externalCaller(target, caller) and
      caller.getDeclaringType() = callerClass
    )
  ) >= 2
select target, "This method is called by multiple methods across multiple classes."
