/** Test BlueCheese's TypeScript public-data-class rule. */
import javascript

from ClassDefinition type
where
  type.getFile().getRelativePath() = "DataClass.ts" and
  exists(FieldDefinition field |
    field.getDeclaringType() = type and
    field.isPublic() and
    not field.isStatic()
  )
select type, "This class exposes public instance data."
