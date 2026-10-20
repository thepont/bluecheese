/** Test BlueCheese's Java long-method rule. */
import java
import LongMethodLib

from Method method
where
  method.getFile().getRelativePath() = "LongMethod.java" and
  isLongMethod(method)
select method.getBody(), "This method exceeds BlueCheese's size or complexity threshold."
