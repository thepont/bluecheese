/**
 * @name Explanatory comments
 * @description Reports long non-documentation Rust comments and TODO comments.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/rust/comments
 * @precision medium
 * @tags maintainability
 */
import rust

from Comment comment
where
  not comment.getCommentMarker().regexpMatch("^(///|//!).*") and
  (
    comment.getText().regexpMatch("(?si).*\\bTODO\\b.*") or
    comment.getText().regexpMatch("(?si).*\\b\\w+\\b(?:\\W+\\w+\\b){5,}.*")
  )
select comment, "This comment is long or marks unfinished work."
