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

from Class type
where
  type.isAbstract() and
  count(TypeAccess access | access.getType() = type) <= 1
select type, "This abstract class has few references and may be speculative."
