/**
 * @name Incomplete library class
 * @description Reports repeated same-file Java helper methods extending an imported type.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/java/incomplete-library-class
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

from ImportType importDecl, ClassOrInterface libraryType, Class left, Class right,
  Method leftHelper, Method rightHelper, Parameter leftReceiver, Parameter rightReceiver,
  VarAccess leftAccess, VarAccess rightAccess, MethodCall leftCall, MethodCall rightCall
where
  importDecl.getImportedType() = libraryType and
  importDecl.getFile() = left.getFile() and
  importDecl.getFile() = right.getFile() and
  left != right and
  leftHelper.getDeclaringType() = left and
  rightHelper.getDeclaringType() = right and
  leftHelper.isStatic() and
  rightHelper.isStatic() and
  leftHelper.getParameter(0) = leftReceiver and
  rightHelper.getParameter(0) = rightReceiver and
  leftReceiver.getType() = libraryType and
  rightReceiver.getType() = libraryType and
  leftCall.getEnclosingCallable() = leftHelper and
  rightCall.getEnclosingCallable() = rightHelper and
  leftCall.getQualifier() = leftAccess and
  rightCall.getQualifier() = rightAccess and
  leftAccess.getVariable() = leftReceiver and
  rightAccess.getVariable() = rightReceiver and
  exists(BlockStmt body | body = leftHelper.getBody()) and
  exists(BlockStmt body | body = rightHelper.getBody()) and
  sameShape(leftHelper, rightHelper)
select left, "This class repeats helper behavior around imported type " + libraryType.getName() + "."
