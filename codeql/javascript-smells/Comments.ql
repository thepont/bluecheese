/**
 * @name Explanatory comments
 * @description Reports long non-documentation comments and TODO comments.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/javascript/comments
 * @precision medium
 * @tags maintainability
 */
import javascript

from Comment comment
where
  not comment instanceof DocComment and
  (
    comment.getText().regexpMatch("(?si).*\\bTODO\\b.*") or
    comment.getText().regexpMatch("(?si).*\\b\\w+\\b(?:\\W+\\w+\\b){5,}.*")
  )
select comment, "This comment is long or marks unfinished work."
