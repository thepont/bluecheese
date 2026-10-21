// @tests bluecheese/java/alternative-classes-different-interfaces
/** Test BlueCheese's Java Alternative Classes heuristic. */
import java

from Class type
where
  type.getFile().getRelativePath() = "AlternativeClasses.java" and
  type.getName() in ["FirstAlternative", "SecondAlternative"]
select type, "This class shares method-body structure with another class in this file."
