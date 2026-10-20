/**
 * @name Primitive obsession
 * @description A TypeScript class declares at least four primitive-typed fields.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/javascript/primitive-obsession
 * @precision medium
 * @tags maintainability
 */
import javascript

from ClassDefinition type
where
  count(FieldDeclaration field |
    field.getDeclaringType() = type and
    field.getTypeAnnotation().toString() in
      ["number", "string", "boolean", "bigint", "symbol"]
  ) >= 4
select type, "This class declares at least four primitive-typed fields."
