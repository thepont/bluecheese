// @tests bluecheese/java/data-clumps
/** Test BlueCheese's Java data-clumps rule. */
import java

from Class type, Method method, Method other
where
  type.getFile().getRelativePath() = "DataClumps.java" and
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
