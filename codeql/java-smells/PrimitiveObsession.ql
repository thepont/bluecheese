/**
 * @name Primitive obsession
 * @description A Java class declares at least four primitive fields.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/java/primitive-obsession
 * @precision high
 * @tags maintainability
 */
import java

from Class type
where
  not type instanceof EnumType and
  not type instanceof Record and
  count(Field field |
    field.getDeclaringType() = type and
    field.getType() instanceof PrimitiveType
  ) >= 4
select type, "This class declares at least four primitive fields."
