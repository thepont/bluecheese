// @tests bluecheese/javascript/parallel-hierarchy
import javascript
import ClassSmells

from ClassDefinition candidate
where isParallelHierarchy(candidate)
select candidate.getFile().getBaseName(), candidate.getName()
