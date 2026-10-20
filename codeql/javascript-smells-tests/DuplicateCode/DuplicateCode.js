class Repeated {
  first(value) {
    const next = value + 1;
    const doubled = next * 2;
    return doubled;
  }
  second(input) {
    const later = input + 1;
    const result = later * 2;
    return result;
  }
  different(value) {
    if (value) return 1;
    return 0;
  }
}
