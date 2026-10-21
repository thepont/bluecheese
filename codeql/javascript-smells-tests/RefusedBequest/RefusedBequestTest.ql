// @tests bluecheese/javascript/refused-bequest
/** Test the JavaScript/TypeScript refused-bequest rule. */
import javascript

predicate emptyMethod(MethodDefinition method) {
  method.getInit().(Function).getNumBodyStmt() = 0
}

from ClassDefinition child, ClassDefinition parent,
  MethodDefinition method, MethodDefinition inherited
where
  child.getFile().getRelativePath() = "RefusedBequest.ts" and
  child.getFile() = parent.getFile() and
  child.getSuperClass().toString() = parent.getName() and
  count(ClassDefinition candidate |
    candidate.getFile() = child.getFile() and
    candidate.getName() = child.getSuperClass().toString()
  ) = 1 and
  method.getDeclaringType() = child and
  inherited.getDeclaringType() = parent and
  method.getName() = inherited.getName() and
  method.getInit().(Function).getNumParameter() =
    inherited.getInit().(Function).getNumParameter() and
  emptyMethod(method)
select method, "This empty override refuses behavior inherited from its parent class."
