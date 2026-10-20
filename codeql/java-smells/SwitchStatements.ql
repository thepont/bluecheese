/**
 * @name Switch statements
 * @description A Java method contains a switch statement or switch expression.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/java/switch-statements
 * @precision high
 * @tags maintainability
 */
import java

from Method method, SwitchStmt switchStmt
where switchStmt.getEnclosingCallable() = method
select switchStmt, "This method contains a switch statement."
