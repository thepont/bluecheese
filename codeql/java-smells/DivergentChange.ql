/**
 * @name Divergent change
 * @description Reports Java classes whose methods repeatedly branch on switches.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/java/divergent-change
 * @precision medium
 * @tags maintainability
 */
import java

from Class owner
where
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
