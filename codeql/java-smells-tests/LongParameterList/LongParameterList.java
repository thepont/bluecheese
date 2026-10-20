class LongParameterList {
  void atLimit(int a, int b, int c) {}
  void tooMany(int a, int b, int c, int d) {}

  LongParameterList(int a, int b, int c, int d, int e) {}
  LongParameterList(String a, String b, String c, String d, String e, String f) {}

  static class Child extends LongParameterList {
    Child() { super(1, 2, 3, 4, 5); }
    @Override
    void tooMany(int a, int b, int c, int d) {}
  }
}
