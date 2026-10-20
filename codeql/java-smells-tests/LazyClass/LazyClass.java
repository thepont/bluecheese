class Service {
  int fetch(int value) {
    return value;
  }
}

class LazyClass {
  int fetch(Service service, int value) {
    return service.fetch(value);
  }
}

class StatefulForwarder {
  int state;

  int fetch(Service service, int value) {
    return service.fetch(value);
  }
}

class MultipleMethods {
  int first(Service service, int value) {
    return service.fetch(value);
  }

  int second(Service service, int value) {
    return service.fetch(value);
  }
}
