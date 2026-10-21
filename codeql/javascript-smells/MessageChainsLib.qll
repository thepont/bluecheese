import javascript

predicate navigationStep(Expr outer, Expr receiver) {
  exists(MethodCallExpr call |
    outer = call and call.getMethodName().regexpMatch("^(get|is|has).*") and
    receiver = call.getReceiver()
  )
  or
  exists(PropAccess access |
    outer = access and receiver = access.getBase() and
    not exists(CallExpr call | call.getCallee() = access)
  )
}

predicate isMessageChain(Expr outer) {
  exists(Expr middle, Expr inner |
    navigationStep(outer, middle) and navigationStep(middle, inner) and navigationStep(inner, _)
  )
}
