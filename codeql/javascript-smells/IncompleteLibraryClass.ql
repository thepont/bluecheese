/**
 * @name Incomplete library class
 * @description Reports repeated same-shape JavaScript prototype patches to an imported type.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/javascript/incomplete-library-class
 * @precision low
 * @tags maintainability
 */
import javascript

predicate isPrototypePatch(AssignExpr assignment, Identifier importedName, string methodName, FunctionExpr implementation) {
  exists(PropAccess methodAccess, PropAccess prototypeAccess, Identifier receiver |
    assignment.getLhs() = methodAccess and
    methodAccess.getBase() = prototypeAccess and
    methodAccess.getPropertyName() = methodName and
    prototypeAccess.getPropertyName() = "prototype" and
    prototypeAccess.getBase() = receiver and
    receiver.getName() = importedName.getName() and
    assignment.getRhs() = implementation
  )
}

predicate sameShape(FunctionExpr left, FunctionExpr right) {
  count(Stmt statement | statement.getParent*() = left.getBody()) =
    count(Stmt statement | statement.getParent*() = right.getBody()) and
  count(Expr expression | expression.getParent*() = left.getBody()) =
    count(Expr expression | expression.getParent*() = right.getBody()) and
  count(AstNode node | node.getParent*() = left.getBody()) >= 8
}

from ImportSpecifier imported, Identifier localName, AssignExpr left, AssignExpr right,
  FunctionExpr leftImplementation, FunctionExpr rightImplementation,
  string leftMethod, string rightMethod
where
  imported.getLocal().getName() = localName.getName() and
  isPrototypePatch(left, localName, leftMethod, leftImplementation) and
  isPrototypePatch(right, localName, rightMethod, rightImplementation) and
  leftMethod != rightMethod and
  left.getLocation().getStartLine() < right.getLocation().getStartLine() and
  sameShape(leftImplementation, rightImplementation)
select left, "This prototype patch repeats helper behavior for an imported type."
