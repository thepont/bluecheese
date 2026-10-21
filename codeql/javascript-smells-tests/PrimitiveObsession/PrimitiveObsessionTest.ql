// @tests bluecheese/javascript/primitive-obsession
/** Test BlueCheese's TypeScript primitive-obsession rule. */
import javascript

from ClassDefinition type
where
  type.getFile().getRelativePath() = "PrimitiveObsession.ts" and
  count(FieldDeclaration field |
    field.getDeclaringType() = type and
    field.getTypeAnnotation().toString() in
      ["number", "string", "boolean", "bigint", "symbol"]
  ) >= 4
select type, "This class declares at least four primitive-typed fields."
