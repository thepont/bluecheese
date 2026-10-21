// @tests bluecheese/java/feature-envy
/** Test BlueCheese's Java Feature Envy rule. */
import java
import FeatureEnvyLib

from Method method
where
  method.getFile().getRelativePath() = "FeatureEnvy.java" and
  isFeatureEnvy(method)
select method, "This method reads three or more fields from one other object."
