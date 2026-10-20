class MessageChains {
  Object longChain(Root root) {
    return root.getChild().getState().getValue();
  }

  Object shortChain(Root root) {
    return root.getChild().getState();
  }
}

class Root {
  Child getChild() { return null; }
}

class Child {
  State getState() { return null; }
}

class State {
  Object getValue() { return null; }
}
