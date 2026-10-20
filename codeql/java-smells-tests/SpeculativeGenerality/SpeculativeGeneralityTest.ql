/** Test BlueCheese's Java speculative-generality rule. */
import java

from Class type
where
  type.getFile().getRelativePath() = "SpeculativeGenerality.java" and
  type.isAbstract() and
  count(TypeAccess access | access.getType() = type) <= 1
select type, "This abstract class has few references and may be speculative."
