/** Test the field-count branch of BlueCheese's Java Large Class rule. */
import java

from Class type
where
  type.getFile().getRelativePath() = "LargeClass.java" and
  count(Field field |
    field.getDeclaringType() = type and
    not field.isStatic()
  ) >= 8
select type, "This class declares at least eight instance fields."
