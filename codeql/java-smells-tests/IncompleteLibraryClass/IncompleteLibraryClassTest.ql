/** Test the Java repeated-imported-type helper subset. */
import java

from Class type
where
  type.getFile().getRelativePath() = "IncompleteLibraryClass.java" and
  type.getName() in ["FirstListHelpers", "SecondListHelpers"]
select type, "This class repeats helper behavior around imported type List."
