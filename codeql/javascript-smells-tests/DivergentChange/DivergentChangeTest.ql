// @tests bluecheese/javascript/divergent-change
import javascript
import ClassSmells

from ClassDefinition candidate
where isDivergentChange(candidate)
select candidate.getFile().getBaseName(), candidate.getName()
