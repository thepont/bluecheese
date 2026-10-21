// @tests bluecheese/java/duplicate-code
/** Test the Java same-class duplicate-body subset. */
import java

from Method method
where
  method.getFile().getRelativePath() = "DuplicateCode.java" and
  method.getDeclaringType().getName() = "RepeatedOperations" and
  method.getName() in ["calculateFirst", "calculateSecond"]
select method, "This method has the same statement and expression structure as another method in its class."
