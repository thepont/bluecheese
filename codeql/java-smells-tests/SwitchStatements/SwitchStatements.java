class SwitchStatements {
  int choose(int value) {
    switch (value) {
      case 1:
        return 10;
      default:
        return 0;
    }
  }

  int plain(int value) {
    return value;
  }

  //semmle-extractor-options: --javac-args -source 14 -target 14
  int chooseExpression(int value) {
    return switch (value) {
      case 1 -> 10;
      default -> 0;
    };
  }
}
