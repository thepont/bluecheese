// @tests bluecheese/java/type-code-ladders
/** Test the Java same-selector if/else-if ladder subset. */
import java

predicate comparesVariable(IfStmt branch, Variable variable) {
  exists(EqualityTest equality, VarAccess access |
    equality = branch.getCondition() and
    (equality.getLeftOperand() = access or equality.getRightOperand() = access) and
    access.getVariable() = variable
  )
}

from IfStmt first, IfStmt second, IfStmt third, Variable selector
where
  not exists(IfStmt parent | parent.getElse() = first) and
  first.getElse() = second and
  second.getElse() = third and
  first.getFile().getRelativePath() = "TypeCodeLadders.java" and
  first.getLocation().getStartLine() = 3 and
  comparesVariable(first, selector) and
  comparesVariable(second, selector) and
  comparesVariable(third, selector)
select first, "This conditional ladder branches on the same type-code variable repeatedly."
