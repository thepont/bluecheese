import javascript

predicate isFeatureEnvy(MethodDefinition method) {
  exists(Variable receiver |
    count(string name |
      exists(PropAccess access |
        access.getEnclosingFunction() = method.getInit() and
        access.getBase().(VarAccess).getVariable() = receiver and
        name = access.getPropertyName() and
        not exists(CallExpr call | call.getCallee() = access)
      )
    ) >= 3
  )
}

predicate isLazyClass(ClassDefinition owner) {
  not exists(owner.getAField()) and
  count(MethodDefinition method |
    method.getDeclaringType() = owner and method.getName() != "constructor"
  ) = 1 and
  exists(MethodDefinition method, Function fn, CallExpr call |
    method.getDeclaringType() = owner and method.getName() != "constructor" and
    fn = method.getInit() and fn.getNumBodyStmt() = 1 and
    count(CallExpr c | c.getEnclosingFunction() = fn) = 1 and
    (call = fn.getBodyStmt(0).(ReturnStmt).getExpr() or
     call = fn.getBodyStmt(0).(ExprStmt).getExpr())
  ) and
  not exists(PropAccess access, AssignExpr assignment, MethodDefinition constructor |
    constructor.getDeclaringType() = owner and constructor.getName() = "constructor" and
    access.getEnclosingFunction() = constructor.getInit() and
    access.getBase() instanceof ThisExpr and assignment.getLhs() = access
  )
}

predicate isDivergentChange(ClassDefinition owner) {
  count(MethodDefinition method |
    method.getDeclaringType() = owner and
    exists(SwitchStmt branch | branch.getContainer() = method.getInit())
  ) >= 3
  or
  exists(string name |
    exists(PropAccess access | name = access.getPropertyName()) and
    count(MethodDefinition method |
      method.getDeclaringType() = owner and
      exists(IfStmt branch, PropAccess access |
        branch.getContainer() = method.getInit() and
        branch.getCondition().getAChild*() = access and
        access.getBase() instanceof ThisExpr and access.getPropertyName() = name
      )
    ) >= 2
  )
}

predicate fieldReference(FieldDefinition field, PropAccess access) {
  not field.isStatic() and
  access.getBase() instanceof ThisExpr and access.getPropertyName() = field.getName() and
  exists(MethodDefinition method |
    method.getDeclaringType() = field.getDeclaringType() and
    method.getName() != "constructor" and access.getEnclosingFunction() = method.getInit()
  )
}

predicate writeTarget(PropAccess access) {
  exists(AssignExpr assignment | assignment.getLhs() = access)
}

predicate inConditionalBranch(PropAccess access) {
  exists(IfStmt branch |
    branch.getContainer() = access.getEnclosingFunction() and
    branch.getThen().getAChild*() = access
  )
}

predicate isTemporaryField(FieldDefinition field) {
  (
    not exists(field.getInit()) and
    exists(IfStmt branch, PropAccess guard, PropAccess read, NullLiteral literal |
      fieldReference(field, guard) and fieldReference(field, read) and
      branch.getCondition().getAChild*() = guard and branch.getCondition().getAChild*() = literal and
      branch.getThen().getAChild*() = read and not writeTarget(read)
    )
  ) or (
    exists(PropAccess read | fieldReference(field, read) and not writeTarget(read)) and
    forall(PropAccess access | fieldReference(field, access) | inConditionalBranch(access))
  )
}

predicate parentName(ClassDefinition child, string parent) {
  parent = child.getSuperClass().toString() or parent = child.getASuperInterface().toString()
}

predicate correspondingChildren(ClassDefinition left, ClassDefinition right) {
  left != right and left.getFile() = right.getFile() and
  left.getName().length() >= 3 and right.getName().length() >= 3 and
  left.getName().substring(0, 3) = right.getName().substring(0, 3)
}

predicate isParallelHierarchy(ClassDefinition child) {
  exists(ClassDefinition peer, ClassDefinition another, ClassDefinition anotherPeer,
         string leftParent, string rightParent |
    parentName(child, leftParent) and parentName(peer, rightParent) and leftParent != rightParent and
    correspondingChildren(child, peer) and
    parentName(another, leftParent) and parentName(anotherPeer, rightParent) and
    child != another and peer != anotherPeer and child.getFile() = another.getFile() and
    correspondingChildren(another, anotherPeer)
  )
}

predicate methodShape(MethodDefinition left, MethodDefinition right) {
  left.getName() != "constructor" and right.getName() != "constructor" and
  left.getInit().(Function).getNumBodyStmt() > 0 and
  right.getInit().(Function).getNumBodyStmt() > 0 and
  forall(ExprOrStmt node |
    node.getContainer() = left.getInit() or node.getContainer() = right.getInit()
  |
    count(ExprOrStmt a | a.getContainer() = left.getInit() and
      a.getAPrimaryQlClass() = node.getAPrimaryQlClass()) =
    count(ExprOrStmt b | b.getContainer() = right.getInit() and
      b.getAPrimaryQlClass() = node.getAPrimaryQlClass())
  ) and
  forall(BinaryExpr operator |
    operator.getEnclosingFunction() = left.getInit() or
    operator.getEnclosingFunction() = right.getInit()
  |
    count(BinaryExpr a | a.getEnclosingFunction() = left.getInit() and
      a.getOperator() = operator.getOperator()) =
    count(BinaryExpr b | b.getEnclosingFunction() = right.getInit() and
      b.getOperator() = operator.getOperator())
  ) and
  forall(UnaryExpr operator |
    operator.getEnclosingFunction() = left.getInit() or
    operator.getEnclosingFunction() = right.getInit()
  |
    count(UnaryExpr a | a.getEnclosingFunction() = left.getInit() and
      a.getOperator() = operator.getOperator()) =
    count(UnaryExpr b | b.getEnclosingFunction() = right.getInit() and
      b.getOperator() = operator.getOperator())
  )
}

predicate isAlternativeClass(ClassDefinition owner) {
  exists(ClassDefinition peer, MethodDefinition a, MethodDefinition b,
         MethodDefinition c, MethodDefinition d |
    owner != peer and owner.getFile() = peer.getFile() and
    a.getDeclaringType() = owner and b.getDeclaringType() = owner and a != b and
    c.getDeclaringType() = peer and d.getDeclaringType() = peer and c != d and
    methodShape(a, c) and methodShape(b, d)
  )
}
