/**
 * @name Data clumps
 * @description Methods in one JavaScript or TypeScript class repeat at least two parameter names.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/javascript/data-clumps
 * @precision medium
 * @tags maintainability
 */
import javascript

predicate compatibleParameter(Parameter left, Parameter right) {
  left.getName() = right.getName() and
  (
    not exists(left.getTypeAnnotation()) or
    not exists(right.getTypeAnnotation()) or
    left.getTypeAnnotation().toString() = right.getTypeAnnotation().toString()
  )
}

predicate isParameterOf(Parameter parameter, MethodDefinition method) {
  method.getAChild*() = parameter
}

from ClassDefinition type, MethodDefinition method, MethodDefinition other
where
  method.getDeclaringType() = type and
  other.getDeclaringType() = type and
  method != other and
  count(Parameter parameter |
    isParameterOf(parameter, method) and
    exists(Parameter matching |
      isParameterOf(matching, other) and
      compatibleParameter(parameter, matching)
    )
  ) >= 2
select method, "This method repeats a parameter group used by another method."
