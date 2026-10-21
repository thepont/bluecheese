/**
 * @name Lazy Class
 * @description Detects lazy class candidates in JavaScript and TypeScript.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/javascript/lazy-class
 * @precision low
 * @tags maintainability
 */
import javascript
import ClassSmells

from ClassDefinition candidate
where isLazyClass(candidate)
select candidate, "Review this lazy class candidate."
