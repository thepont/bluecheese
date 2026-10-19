// Not smelly. This file pins what counts as a decision: a logical operator is
// one only when a branch or loop encloses it, mirroring Spoon.
package adapter;

public class Logic {

    // No enclosing branch or loop, so neither operator is a decision.
    public boolean bare(int a) {
        return a > 0 && a < 10;
    }

    // The same operators inside an `if`: the `if` is a decision and so are the
    // two operators, so complexity is 1 + 1 + 2.
    public boolean guarded(int a) {
        if (a > 0 && a < 10 && a != 5) {
            return true;
        }
        return false;
    }
}
