/**
 * @name Lazy class
 * @description A fieldless Java class contains one method that forwards through one call.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/java/lazy-class
 * @precision medium
 * @tags maintainability
 */
import java
import LazyClassLib

from Class type
where isLazyClass(type)
select type, "This class contains only one forwarding method and no fields."
