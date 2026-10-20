/**
 * @name Long parameter list
 * @description A function or trait declaration has more explicit parameters than the BlueCheese limit.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/rust/long-parameter-list
 * @precision high
 * @tags maintainability
 */
import LongParameterListLib

from LongParameterListFunction function
select function, "This function has more than 3 parameters."
