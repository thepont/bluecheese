/**
 * @name Long parameter list
 * @description A Java method or constructor exceeds BlueCheese's parameter limit.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/java/long-parameter-list
 * @precision high
 * @tags maintainability
 */
import java
import LongParameterListLib

from Callable callable
where isLongParameterCallable(callable)
select callable, "This callable exceeds BlueCheese's parameter limit."
