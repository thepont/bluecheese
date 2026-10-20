/**
 * @name Speculative generality
 * @description An abstract Java class has at most one source type reference.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/java/speculative-generality
 * @precision medium
 * @tags maintainability
 */
import java

from ClassOrInterface type
where
  (
    type instanceof Interface or
    exists(Class abstractClass |
      type = abstractClass and abstractClass.isAbstract()
    )
  ) and
  count(TypeAccess access | access.getType() = type) <= 1
select type, "This abstract type has few references and may be speculative."
