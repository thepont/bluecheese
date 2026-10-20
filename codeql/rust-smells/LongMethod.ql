/**
 * @name Long method
 * @description A Rust function reaches BlueCheese's statement or complexity threshold.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/rust/long-method
 * @precision medium
 * @tags maintainability
 */
import rust
import LongMethodLib

from Function function
where isLongMethod(function)
select function.getBody(), "This function exceeds BlueCheese's size or complexity threshold."
