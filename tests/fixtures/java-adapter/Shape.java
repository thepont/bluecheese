// Not smelly. This fixture exists for the shapes the adapter has to map, not for
// smells: annotations, a nested type, an enum, a record, an annotation type and
// an interface with a default method.
package adapter;

@interface Marker {
}

@Marker
@Deprecated
public class Shape {

    private int sides;

    // A constructor: excluded from the methods of its type.
    public Shape() {
        this.sides = 0;
    }

    public int sides() {
        return sides;
    }

    // Nested: Spoon's CtModel.getAllTypes() returns top-level types only, so
    // this must never appear as a type of its own.
    public static class Inner {
        public void hide() {
        }
    }

    // Enum members live under enum_body_declarations, one level deeper than a
    // class's methods.
    public enum Kind {
        FLAT;

        public boolean isFlat() {
            return true;
        }
    }

    public record Point(int x, int y) {
        public int sum() {
            return x + y;
        }
    }
}

// The only modifier is an annotation, so the declaration starts at the next
// token rather than on the annotated line.
@Marker
class Bare {
}

interface Measurable {

    // Abstract: listed as a method, but it has no body to measure.
    int size();

    default int twice() {
        return size() * 2;
    }
}
