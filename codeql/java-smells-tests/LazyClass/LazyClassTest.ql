/** Test BlueCheese's Java Lazy Class rule. */
import java
import LazyClassLib

from Class type
where
  type.getFile().getRelativePath() = "LazyClass.java" and
  isLazyClass(type)
select type, "This class contains only one forwarding method and no fields."
