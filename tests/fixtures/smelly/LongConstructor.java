// SMELLY ON PURPOSE, and frozen: see tests/smelly_java.rs for what this pins.
//
// The trap: a constructor that is as long as any smelly method, which must NOT
// be reported. Spoon's CtType.getMethods() returns methods only, and a
// detector that measured constructors would invent a finding here.
package smelly;

public class LongConstructor {

    private int total;

    public LongConstructor() {
        this.total = 0;
        this.total += 1;
        this.total += 2;
        this.total += 3;
        this.total += 4;
        this.total += 5;
        this.total += 6;
        this.total += 7;
        this.total += 8;
        this.total += 9;
        this.total += 10;
        this.total += 11;
        this.total += 12;
        this.total += 13;
        this.total += 14;
        this.total += 15;
        this.total += 16;
        this.total += 17;
        this.total += 18;
        this.total += 19;
        this.total += 20;
        this.total += 21;
        this.total += 22;
        this.total += 23;
        this.total += 24;
        this.total += 25;
        this.total += 26;
        this.total += 27;
        this.total += 28;
        this.total += 29;
        this.total += 30;
    }

    public int total() {
        return total;
    }
}
