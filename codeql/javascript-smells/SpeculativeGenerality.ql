/**
 * @name Speculative generality
 * @description Reports unused abstract JavaScript or TypeScript declarations.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/javascript/speculative-generality
 * @precision medium
 * @tags maintainability
 */
import javascript

from ClassOrInterface type
where
  type.isAbstract() and
  count(TypeAccess access | access.toString() = type.getName()) +
    count(ClassDefinition child |
      child.getSuperClass().toString() = type.getName()
    ) <= 1
select type, "This abstract type has few references and may be speculative."
