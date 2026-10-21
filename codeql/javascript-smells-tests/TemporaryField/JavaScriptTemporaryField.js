class Scratch { scratch; run(ok) { if (ok) { this.scratch = 1; return this.scratch; } return 0; } }
class Nullable { cache; run() { if (this.cache != null) return this.cache.length; } }
class Always { total = 0; run() { this.total += 1; return this.total; } }
class Local { state; run(ok) { if (ok) { let state = 1; return state; } } }
class Other { state; run(o, ok) { if (ok) return o.state; } }
