// The control for the Rust half of the corpus, and frozen: see
// tests/smelly_rust.rs for what this pins.
//
// Not smelly. Each shape the adapter has to map is here in its short form, so
// that a detector which reports everything is caught as surely as one which
// reports nothing.

pub struct Tiny {
    pub value: i32,
}

impl Tiny {
    pub fn doubled(&self) -> i32 {
        self.value * 2
    }
}

pub trait Named {
    fn name(&self) -> &'static str {
        "tiny"
    }
}

pub fn chooses(a: i32) -> &'static str {
    match a {
        0 => "zero",
        1 => "one",
        _ => "many",
    }
}
