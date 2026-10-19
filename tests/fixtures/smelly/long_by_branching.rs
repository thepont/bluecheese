// SMELLY ON PURPOSE, and frozen: see tests/smelly_rust.rs for what this pins.
//
// Branching alone: fifteen `&&` in one expression put complexity at 16 against a
// limit of 15, while the body holds a single expression, so the size limit
// cannot be what triggers the finding.
//
// This is also where the Rust metric deliberately parts company with the Java
// one. Spoon counts a logical operator only when an enclosing loop or branch
// sits between it and the method; here a `&&` counts wherever it appears, which
// is why nothing in this file is wrapped in an `if`.

pub fn in_range(a: i32) -> bool {
    a > 0
        && a < 1
        && a < 2
        && a < 3
        && a < 4
        && a < 5
        && a < 6
        && a < 7
        && a < 8
        && a < 9
        && a < 10
        && a < 11
        && a < 12
        && a < 13
        && a < 14
        && a < 15
}
