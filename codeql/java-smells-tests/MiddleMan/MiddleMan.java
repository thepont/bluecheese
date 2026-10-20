class MiddleMan {
  private Worker worker;

  int first() { return worker.first(); }

  int second() { return worker.second(); }
}

class Worker {
  int first() { return 1; }
  int second() { return 2; }
}

class NotMiddleMan {
  private Worker worker;

  int delegate() { return worker.first(); }

  int ownWork() { return 1; }
  int moreOwnWork() { return 2; }
}
