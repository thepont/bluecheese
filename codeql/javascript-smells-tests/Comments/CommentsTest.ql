// @tests bluecheese/javascript/comments
/** Test BlueCheese's JavaScript/TypeScript Comments rule. */
import javascript

from Comment comment
where
  comment.getFile().getRelativePath() = "Comments.js" and
  not comment instanceof DocComment and
  (
    comment.getText().regexpMatch("(?si).*\\bTODO\\b.*") or
    comment.getText().regexpMatch("(?si).*\\b\\w+\\b(?:\\W+\\w+\\b){5,}.*")
  )
select comment, "This comment is long or marks unfinished work."
