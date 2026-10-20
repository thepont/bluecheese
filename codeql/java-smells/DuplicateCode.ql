/**
 * @name Duplicate code
 * @description Reports same-class Java methods with matching statement and expression node-class counts.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/java/duplicate-code
 * @precision low
 * @tags maintainability
 */
import java

predicate sameShape(Method left, Method right) {
  not exists(Stmt node |
    node.getEnclosingCallable() = left and
    count(Stmt other |
      other.getEnclosingCallable() = left and
      other.getAPrimaryQlClass() = node.getAPrimaryQlClass()
    ) !=
    count(Stmt other |
      other.getEnclosingCallable() = right and
      other.getAPrimaryQlClass() = node.getAPrimaryQlClass()
    )
  ) and
  not exists(Stmt node |
    node.getEnclosingCallable() = right and
    count(Stmt other |
      other.getEnclosingCallable() = left and
      other.getAPrimaryQlClass() = node.getAPrimaryQlClass()
    ) !=
    count(Stmt other |
      other.getEnclosingCallable() = right and
      other.getAPrimaryQlClass() = node.getAPrimaryQlClass()
    )
  ) and
  not exists(Expr node |
    node.getEnclosingCallable() = left and
    count(Expr other |
      other.getEnclosingCallable() = left and
      other.getAPrimaryQlClass() = node.getAPrimaryQlClass()
    ) !=
    count(Expr other |
      other.getEnclosingCallable() = right and
      other.getAPrimaryQlClass() = node.getAPrimaryQlClass()
    )
  ) and
  not exists(Expr node |
    node.getEnclosingCallable() = right and
    count(Expr other |
      other.getEnclosingCallable() = left and
      other.getAPrimaryQlClass() = node.getAPrimaryQlClass()
    ) !=
    count(Expr other |
      other.getEnclosingCallable() = right and
      other.getAPrimaryQlClass() = node.getAPrimaryQlClass()
    )
  )
}

from Class owner, Method left, Method right
where
  left.getDeclaringType() = owner and
  right.getDeclaringType() = owner and
  left != right and
  left.getLocation().getStartLine() < right.getLocation().getStartLine() and
  exists(BlockStmt body | body = left.getBody()) and
  exists(BlockStmt body | body = right.getBody()) and
  count(Stmt statement | statement.getEnclosingCallable() = left) >= 3 and
  count(Stmt statement | statement.getEnclosingCallable() = right) >= 3 and
  sameShape(left, right)
select left, "This method has the same statement and expression structure as another method in its class."
