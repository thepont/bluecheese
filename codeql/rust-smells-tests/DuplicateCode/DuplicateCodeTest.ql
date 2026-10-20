import rust
import codeql.rust.elements.AssocItemList
import codeql.rust.elements.Function
import codeql.rust.elements.Impl

predicate methodInImpl(Function method, Impl owner) {
  exists(AssocItemList items |
    owner.getAssocItemList() = items and
    items.getAnAssocItem() = method
  )
}

from Function left, Function right, Impl owner
where
  left.getFile().getRelativePath() = "DuplicateCode.rs" and
  methodInImpl(left, owner) and
  methodInImpl(right, owner) and
  left != right and
  left.getLocation().getStartLine() < right.getLocation().getStartLine() and
  right.getFile().getRelativePath() = "DuplicateCode.rs" and
  count(Stmt statement | statement.getEnclosingCallable() = left) >= 3 and
  count(Stmt statement | statement.getEnclosingCallable() = left) =
    count(Stmt statement | statement.getEnclosingCallable() = right) and
  count(Expr expression | expression.getEnclosingCallable() = left) =
    count(Expr expression | expression.getEnclosingCallable() = right)
select left, "This method has the same statement and expression structure as another method in its impl."
