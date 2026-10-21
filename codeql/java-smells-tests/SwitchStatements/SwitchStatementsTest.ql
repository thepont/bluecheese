// @tests bluecheese/java/switch-statements
/** Test BlueCheese's Java Switch Statements rule. */
import java

from Method method, SwitchStmt switchStmt
where
  method.getFile().getRelativePath() = "SwitchStatements.java" and
  switchStmt.getEnclosingCallable() = method
select switchStmt, "This method contains a switch statement."
