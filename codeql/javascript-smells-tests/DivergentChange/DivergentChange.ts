class Decisions {
 first() { if (this.mode) return 1; }
 second() { if (this.mode) return 2; }
}
class Switches {
 first(x) { switch(x) { case 1: return 1; } }
 second(x) { switch(x) { case 2: return 2; } }
 third(x) { switch(x) { case 3: return 3; } }
}
class Cohesive { first() { return this.value; } second() { return this.value + 1; } }
