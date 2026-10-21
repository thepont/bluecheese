// @tests bluecheese/rust/message-chains
/** Test BlueCheese's Rust message-chains rule. */
import codeql.rust.elements.MethodCallExpr

predicate isAccessor(MethodCallExpr call) {
  call.getIdentifier().getText().regexpMatch("^(get|is).*")
}

from MethodCallExpr outer, MethodCallExpr middle, MethodCallExpr inner
where
  outer.getFile().getRelativePath() = "MessageChains.rs" and
  outer.getReceiver() = middle and
  middle.getReceiver() = inner and
  isAccessor(outer) and
  isAccessor(middle) and
  isAccessor(inner)
select outer, "This function contains a chain of at least three getter calls."
