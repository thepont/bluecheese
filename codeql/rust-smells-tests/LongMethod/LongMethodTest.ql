// @tests bluecheese/rust/long-method
/** Test BlueCheese's Rust long-method rule. */
import rust
import LongMethodLib

from Function function
where
  function.getFile().getRelativePath() = "LongMethod.rs" and
  isLongMethod(function)
select function.getBody(), "This function exceeds BlueCheese's size or complexity threshold."
