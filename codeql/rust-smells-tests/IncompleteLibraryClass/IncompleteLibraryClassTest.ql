import rust
import codeql.rust.elements.AssocItemList
import codeql.rust.elements.Function
import codeql.rust.elements.Impl
import codeql.rust.elements.Struct
import codeql.rust.elements.Trait

predicate methodInImpl(Function method, Impl owner) {
  exists(AssocItemList items |
    owner.getAssocItemList() = items and
    items.getAnAssocItem() = method
  )
}

predicate sameMethodCount(Impl left, Impl right) {
  count(Function method | methodInImpl(method, left)) =
    count(Function method | methodInImpl(method, right))
}

from Impl left, Impl right, Trait leftTrait, Trait rightTrait
where
  left.getFile().getRelativePath() = "IncompleteLibraryClass.rs" and
  left != right and
  left.getFile() = right.getFile() and
  left.getSelfTy().toString() = right.getSelfTy().toString() and
  left.getTraitTy().toString().matches("%" + leftTrait.getName().toString()) and
  right.getTraitTy().toString().matches("%" + rightTrait.getName().toString()) and
  leftTrait.getFile() = left.getFile() and
  rightTrait.getFile() = right.getFile() and
  leftTrait != rightTrait and
  not exists(Struct localType |
    localType.getFile() = left.getFile() and
    localType.getName().toString() = left.getSelfTy().toString()
  ) and
  sameMethodCount(left, right)
select left, "This file repeats behavior across trait implementations for an external type."
