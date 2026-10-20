class Other {
  secret = "value";
}

class Intimate {
  other = new Other();
  read() { return this.other.secret; }
  selfRead() { return this.other; }
}

class Encapsulated {
  private value = "value";
  read() { return this.value; }
}
