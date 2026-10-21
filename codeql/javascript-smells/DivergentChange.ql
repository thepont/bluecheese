/**
 * @name Divergent Change
 * @description Detects divergent change candidates in JavaScript and TypeScript.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/javascript/divergent-change
 * @precision low
 * @tags maintainability
 */
import javascript
import ClassSmells

from ClassDefinition candidate
where isDivergentChange(candidate)
select candidate, "Review this divergent change candidate."
