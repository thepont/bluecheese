// SMELLY ON PURPOSE, and frozen: see tests/refactored.rs for what this pins.
//
// The before half of a pair, and the book's Switch Statements smell in the shape
// Rust writes it: a `match` over a kind that decides what the work is. It is long
// enough to be reported for size as well, which is usually what sends a reader
// looking for the match in the first place.
//
// The three arms add 1..9, 11..19 and 21..29 to the weight: 45, 135 and 225, the
// same three numbers as the Java corpus's pair of the same name. Its twin in
// tests/fixtures/refactored/ computes them without a match.
//
// Thirty-three statements against a limit of 30, branching four ways against a
// limit of 15, so it is size that reports it rather than the match.

pub enum Kind {
    Letter,
    Parcel,
    Pallet,
}

pub struct SwitchOnKind;

impl SwitchOnKind {
    pub fn cost(kind: Kind, weight: i32) -> i32 {
        let mut cost = 0;
        match kind {
            Kind::Letter => {
                cost += weight;
                cost += 1;
                cost += 2;
                cost += 3;
                cost += 4;
                cost += 5;
                cost += 6;
                cost += 7;
                cost += 8;
                cost += 9;
            }
            Kind::Parcel => {
                cost += weight;
                cost += 11;
                cost += 12;
                cost += 13;
                cost += 14;
                cost += 15;
                cost += 16;
                cost += 17;
                cost += 18;
                cost += 19;
            }
            Kind::Pallet => {
                cost += weight;
                cost += 21;
                cost += 22;
                cost += 23;
                cost += 24;
                cost += 25;
                cost += 26;
                cost += 27;
                cost += 28;
                cost += 29;
            }
        }
        cost
    }
}
