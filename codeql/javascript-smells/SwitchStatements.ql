/**
 * @name Switch statements
 * @description A JavaScript or TypeScript method contains a switch statement.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/javascript/switch-statements
 * @precision high
 * @tags maintainability
 */
import javascript

from Method method, SwitchStmt switchStmt
where switchStmt.getContainer() = method
select switchStmt, "This method contains a switch statement."
