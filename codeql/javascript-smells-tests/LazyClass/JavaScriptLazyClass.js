class Shell { run(x) { return worker.run(x); } }
class Stateful { count = 0; run(x) { return worker.run(x); } }
class Useful { first() { return 1; } second() { return 2; } }
class Compute { run(x) { return worker.run(x) + 1; } }
