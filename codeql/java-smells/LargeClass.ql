/**
 * @name Large class
 * @description Reports Java classes with at least eight non-static fields.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/java/large-class
 * @precision medium
 * @tags maintainability
 */
import java

from Class type
where
  count(Field field |
    field.getDeclaringType() = type and
    not field.isStatic()
  ) >= 8
select type, "This class declares at least eight instance fields."
