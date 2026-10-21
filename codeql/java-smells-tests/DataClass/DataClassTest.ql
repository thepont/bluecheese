// @tests bluecheese/java/data-class
/** Test BlueCheese's Java public-data-class rule. */
import java

from Class type
where
  type.getFile().getRelativePath() = "DataClass.java" and
  exists(Field field |
    field.getDeclaringType() = type and
    field.isPublic() and
    not field.isStatic() and
    not field.isFinal()
  )
select type, "This class exposes mutable public instance data."
