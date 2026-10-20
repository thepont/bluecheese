/**
 * @name Duplicate code
 * @description Reports JavaScript or TypeScript methods with matching statement and expression node-class counts.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/javascript/duplicate-code
 * @precision low
 * @tags maintainability
 */
import javascript

predicate sameShape(MethodDefinition left, MethodDefinition right) {
  not exists(Stmt node |
    node.getParent*() = left.getInit() and
    count(Stmt other |
      other.getParent*() = left.getInit() and
      other.getAPrimaryQlClass() = node.getAPrimaryQlClass()
    ) !=
    count(Stmt other |
      other.getParent*() = right.getInit() and
      other.getAPrimaryQlClass() = node.getAPrimaryQlClass()
    )
  ) and
  not exists(Stmt node |
    node.getParent*() = right.getInit() and
    count(Stmt other |
      other.getParent*() = left.getInit() and
      other.getAPrimaryQlClass() = node.getAPrimaryQlClass()
    ) !=
    count(Stmt other |
      other.getParent*() = right.getInit() and
      other.getAPrimaryQlClass() = node.getAPrimaryQlClass()
    )
  ) and
  not exists(Expr node |
    node.getParent*() = left.getInit() and
    count(Expr other |
      other.getParent*() = left.getInit() and
      other.getAPrimaryQlClass() = node.getAPrimaryQlClass()
    ) !=
    count(Expr other |
      other.getParent*() = right.getInit() and
      other.getAPrimaryQlClass() = node.getAPrimaryQlClass()
    )
  ) and
  not exists(Expr node |
    node.getParent*() = right.getInit() and
    count(Expr other |
      other.getParent*() = left.getInit() and
      other.getAPrimaryQlClass() = node.getAPrimaryQlClass()
    ) !=
    count(Expr other |
      other.getParent*() = right.getInit() and
      other.getAPrimaryQlClass() = node.getAPrimaryQlClass()
    )
  )
}

from ClassDefinition owner, MethodDefinition left, MethodDefinition right
where
  left.getDeclaringType() = owner and
  right.getDeclaringType() = owner and
  left != right and
  left.getLocation().getStartLine() < right.getLocation().getStartLine() and
  count(Stmt statement | statement.getParent*() = left.getInit()) >= 3 and
  count(Stmt statement | statement.getParent*() = right.getInit()) >= 3 and
  sameShape(left, right)
select left, "This method has the same statement and expression structure as another method in its class."
