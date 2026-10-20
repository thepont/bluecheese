class Parent {
  run(value: number) { return value; }
  keep() {}
}

class RefusingChild extends Parent {
  override run(value: number) {}
}

class WorkingChild extends Parent {
  override run(value: number) { return super.run(value); }
}
