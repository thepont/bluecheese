class SharedService {
  void serve() {}
}

class FirstClient {
  SharedService service;

  void first() {
    service.serve();
    service.serve();
  }
}

class SecondClient {
  SharedService service;

  void second() {
    service.serve();
  }
}

class PrivateService {
  void serve() {}
}

class OnlyClient {
  PrivateService service;

  void call() {
    service.serve();
  }
}
