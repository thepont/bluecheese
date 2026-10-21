// @tests bluecheese/java/inappropriate-intimacy
/** Test BlueCheese's Java inappropriate-intimacy rule. */
import java

from Class owner, Method method, FieldAccess access, RefType receiverType
where
  method.getFile().getRelativePath() = "InappropriateIntimacy.java" and
  method.getDeclaringType() = owner and
  access.getEnclosingCallable() = method and
  access.getQualifier().getType() = receiverType and
  receiverType.fromSource() and
  receiverType != owner and
  not access.getQualifier() instanceof TypeAccess
select access, "This method reads a field from another source class."
