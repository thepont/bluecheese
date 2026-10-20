/**
 * @name Refused bequest
 * @description Reports Java overrides that refuse inherited behavior with an empty body or unsupported-operation exception.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/java/refused-bequest
 * @precision medium
 * @tags maintainability
 */
import java

predicate isEmptyVoidBody(Method method) {
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

predicate isUnsupportedOperationBody(Method method) {
  method.getBody().getNumStmt() = 1 and
  exists(ThrowStmt statement, ClassInstanceExpr exception |
    statement = method.getBody().getStmt(0) and
    statement.getExpr() = exception and
    exception.getType().getName().matches("%UnsupportedOperation%")
  )
}

from Method method, Class owner
where
  method.getDeclaringType() = owner and
  exists(Method inherited | method.overrides(inherited)) and
  exists(Annotation annotation |
    annotation = method.getAnAnnotation() and
    annotation.getType().hasQualifiedName("java.lang", "Override")
  ) and
  (isEmptyVoidBody(method) or isUnsupportedOperationBody(method))
select owner, "This override refuses the inherited behavior."
