/** Test the TypeScript resolved-field subset of Inappropriate Intimacy. */
import javascript

from ClassDefinition owner, MethodDefinition method,
  PropAccess access, FieldDefinition field
where
  access.getFile().getRelativePath() = "InappropriateIntimacy.ts" and
  method.getDeclaringType() = owner and
  access.getEnclosingFunction() = method.getInit() and
  access.getPropertyName() = field.getName() and
  field.getDeclaringType() != owner
select access, "This method reads a field from another source class."
