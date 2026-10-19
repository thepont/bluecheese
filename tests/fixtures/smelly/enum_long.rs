// SMELLY ON PURPOSE, and frozen: see tests/smelly_rust.rs for what this pins.
//
// The counterpart of the Java corpus's enum fixture: a long method attached to a
// top-level enum IS reported, and the finding names the enum.
//
// What it pins here is not the Java quirk it resembles. Java's enum body sits
// under an extra `enum_body_declarations` level that the adapter has to descend
// through; Rust has no such level, and an `impl` block is found by the name of
// the type it targets whether that type is a struct, an enum, a union or a
// trait. The point of the pairing is that a type which is not a class is a type
// like any other.
//
// Thirty-three statements against a limit of 30, and nothing branches.

pub enum Rank {
    Low,
    Middle,
    High,
}

impl Rank {
    pub fn really_long(&self) -> i32 {
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
        total += 31;
        total
    }

    pub fn short(&self) -> i32 {
        1
    }
}
