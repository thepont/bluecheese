// SMELLY ON PURPOSE, and frozen: see tests/smelly_rust.rs for what this pins.
//
// The trap: a genuinely long function that must NOT be reported, because it
// lives one level down in a module. `types()` returns the file and its
// top-level type items, and the file's free functions are its *direct*
// children, so nothing inside a nested module is ever reached.
//
// This is the Rust reading of the same quirk as the Java corpus's nested type:
// Spoon's getAllTypes() returns top-level types only.

pub mod inner {
    pub fn really_long() -> i32 {
        let mut total = 0;
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
        total
    }
}
