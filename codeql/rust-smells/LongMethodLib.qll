import rust

/** A Rust function that exceeds BlueCheese's statement or complexity threshold. */
predicate isLongMethod(Function function) {
  function.hasBody() and
  (
    statementCount(function) >= 30 or
    cyclomaticComplexity(function) > 15
  )
}

/** Count statements belonging to this function, excluding nested functions. */
int statementCount(Function function) {
  result = count(Stmt statement |
    statement.getEnclosingCallable() = function and statement != function
  )
}

/** Rust branch points mirror BlueCheese's adapter metric. */
int cyclomaticComplexity(Function function) {
  result = 1 +
    count(IfExpr expression | expression.getEnclosingCallable() = function) +
    count(ForExpr expression | expression.getEnclosingCallable() = function) +
    count(WhileExpr expression | expression.getEnclosingCallable() = function) +
    count(LoopExpr expression | expression.getEnclosingCallable() = function) +
    count(TryExpr expression | expression.getEnclosingCallable() = function) +
    count(MatchArm arm |
      arm.getEnclosingCallable() = function and
      not exists(WildcardPat wildcard | arm.getPat() = wildcard)
    ) +
    count(BinaryExpr expression |
      expression.getEnclosingCallable() = function and
      expression.getOperatorName() in ["&&", "||"]
    )
}
