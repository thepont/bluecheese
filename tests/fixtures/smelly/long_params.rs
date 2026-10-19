// SMELLY ON PURPOSE, and frozen: see tests/smelly_rust.rs for what this pins.
//
// Four parameters is one too many; three is not. The receiver is not a
// parameter: `&self` is not something a caller writes, and Java's `this` is not
// counted either, so leaving it in would make a four-parameter method look like
// a five-parameter one on this side only.
//
// There is no reference implementation for Rust smells, so this pins *our*
// definition, and it deliberately agrees with the Java fixtures about where the
// limit is.

pub struct LongParams {
    a: i32,
}

impl LongParams {
    pub fn three(&self, a: i32, b: i32, c: i32) -> i32 {
        a + b + c
    }

    pub fn four(&self, a: i32, b: i32, c: i32, d: i32) -> i32 {
        a + b + c + d
    }
}

pub fn free_four(a: i32, b: i32, c: i32, d: i32) -> i32 {
    a + b + c + d
}
