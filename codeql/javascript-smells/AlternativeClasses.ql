/**
 * @name Alternative Classes
 * @description Detects alternative classes candidates in JavaScript and TypeScript.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/javascript/alternative-classes-different-interfaces
 * @precision low
 * @tags maintainability
 */
import javascript
import ClassSmells

from ClassDefinition candidate
where isAlternativeClass(candidate)
select candidate, "Review this alternative classes candidate."
