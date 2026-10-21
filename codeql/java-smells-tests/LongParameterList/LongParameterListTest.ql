// @tests bluecheese/java/long-parameter-list
/** Test BlueCheese's Java long-parameter rule. */
import java
import LongParameterListLib

from Callable callable
where
  callable.getFile().getRelativePath() = "LongParameterList.java" and
  isLongParameterCallable(callable)
select callable, "This callable exceeds BlueCheese's parameter limit."
