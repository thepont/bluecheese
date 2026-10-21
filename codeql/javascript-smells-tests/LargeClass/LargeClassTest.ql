// @tests bluecheese/javascript/large-class
/** Test the JavaScript/TypeScript large-class field-count rule. */
import javascript

from ClassDefinition type
where
  count(FieldDefinition field |
    field.getDeclaringType() = type and
    not field.isStatic()
  ) >= 8
select type, "This class declares at least eight instance fields."
