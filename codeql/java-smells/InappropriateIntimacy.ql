/**
 * @name Inappropriate intimacy
 * @description A Java method directly reads a field through another source class.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/java/inappropriate-intimacy
 * @precision medium
 * @tags maintainability
 */
import java

from Class owner, Method method, FieldAccess access, RefType receiverType
where
  method.getDeclaringType() = owner and
  access.getEnclosingCallable() = method and
  access.getQualifier().getType() = receiverType and
  receiverType.fromSource() and
  receiverType != owner and
  not access.getQualifier() instanceof TypeAccess
select access, "This method reads a field from another source class."
