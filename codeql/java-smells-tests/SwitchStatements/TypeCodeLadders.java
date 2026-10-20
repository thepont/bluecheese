class TypeCodeLadders {
  int action(int kind) {
    if (kind == 1) {
      return 10;
    } else if (kind == 2) {
      return 20;
    } else if (kind == 3) {
      return 30;
    }
    return 0;
  }

  int twoBranches(int kind) {
    if (kind == 1) {
      return 1;
    } else if (kind == 2) {
      return 2;
    }
    return 0;
  }

  int differentSelectors(int first, int second) {
    if (first == 1) {
      return 1;
    } else if (second == 2) {
      return 2;
    } else if (first == 3) {
      return 3;
    }
    return 0;
  }
}
