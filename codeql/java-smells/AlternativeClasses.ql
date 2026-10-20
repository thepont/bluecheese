/**
 * @name Alternative classes with different interfaces
 * @description Reports same-file Java classes with two methods sharing body structure.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/java/alternative-classes-different-interfaces
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

from Class left, Class right
where
  left != right and
  left.getFile() = right.getFile() and
  exists(Method leftA, Method leftB, Method rightA, Method rightB |
    leftA.getDeclaringType() = left and
    leftB.getDeclaringType() = left and
    rightA.getDeclaringType() = right and
    rightB.getDeclaringType() = right and
    leftA != leftB and
    rightA != rightB and
    exists(BlockStmt body | body = leftA.getBody()) and
    exists(BlockStmt body | body = rightA.getBody()) and
    exists(BlockStmt body | body = leftB.getBody()) and
    exists(BlockStmt body | body = rightB.getBody()) and
    sameShape(leftA, rightA) and
    sameShape(leftB, rightB)
  )
select left, "This class shares method-body structure with another class in this file."
