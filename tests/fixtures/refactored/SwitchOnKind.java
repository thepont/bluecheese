// The after half of the pair whose before half is
// tests/fixtures/smelly/SwitchOnKind.java, and frozen.
//
// Replace Conditional with Polymorphism, applied to a type code that was an enum
// and is now a class hierarchy: each kind knows its own cost, so the method that
// hosted the switch is one line and every method in the file is short.
//
// 45, 135 and 225 are exactly the sums the three case bodies in the twin add up
// to, which is what makes this a refactoring rather than a rewrite.

package refactored;

public class SwitchOnKind {

    public int cost(Kind kind, int weight) {
        return kind.costOf(weight);
    }

    public abstract static class Kind {
        public abstract int costOf(int weight);
    }

    public static final class Letter extends Kind {
        @Override
        public int costOf(int weight) {
            return weight + 45;
        }
    }

    public static final class Parcel extends Kind {
        @Override
        public int costOf(int weight) {
            return weight + 135;
        }
    }

    public static final class Pallet extends Kind {
        @Override
        public int costOf(int weight) {
            return weight + 225;
        }
    }
}
