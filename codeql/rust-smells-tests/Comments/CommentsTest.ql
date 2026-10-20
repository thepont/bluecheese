/** Test BlueCheese's Rust explanatory-comment rule. */
import rust

from Comment comment
where
  comment.getFile().getRelativePath() = "Comments.rs" and
  not comment.getCommentMarker().regexpMatch("^(///|//!).*") and
  (
    comment.getText().regexpMatch("(?si).*\\bTODO\\b.*") or
    comment.getText().regexpMatch("(?si).*\\b\\w+\\b(?:\\W+\\w+\\b){5,}.*")
  )
select comment, "This comment is long or marks unfinished work."
