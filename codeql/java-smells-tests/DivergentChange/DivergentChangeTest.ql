/** Test the repeated-switch case of BlueCheese's Java Divergent Change rule. */
import java

from Class owner
where
  owner.getFile().getRelativePath() = "DivergentChange.java" and
  count(Method method |
    method.getDeclaringType() = owner and
    (
      exists(SwitchStmt switchStmt |
        switchStmt.getEnclosingCallable() = method
      ) or
      exists(SwitchExpr switchExpr |
        switchExpr.getEnclosingCallable() = method
      )
    )
  ) >= 3
select owner, "Switch-based behavior is spread across at least three methods."
