// @tests bluecheese/javascript/feature-envy
import javascript
import ClassSmells

from MethodDefinition candidate
where isFeatureEnvy(candidate)
select candidate.getFile().getBaseName(), candidate.getName()
