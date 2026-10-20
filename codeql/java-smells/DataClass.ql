/**
 * @name Data class
 * @description A Java class exposes mutable public instance fields.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/java/data-class
 * @precision medium
 * @tags maintainability
 */
import java

from Class type
where
  exists(Field field |
    field.getDeclaringType() = type and
    field.isPublic() and
    not field.isStatic() and
    not field.isFinal()
  )
select type, "This class exposes mutable public instance data."
