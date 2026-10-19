// SMELLY ON PURPOSE, and frozen: see tests/smelly_rust.rs for what this pins.
//
// Rust keeps methods out of the type, in `impl` blocks that may sit anywhere in
// the file and need not follow the type they target. A long method there is
// reported under the name of the type it targets, not under its own name.
//
// Unlike the Java corpus, there is no constructor trap to pair this with: Rust
// has no constructor in Spoon's sense, so a long `fn new` is an ordinary method
// and is reported like any other.

pub struct Accumulator {
    total: i64,
}

impl Accumulator {
    pub fn new() -> Self {
        Accumulator { total: 0 }
    }

    pub fn short(&self) -> i64 {
        self.total
    }

    pub fn from_weights(weights: &[i64]) -> Self {
        let mut total = 0;
        total += weights[0];
        total += weights[1];
        total += weights[2];
        total += weights[3];
        total += weights[4];
        total += weights[5];
        total += weights[6];
        total += weights[7];
        total += weights[8];
        total += weights[9];
        total += weights[10];
        total += weights[11];
        total += weights[12];
        total += weights[13];
        total += weights[14];
        total += weights[15];
        total += weights[16];
        total += weights[17];
        total += weights[18];
        total += weights[19];
        total += weights[20];
        total += weights[21];
        total += weights[22];
        total += weights[23];
        total += weights[24];
        total += weights[25];
        total += weights[26];
        total += weights[27];
        total += weights[28];
        total += weights[29];
        Accumulator { total }
    }
}
