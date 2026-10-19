// SMELLY ON PURPOSE, and frozen: see tests/smelly_rust.rs for what this pins.
//
// The counterpart of the Java corpus's constructor trap, and deliberately the
// opposite of it: Rust has no constructor in Spoon's sense, so a long `fn new`
// is an ordinary method of its type and is reported like any other. A reader who
// has just been told that a long Java constructor is invisible must not conclude
// that Rust's naming convention for one hides anything.

pub struct Config {
    pub limit: i32,
}

impl Config {
    pub fn new(seeds: &[i32]) -> Self {
        let mut limit = 0;
        limit += seeds[0];
        limit += seeds[1];
        limit += seeds[2];
        limit += seeds[3];
        limit += seeds[4];
        limit += seeds[5];
        limit += seeds[6];
        limit += seeds[7];
        limit += seeds[8];
        limit += seeds[9];
        limit += seeds[10];
        limit += seeds[11];
        limit += seeds[12];
        limit += seeds[13];
        limit += seeds[14];
        limit += seeds[15];
        limit += seeds[16];
        limit += seeds[17];
        limit += seeds[18];
        limit += seeds[19];
        limit += seeds[20];
        limit += seeds[21];
        limit += seeds[22];
        limit += seeds[23];
        limit += seeds[24];
        limit += seeds[25];
        limit += seeds[26];
        limit += seeds[27];
        limit += seeds[28];
        limit += seeds[29];
        Config { limit }
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }
}
