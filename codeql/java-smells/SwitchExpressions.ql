/**
 * @name Switch expressions
 * @description A Java method contains a switch expression.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/java/switch-expressions
 * @precision high
 * @tags maintainability
 */
import java

from Method method, SwitchExpr switchExpr
where switchExpr.getEnclosingCallable() = method
select switchExpr, "This method contains a switch expression."
