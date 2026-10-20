class FirstAlternative {
  int normalizeFirst(int first) {
    int adjusted = first + 10;
    return adjusted * 2;
  }

  int clampFirst(int first) {
    if (first < 0) {
      return 0;
    }
    return first;
  }
}

class SecondAlternative {
  int normalizeSecond(int second) {
    int changed = second + 20;
    return changed * 3;
  }

  int clampSecond(int second) {
    if (second < 5) {
      return 0;
    }
    return second;
  }
}

class Unrelated {
  int onlyOne(int value) {
    return value + 1;
  }

  int branching(int value) {
    if (value < 0) {
      return 0;
    } else if (value > 10) {
      return 10;
    }
    return value;
  }
}
