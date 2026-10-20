/**
 * @name Data class
 * @description A JavaScript or TypeScript class exposes public instance fields.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/javascript/data-class
 * @precision medium
 * @tags maintainability
 */
import javascript

from ClassDefinition type
where
  exists(FieldDefinition field |
    field.getDeclaringType() = type and
    field.isPublic() and
    not field.isStatic()
  )
select type, "This class exposes public instance data."
