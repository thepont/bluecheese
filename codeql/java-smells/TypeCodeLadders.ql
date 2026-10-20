/**
 * @name Type-code conditional ladders
 * @description Reports Java else-if ladders with three equality tests on the same variable.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/java/type-code-ladders
 * @precision medium
 * @tags maintainability
 */
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
  comparesVariable(first, selector) and
  comparesVariable(second, selector) and
  comparesVariable(third, selector)
select first, "This conditional ladder branches on the same type-code variable repeatedly."
