// @tests bluecheese/javascript/message-chains
/** Test BlueCheese's JavaScript and TypeScript message-chains rule. */
import javascript
import MessageChainsLib

from Expr outer
where isMessageChain(outer)
select outer.getFile().getBaseName(), outer.getEnclosingFunction().getName()
