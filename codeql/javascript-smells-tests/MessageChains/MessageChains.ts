class MessageChains {
  longChain(root: Root): unknown {
    return root.getChild().getState().getValue();
  }

  shortChain(root: Root): unknown {
    return root.getChild().getState();
  }
}

declare class Root { getChild(): Child; }
declare class Child { getState(): State; }
declare class State { getValue(): unknown; }
