// @tests bluecheese/java/speculative-generality
/** Test BlueCheese's Java speculative-generality rule. */
import java

from ClassOrInterface type
where
  type.getFile().getRelativePath() = "SpeculativeGenerality.java" and
  (
    type instanceof Interface or
    exists(Class abstractClass |
      type = abstractClass and abstractClass.isAbstract()
    )
  ) and
  count(TypeAccess access | access.getType() = type) <= 1
select type, "This abstract type has few references and may be speculative."
