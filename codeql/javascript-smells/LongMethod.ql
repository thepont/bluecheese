/**
 * @name Long method
 * @description A JavaScript or TypeScript method reaches BlueCheese's size or complexity threshold.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/javascript/long-method
 * @precision medium
 * @tags maintainability
 */
import javascript
import LongMethodLib

from Method method
where isLongMethod(method)
select method.getBody(), "This method exceeds BlueCheese's size or complexity threshold."
