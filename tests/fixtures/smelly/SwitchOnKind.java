// SMELLY ON PURPOSE, and frozen: see tests/refactored.rs for what this pins.
//
// The before half of a pair, and the book's Switch Statements smell in its most
// ordinary shape: a method that switches on a type code. It is long enough to be
// reported for size as well, which is usually what sends a reader looking for
// the switch in the first place.
//
// The three case bodies add up to 45, 135 and 225 respectively. Its twin in
// tests/fixtures/refactored/ computes the same three numbers without a switch,
// so the pair is behaviourally equal, not merely similar.

package smelly;

public class SwitchOnKind {

    public enum Kind {
        LETTER,
        PARCEL,
        PALLET
    }

    public int cost(Kind kind, int weight) {
        int cost = 0;
        switch (kind) {
            case LETTER:
                cost += weight;
                cost += 1;
                cost += 2;
                cost += 3;
                cost += 4;
                cost += 5;
                cost += 6;
                cost += 7;
                cost += 8;
                cost += 9;
                break;
            case PARCEL:
                cost += weight;
                cost += 11;
                cost += 12;
                cost += 13;
                cost += 14;
                cost += 15;
                cost += 16;
                cost += 17;
                cost += 18;
                cost += 19;
                break;
            default:
                cost += weight;
                cost += 21;
                cost += 22;
                cost += 23;
                cost += 24;
                cost += 25;
                cost += 26;
                cost += 27;
                cost += 28;
                cost += 29;
                break;
        }
        return cost;
    }
}
