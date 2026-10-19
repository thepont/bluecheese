// SMELLY ON PURPOSE, and frozen: see tests/smelly_java.rs for what this pins.
//
// The trap: a record's compact constructor declares no parameter list, and Java
// gives it the record's components, so this one takes six parameters and is
// reported. A record that declares no constructor has none in the model at all
// — Spoon's implicit canonical constructor has no position — so it is not
// reported, even though it too takes six components.
package smelly;

public record RecordCtor(int a, int b, int c, int d, int e, int f) {

    public RecordCtor {
    }
}

record ImplicitCtor(int a, int b, int c, int d, int e, int f) {
}
