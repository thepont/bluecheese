/**
 * @name Inappropriate intimacy
 * @description A JavaScript or TypeScript method directly reads a field through another source class.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/javascript/inappropriate-intimacy
 * @precision medium
 * @tags maintainability
 */
import javascript

from ClassDefinition owner, MethodDefinition method,
  PropAccess access, FieldDefinition field
where
  method.getDeclaringType() = owner and
  access.getEnclosingFunction() = method.getInit() and
  access.getPropertyName() = field.getName() and
  field.getDeclaringType() != owner
select access, "This method reads a field from another source class."
