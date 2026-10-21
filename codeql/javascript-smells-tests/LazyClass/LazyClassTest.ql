// @tests bluecheese/javascript/lazy-class
import javascript
import ClassSmells

from ClassDefinition candidate
where isLazyClass(candidate)
select candidate.getFile().getBaseName(), candidate.getName()
