class EmptyParent {
  public void empty() {}
}

class EmptyRefusal extends EmptyParent {
  @Override public void empty() {}
}

class WorkParent {
  public void run() {}
}

class ThrowRefusal extends WorkParent {
  @Override public void run() {
    throw new UnsupportedOperationException();
  }
}

class NormalOverride extends WorkParent {
  @Override public void run() {
    System.out.println("working");
  }
}
