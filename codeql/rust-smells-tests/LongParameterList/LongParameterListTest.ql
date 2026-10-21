// @tests bluecheese/rust/long-parameter-list
import LongParameterListLib

from LongParameterListFunction function
where function.getFile().getRelativePath() = "LongParameterList.rs"
select function, "This function has more than 3 parameters."
