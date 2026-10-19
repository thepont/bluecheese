// SMELLY ON PURPOSE, and frozen: see tests/smelly_java.rs for what this pins.
//
// The trap: an override restates a parameter list it did not choose, so it is
// not reported — and the declaration it restates is, because that is where the
// parameter list is decided. A detector that reported both would report every
// implementation of an interface; one that reported neither would never report
// the interface at all.
//
// The annotation is read from the method, never resolved against the interface,
// which is what `@Override` marks in the reference tool.
package smelly;

public class OverrideParams implements FourArgs {

    @Override
    public int four(int a, int b, int c, int d) {
        return a + b + c + d;
    }

    public int own(int a, int b, int c, int d) {
        return a + b + c + d;
    }
}

interface FourArgs {

    int four(int a, int b, int c, int d);
}
