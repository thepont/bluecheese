/**
 * @name Temporary field
 * @description Reports uninitialized Java instance fields used in a null-guarded branch.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/java/temporary-field
 * @precision low
 * @tags maintainability
 */
import java

from Field field, IfStmt branch, FieldAccess guardedAccess, FieldAccess branchAccess
where
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
