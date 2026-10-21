// @tests bluecheese/java/primitive-obsession
/** Test BlueCheese's Java primitive-obsession rule. */
import java

from Class type
where
  type.getFile().getRelativePath() = "PrimitiveObsession.java" and
  not type instanceof EnumType and
  not type instanceof Record and
  count(Field field |
    field.getDeclaringType() = type and
    field.getType() instanceof PrimitiveType
  ) >= 4
select type, "This class declares at least four primitive fields."
