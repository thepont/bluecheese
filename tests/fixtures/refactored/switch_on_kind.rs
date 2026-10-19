// The after half of the pair whose before half is
// tests/fixtures/smelly/switch_on_kind.rs, and frozen.
//
// Replace Conditional with Polymorphism, in the shape Rust writes it: the kind
// becomes a trait and each variant a type that answers for itself, instead of an
// enum and a match. 45, 135 and 225 are exactly the sums the three arms in the
// twin add up to, which is what makes this a refactoring rather than a rewrite.
//
// The method that hosted the match is now one line, and `cost_of` in the trait
// is a signature with no body: it is a method of the trait, and there is nothing
// in it to measure.

pub trait Kind {
    fn cost_of(&self, weight: i32) -> i32;
}

pub struct Letter;
pub struct Parcel;
pub struct Pallet;

impl Kind for Letter {
    fn cost_of(&self, weight: i32) -> i32 {
        weight + 45
    }
}

impl Kind for Parcel {
    fn cost_of(&self, weight: i32) -> i32 {
        weight + 135
    }
}

impl Kind for Pallet {
    fn cost_of(&self, weight: i32) -> i32 {
        weight + 225
    }
}

pub fn cost(kind: &dyn Kind, weight: i32) -> i32 {
    kind.cost_of(weight)
}
