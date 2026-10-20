import javascript

/** A JavaScript or TypeScript method that exceeds BlueCheese's thresholds. */
predicate isLongMethod(Method method) {
  method.hasBody() and
  (
    logicalLinesOfCode(method) >= 30 or
    method.getCyclomaticComplexity() > 15
  )
}

/** Count executable statements in the method, excluding block containers. */
int logicalLinesOfCode(Method method) {
  result = count(Stmt statement |
    statement.getContainer() = method and not statement instanceof BlockStmt
  )
}
