/**
 * @name Message chains
 * @description A JavaScript or TypeScript function navigates at least three properties or accessor calls.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/javascript/message-chains
 * @precision medium
 * @tags maintainability
 */
import javascript
import MessageChainsLib

from Expr outer
where isMessageChain(outer)
select outer, "This function contains at least three property or accessor navigation steps."
