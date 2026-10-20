import java

/** A fieldless class with one method that forwards through one method call. */
predicate isLazyClass(Class type) {
  not exists(Field field | field.getDeclaringType() = type) and
  count(Method method | method.getDeclaringType() = type) = 1 and
  exists(Method method, BlockStmt body |
    method.getDeclaringType() = type and
    body = method.getBody() and
    body.getNumStmt() = 1 and
    (
      exists(ReturnStmt statement | body.getAStmt() = statement) or
      exists(ExprStmt statement | body.getAStmt() = statement)
    ) and
    count(MethodCall call | call.getEnclosingCallable() = method) = 1
  )
}
