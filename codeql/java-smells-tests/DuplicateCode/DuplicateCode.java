class RepeatedOperations {
  int calculateFirst(int first) {
    int shifted = first + 4;
    int scaled = shifted * 3;
    return scaled;
  }

  int calculateSecond(int second) {
    int moved = second + 9;
    int enlarged = moved * 6;
    return enlarged;
  }

  int unrelated(int value) {
    if (value < 0) {
      return -value;
    }
    return value;
  }
}
