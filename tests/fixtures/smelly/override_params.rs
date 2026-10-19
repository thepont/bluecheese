// SMELLY ON PURPOSE, and frozen: see tests/smelly_rust.rs for what this pins.
//
// The trap, and the Rust half of OverrideParams.java: a function written inside
// `impl Trait for Type` restates the trait's signature, so it is not reported,
// while the trait's own declaration is. A default method declared in the trait
// has a body but is still the declaration, so it is reported with the trait.
//
// This is our definition, not Spoon's: Rust has no annotation to read, so the
// enclosing `impl` block is what says whether the signature was chosen here.

pub trait FourArgs {
    fn four(&self, a: i32, b: i32, c: i32, d: i32) -> i32;

    fn default_four(&self, a: i32, b: i32, c: i32, d: i32) -> i32 {
        a + b + c + d
    }
}

pub struct Args;

impl FourArgs for Args {
    fn four(&self, a: i32, b: i32, c: i32, d: i32) -> i32 {
        a + b + c + d
    }
}

impl Args {
    pub fn own(&self, a: i32, b: i32, c: i32, d: i32) -> i32 {
        a + b + c + d
    }
}
