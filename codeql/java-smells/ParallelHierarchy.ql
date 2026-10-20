/**
 * @name Parallel inheritance hierarchies
 * @description Reports Java subclasses with corresponding names under separate source-defined parents.
 * @kind problem
 * @problem.severity recommendation
 * @id bluecheese/java/parallel-hierarchy
 * @precision low
 * @tags maintainability
 */
import java

predicate directSubtypeOf(Class child, Class parent) {
  child.getASupertype() = parent and
  not exists(Class intermediate |
    intermediate != child and
    intermediate != parent and
    child.getASupertype() = intermediate and
    intermediate.getASupertype() = parent
  )
}

predicate correspondingNames(Class left, Class right) {
  left != right and
  left.getName().length() >= 3 and
  right.getName().length() >= 3 and
  left.getName().substring(0, 3) = right.getName().substring(0, 3)
}

predicate pairedSubclasses(
  Class leftParent, Class rightParent, Class leftChild, Class rightChild
) {
  leftParent != rightParent and
  leftParent.fromSource() and
  rightParent.fromSource() and
  directSubtypeOf(leftChild, leftParent) and
  directSubtypeOf(rightChild, rightParent) and
  leftChild.getFile() = rightChild.getFile() and
  correspondingNames(leftChild, rightChild)
}

from Class leftParent, Class rightParent, Class leftChild, Class rightChild
where
  pairedSubclasses(leftParent, rightParent, leftChild, rightChild) and
  exists(Class anotherLeft, Class anotherRight |
    anotherLeft != leftChild and
    anotherRight != rightChild and
    pairedSubclasses(leftParent, rightParent, anotherLeft, anotherRight)
  )
select leftChild, "This class is part of a parallel inheritance hierarchy."
