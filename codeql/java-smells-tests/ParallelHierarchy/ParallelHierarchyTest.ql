// @tests bluecheese/java/parallel-hierarchy
/** Test BlueCheese's Java Parallel Inheritance Hierarchies rule. */
import java

predicate directSubtypeOfForTest(Class child, Class parent) {
  child.getASupertype() = parent and
  not exists(Class intermediate |
    intermediate != child and
    intermediate != parent and
    child.getASupertype() = intermediate and
    intermediate.getASupertype() = parent
  )
}

predicate correspondingNamesForTest(Class left, Class right) {
  left != right and
  left.getName().length() >= 3 and
  right.getName().length() >= 3 and
  left.getName().substring(0, 3) = right.getName().substring(0, 3)
}

predicate pairedSubclassesForTest(
  Class leftParent, Class rightParent, Class leftChild, Class rightChild
) {
  leftParent != rightParent and
  leftParent.fromSource() and
  rightParent.fromSource() and
  directSubtypeOfForTest(leftChild, leftParent) and
  directSubtypeOfForTest(rightChild, rightParent) and
  leftChild.getFile() = rightChild.getFile() and
  correspondingNamesForTest(leftChild, rightChild)
}

from Class leftParent, Class rightParent, Class leftChild, Class rightChild
where
  leftChild.getFile().getRelativePath() = "ParallelHierarchy.java" and
  pairedSubclassesForTest(leftParent, rightParent, leftChild, rightChild) and
  exists(Class anotherLeft, Class anotherRight |
    anotherLeft != leftChild and
    anotherRight != rightChild and
    pairedSubclassesForTest(leftParent, rightParent, anotherLeft, anotherRight)
  )
select leftChild, "This class is part of a parallel inheritance hierarchy."
