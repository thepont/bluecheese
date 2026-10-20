import javascript

/** Holds for a JavaScript or TypeScript function over BlueCheese's parameter limit. */
predicate isLongParameterFunction(Function function) {
  function.getFile().getRelativePath() != "" and
  (
    function.getName() = "constructor" and function.getNumParameter() > 5
    or
    function.getName() != "constructor" and function.getNumParameter() > 3
  )
}
