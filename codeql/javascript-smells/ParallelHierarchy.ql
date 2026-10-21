/**
 * @name Parallel Hierarchy
 * @description Detects parallel hierarchy candidates in JavaScript and TypeScript.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/javascript/parallel-hierarchy
 * @precision low
 * @tags maintainability
 */
import javascript
import ClassSmells

from ClassDefinition candidate
where isParallelHierarchy(candidate)
select candidate, "Review this parallel hierarchy candidate."
