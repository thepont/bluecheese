/** Test BlueCheese's JavaScript and TypeScript Switch Statements rule. */
import javascript

from Method method, SwitchStmt switchStmt
where
  method.getFile().getRelativePath() in
    ["JavaScriptSwitchStatements.js", "SwitchStatements.ts"] and
  switchStmt.getContainer() = method
select switchStmt, "This method contains a switch statement."
