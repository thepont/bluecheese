/**
 * @name Feature envy
 * @description A Java method reads at least three distinct fields through the same variable.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/java/feature-envy
 * @precision medium
 * @tags maintainability
 */
import java
import FeatureEnvyLib

from Method method
where isFeatureEnvy(method)
select method, "This method reads three or more fields from one other object."
