import java.util.List;

class FirstListHelpers {
  static boolean isEmpty(List values) {
    return values.isEmpty();
  }
}

class SecondListHelpers {
  static boolean hasNoItems(List items) {
    return items.isEmpty();
  }
}

class UnrelatedHelper {
  static int size(List values) {
    return values.size();
  }
}
