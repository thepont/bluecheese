import java

/** A Java method that accesses three different fields on one variable. */
predicate isFeatureEnvy(Method method) {
  exists(
    FieldAccess first, FieldAccess second, FieldAccess third,
    VarAccess firstReceiver, VarAccess secondReceiver, VarAccess thirdReceiver
  |
    first.getSite() = method and
    second.getSite() = method and
    third.getSite() = method and
    firstReceiver = first.getQualifier() and
    secondReceiver = second.getQualifier() and
    thirdReceiver = third.getQualifier() and
    not firstReceiver instanceof FieldAccess and
    not secondReceiver instanceof FieldAccess and
    not thirdReceiver instanceof FieldAccess and
    firstReceiver.getVariable() = secondReceiver.getVariable() and
    firstReceiver.getVariable() = thirdReceiver.getVariable() and
    first.getField().getName() != second.getField().getName() and
    first.getField().getName() != third.getField().getName() and
    second.getField().getName() != third.getField().getName()
  )
}
