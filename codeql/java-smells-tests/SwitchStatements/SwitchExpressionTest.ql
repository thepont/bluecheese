/** Test BlueCheese's Java Switch Expressions rule. */
import java

from Method method, SwitchExpr switchExpr
where
  method.getFile().getRelativePath() = "SwitchStatements.java" and
  switchExpr.getEnclosingCallable() = method
select switchExpr, "This method contains a switch expression."
