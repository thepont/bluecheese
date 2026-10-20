/**
 * @name Long method
 * @description A Java method reaches BlueCheese's size or complexity threshold.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/java/long-method
 * @precision medium
 * @tags maintainability
 */
import java
import LongMethodLib

from Method method
where isLongMethod(method)
select method.getBody(), "This method exceeds BlueCheese's size or complexity threshold."
