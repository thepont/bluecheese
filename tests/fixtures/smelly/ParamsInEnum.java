// SMELLY ON PURPOSE, and frozen: see tests/smelly_java.rs for what this pins.
//
// The trap: Spoon only asks a CtClass for constructors, and an enum is one, so
// an enum's constructor is read exactly as a class's is. An adapter that treated
// only `class` declarations as classes would miss this constructor, and nothing
// in the SmellBench corpus would notice: its constructors all sit in classes.
package smelly;

public enum ParamsInEnum {
    ONLY(1, 2, 3, 4, 5, 6);

    ParamsInEnum(int a, int b, int c, int d, int e, int f) {
    }
}
