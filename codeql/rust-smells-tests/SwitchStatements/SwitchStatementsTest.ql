// @tests bluecheese/rust/switch-statements
/** Test BlueCheese's Rust match-expression rule. */
import codeql.rust.elements.MatchExpr

from MatchExpr matchExpr
where
  matchExpr.getFile().getRelativePath() = "SwitchStatements.rs"
select matchExpr, "This is a Rust match expression."
