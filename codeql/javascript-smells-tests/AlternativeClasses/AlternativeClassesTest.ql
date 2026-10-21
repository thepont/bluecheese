// @tests bluecheese/javascript/alternative-classes-different-interfaces
import javascript
import ClassSmells

from ClassDefinition candidate
where isAlternativeClass(candidate)
select candidate.getFile().getBaseName(), candidate.getName()
