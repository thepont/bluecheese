import java

/** Holds for a Java method or constructor over BlueCheese's parameter limit. */
predicate isLongParameterCallable(Callable callable) {
  callable.getFile().getRelativePath() != "" and
  (
    callable instanceof Method and
    callable.getNumberOfParameters() > 3 and
    not exists(Method overridden | callable.(Method).overrides(overridden))
    or
    callable instanceof Constructor and callable.getNumberOfParameters() > 5
  )
}
