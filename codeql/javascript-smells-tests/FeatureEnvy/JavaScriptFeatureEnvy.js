class Invoice { total(c) { return c.balance + c.tax + c.discount; } }
class Own { total() { return this.balance + this.tax + this.discount; } }
class Small { run(c) { return c.a + c.b + c.b; } }
class Calls { run(c) { c.first(); c.second(); c.third(); } }
