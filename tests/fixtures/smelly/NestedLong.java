// SMELLY ON PURPOSE, and frozen: see tests/smelly_java.rs for what this pins.
//
// The trap: a genuinely long method that must NOT be reported, because it lives
// in a nested type. Spoon's CtModel.getAllTypes() returns top-level types only,
// so a nested class is invisible to every detector.
package smelly;

public class NestedLong {

    // The top-level type has only a short method, so there is nothing to report
    // at this level.
    public int shortOne() {
        return 1;
    }

    public static class Inner {

        public void reallyLong() {
            int total = 0;
            total += 1;
            total += 2;
            total += 3;
            total += 4;
            total += 5;
            total += 6;
            total += 7;
            total += 8;
            total += 9;
            total += 10;
            total += 11;
            total += 12;
            total += 13;
            total += 14;
            total += 15;
            total += 16;
            total += 17;
            total += 18;
            total += 19;
            total += 20;
            total += 21;
            total += 22;
            total += 23;
            total += 24;
            total += 25;
            total += 26;
            total += 27;
            total += 28;
            total += 29;
            total += 30;
        }
    }
}
