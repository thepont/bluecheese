/**
 * @name Large class
 * @description Reports JavaScript or TypeScript classes with at least eight instance fields.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/javascript/large-class
 * @precision medium
 * @tags maintainability
 */
import javascript

from ClassDefinition type
where
  count(FieldDefinition field |
    field.getDeclaringType() = type and
    not field.isStatic()
  ) >= 8
select type, "This class declares at least eight instance fields."
