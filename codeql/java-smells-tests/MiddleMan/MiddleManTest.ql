/** Test BlueCheese's Java Middle Man rule. */
import java

predicate delegatesToProjectField(Method method, Class owner) {
  method.getBody().getNumStmt() = 1 and
  exists(MethodCall call, FieldAccess receiver, Class target |
    call.getEnclosingCallable() = method and
    (
      exists(ReturnStmt statement |
        statement = method.getBody().getStmt(0) and
        statement.getExpr() = call
      ) or
      exists(ExprStmt statement |
        statement = method.getBody().getStmt(0) and
        statement.getExpr() = call
      )
    ) and
    call.getQualifier() = receiver and
    receiver.getField().getDeclaringType() = owner and
    receiver.getField().getType() = target and
    target.fromSource()
  )
}

from Class type
where
  type.getFile().getRelativePath() = "MiddleMan.java" and
  count(Method method | method.getDeclaringType() = type) >= 2 and
  count(Method method |
    method.getDeclaringType() = type and
    delegatesToProjectField(method, type)
  ) >= 2 and
  2 * count(Method method |
    method.getDeclaringType() = type and
    delegatesToProjectField(method, type)
  ) >= count(Method method | method.getDeclaringType() = type)
select type, "At least half of this class's methods delegate to another project type."
