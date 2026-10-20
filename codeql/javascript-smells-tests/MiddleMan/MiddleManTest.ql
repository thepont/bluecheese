import javascript

predicate delegatesThroughProjectField(MethodDefinition method, ClassDefinition owner) {
  method.getInit().(FunctionExpr).getBody().(BlockStmt).getNumStmt() = 1 and
  count(MethodCallExpr call | call.getEnclosingFunction() = method.getInit()) = 1 and
  exists(MethodCallExpr call, PropAccess receiver, FieldDefinition field, ClassDefinition target |
    call.getEnclosingFunction() = method.getInit() and
    call.getReceiver() = receiver and
    receiver.getBase() instanceof ThisExpr and
    receiver.getPropertyName() = field.getName() and
    field.getDeclaringType() = owner and
    exists(NewExpr init |
      field.getInit() = init and
      init.getCallee() instanceof Identifier and
      init.getCallee().(Identifier).getName() = target.getName()
    )
  )
}

from ClassDefinition owner
where
  owner.getFile().getRelativePath() = "MiddleMan.js" and
  count(MethodDefinition method | method.getDeclaringType() = owner) >= 2 and
  count(MethodDefinition method |
    method.getDeclaringType() = owner and
    delegatesThroughProjectField(method, owner)
  ) >= 2 and
  2 * count(MethodDefinition method |
    method.getDeclaringType() = owner and
    delegatesThroughProjectField(method, owner)
  ) >= count(MethodDefinition method | method.getDeclaringType() = owner)
select owner, "At least half of this class's methods delegate through a source-defined field."
