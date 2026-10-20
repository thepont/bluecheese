/**
 * @name Long parameter list
 * @description A JavaScript or TypeScript callable exceeds BlueCheese's parameter limit.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/javascript/long-parameter-list
 * @precision high
 * @tags maintainability
 */
import javascript
import LongParameterListLib

from Function function
where isLongParameterFunction(function)
select function, "This function exceeds BlueCheese's parameter limit."
