/** Test the null-guard case of BlueCheese's Java Temporary Field rule. */
import java

from Field field, IfStmt branch, FieldAccess guardedAccess, FieldAccess branchAccess
where
  field.getFile().getRelativePath() = "TemporaryField.java" and
  field.getDeclaringType() instanceof Class and
  not field.isStatic() and
  not exists(field.getInitializer()) and
  guardedAccess.getField() = field and
  guardedAccess.getLocation().getStartLine() >= branch.getCondition().getLocation().getStartLine() and
  guardedAccess.getLocation().getEndLine() <= branch.getCondition().getLocation().getEndLine() and
  exists(NullLiteral nullLiteral |
    branch.getCondition().getAChildExpr*() = nullLiteral
  ) and
  branchAccess.getField() = field and
  branchAccess.getLocation().getStartLine() >= branch.getThen().getLocation().getStartLine() and
  branchAccess.getLocation().getEndLine() <= branch.getThen().getLocation().getEndLine()
select field, "This field is read only in a null-guarded branch."
