// SMELLY ON PURPOSE, and frozen: see tests/smelly_java.rs for what this pins.
//
// The counterpart to NestedLong: an enum is a top-level type, so a long method
// inside it IS reported, even though its body sits under the extra
// enum_body_declarations level.
package smelly;

public enum EnumLong {
    ONLY;

    public int reallyLong() {
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
        return total;
    }
}
