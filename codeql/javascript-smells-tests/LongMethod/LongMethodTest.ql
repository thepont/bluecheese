/** Test BlueCheese's JavaScript and TypeScript long-method rule. */
import javascript
import LongMethodLib

from Method method
where
  method.getFile().getRelativePath() in ["JavaScriptLongMethod.js", "LongMethod.ts"] and
  isLongMethod(method)
select method.getBody(), "This method exceeds BlueCheese's size or complexity threshold."
