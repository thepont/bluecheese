/** Test BlueCheese's JavaScript/TypeScript long-parameter rule. */
import javascript
import LongParameterListLib

from Function function
where
  (
    function.getFile().getRelativePath() = "JavaScriptLongParameterList.js" or
    function.getFile().getRelativePath() = "LongParameterList.ts"
  ) and
  isLongParameterFunction(function)
select function, "This function exceeds BlueCheese's parameter limit."
