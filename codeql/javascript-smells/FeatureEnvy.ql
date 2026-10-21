/**
 * @name Feature Envy
 * @description Detects feature envy candidates in JavaScript and TypeScript.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/javascript/feature-envy
 * @precision low
 * @tags maintainability
 */
import javascript
import ClassSmells

from MethodDefinition candidate
where isFeatureEnvy(candidate)
select candidate, "Review this feature envy candidate."
