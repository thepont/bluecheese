// SMELLY ON PURPOSE, and frozen: see tests/smelly_java.rs for what this pins.
//
// The trap: a method is allowed three parameters and a constructor five, so the
// same count is over one limit and under the other. A detector that used one
// limit for both would report the compliant constructor or miss nothing here
// while reporting every three-argument method in the world.
//
// The split declaration is reported at the line it is declared on, not at one of
// its parameters: the reference tool reports the position of the declaration.
package smelly;

public class LongParams {

    private final int a;

    public LongParams(int a, int b, int c) {
        this.a = a;
    }

    public LongParams(int a, int b, int c, int d, int e, int f) {
        this.a = a;
    }

    public int three(int a, int b, int c) {
        return a + b + c;
    }

    public int four(int a, int b, int c, int d) {
        return a + b + c + d;
    }

    public int split(
            int a,
            int b,
            int c,
            int d) {
        return a + b + c + d;
    }
}
