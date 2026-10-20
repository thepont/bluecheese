/**
 * @name Data clumps
 * @description Methods in one Java class repeat at least two parameter names and types.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/java/data-clumps
 * @precision medium
 * @tags maintainability
 */
import java

from Class type, Method method, Method other
where
  method.getDeclaringType() = type and
  other.getDeclaringType() = type and
  method != other and
  count(Parameter parameter |
    parameter.getCallable() = method and
    exists(Parameter matching |
      matching.getCallable() = other and
      parameter.getName() = matching.getName() and
      parameter.getType() = matching.getType()
    )
  ) >= 2
select method, "This method repeats a parameter group used by another method."
