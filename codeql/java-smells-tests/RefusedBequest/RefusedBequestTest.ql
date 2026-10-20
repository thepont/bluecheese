/** Test BlueCheese's Java Refused Bequest rule. */
import java

from Class type
where
  type.getFile().getRelativePath() = "RefusedBequest.java" and
  exists(Method method |
    method.getDeclaringType() = type and
    exists(Method inherited | method.overrides(inherited)) and
    exists(Annotation annotation |
      annotation = method.getAnAnnotation() and
      annotation.getType().hasQualifiedName("java.lang", "Override")
    ) and
    (
      isEmptyVoidBodyForTest(method) or
      isUnsupportedOperationBodyForTest(method)
    )
  )
select type, "This override refuses the inherited behavior."

predicate isEmptyVoidBodyForTest(Method method) {
  method.getReturnType().getName() = "void" and
  (
    method.getBody().getNumStmt() = 0 or
    exists(ReturnStmt statement |
      method.getBody().getNumStmt() = 1 and
      statement = method.getBody().getStmt(0) and
      not exists(statement.getExpr())
    )
  )
}

predicate isUnsupportedOperationBodyForTest(Method method) {
  method.getBody().getNumStmt() = 1 and
  exists(ThrowStmt statement, ClassInstanceExpr exception |
    statement = method.getBody().getStmt(0) and
    statement.getExpr() = exception and
    exception.getType().getName().matches("%UnsupportedOperation%")
  )
}
