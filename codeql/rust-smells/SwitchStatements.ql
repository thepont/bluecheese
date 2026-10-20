/**
 * @name Rust match expressions
 * @description A Rust function contains a match expression.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/rust/switch-statements
 * @precision high
 * @tags maintainability
 */
import codeql.rust.elements.MatchExpr

from MatchExpr matchExpr
select matchExpr, "This is a Rust match expression."
