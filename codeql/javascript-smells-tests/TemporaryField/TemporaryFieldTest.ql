// @tests bluecheese/javascript/temporary-field
import javascript
import ClassSmells

from FieldDefinition candidate
where isTemporaryField(candidate)
select candidate.getFile().getBaseName(), candidate.getName()
