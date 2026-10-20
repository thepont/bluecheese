class IntimateOwner {
  int ownValue;

  int readOther(OtherType other) {
    return other.value;
  }

  int readOwn() {
    return ownValue;
  }
}

class OtherType {
  int value;
}
