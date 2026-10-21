// @tests bluecheese/javascript/speculative-generality
/** Test JavaScript/TypeScript abstract-class and interface coverage. */
import javascript

from ClassOrInterface type
where
  type.getFile().getRelativePath() = "SpeculativeGenerality.ts" and
  type.isAbstract() and
  count(TypeAccess access | access.toString() = type.getName()) +
    count(ClassDefinition child |
      child.getSuperClass().toString() = type.getName()
    ) <= 1
select type, "This abstract type has few references and may be speculative."
