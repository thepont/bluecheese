import java

/** A Java method that exceeds BlueCheese's size or complexity thresholds. */
predicate isLongMethod(Method method) {
  exists(MetricCallable metrics, Stmt body |
    body = method.getBody() and
    metrics = method.getMetrics() and
    (
      logicalLinesOfCode(method) >= 30 or
      metrics.getCyclomaticComplexity() > 15
    )
  )
}

/** Count executable statements, excluding block containers. */
int logicalLinesOfCode(Method method) {
  result =
    count(Stmt stmt |
      stmt.getParent+() = method.getBody() and not stmt instanceof BlockStmt
    ) +
    count(EnhancedForStmt loop | loop.getParent+() = method.getBody())
}
