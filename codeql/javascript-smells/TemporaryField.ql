/**
 * @name Temporary Field
 * @description Detects temporary field candidates in JavaScript and TypeScript.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/javascript/temporary-field
 * @precision low
 * @tags maintainability
 */
import javascript
import ClassSmells

from FieldDefinition candidate
where isTemporaryField(candidate)
select candidate, "Review this temporary field candidate."
