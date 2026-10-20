class TemporaryContext {
  String context;

  void render() {
    if (this.context != null) {
      System.out.println(this.context);
    }
  }
}

class OrdinaryValue {
  String value;

  void update() {
    this.value = "ready";
  }
}

class ImplicitTemporaryContext {
  String context;

  void render() {
    if (context != null) {
      System.out.println(context);
    }
  }
}

class InitializedContext {
  String context = "ready";

  void render() {
    if (this.context != null) {
      System.out.println(this.context);
    }
  }
}
