class Service {
  first() {}
  second() {}
}

class Facade {
  service = new Service();
  first() { return this.service.first(); }
  second() { return this.service.second(); }
}

class Direct {
  first() { return 1; }
  second() { return 2; }
}
