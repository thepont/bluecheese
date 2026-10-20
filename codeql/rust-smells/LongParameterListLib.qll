import rust

/** A Rust function that violates BlueCheese's long-parameter threshold. */
class LongParameterListFunction extends Function {
  LongParameterListFunction() {
    this.getFile().getRelativePath() != "" and
    this.getNumberOfParams() > 3 and
    not exists(Impl impl, Trait trait, AssocItemList items |
      impl.getTrait() = trait and
      impl.getAssocItemList() = items and
      items.getAnAssocItem() = this
    )
  }
}
