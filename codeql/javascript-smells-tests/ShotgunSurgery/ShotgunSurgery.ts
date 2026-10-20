class SharedService {
  serve() {}
}

class FirstClient {
  service = new SharedService();
  first() { this.service.serve(); }
}

class SecondClient {
  service = new SharedService();
  second() { this.service.serve(); }
}

class OnlyClient {
  privateOnly() { this.serviceOnly(); }
  serviceOnly() {}
}
