// The control, and frozen: see tests/smelly_java.rs for what this pins.
//
// Not smelly. A corpus of only smelly code cannot tell a working detector from
// one that reports everything, so this file must stay clean.
package smelly;

public class Clean {

    public int one() {
        return 1;
    }

    public int two() {
        return 2;
    }
}
