//! Not smelly. Adapter behaviours that the smell corpus does not reach: trait
//! default methods, which operators count as decisions, and the line a
//! declaration is reported on.

#![allow(dead_code)]

/// The trait declares one method that has a body and one that does not. Only the
/// one with a body can be measured, so only that one is a method here.
pub trait Volume {
    fn size(&self) -> usize;

    fn doubled(&self) -> usize {
        self.size() * 2
    }
}

/// Attributes sit above the item, so the reported line must be the item's own.
#[derive(Debug, Default)]
#[allow(clippy::len_without_is_empty)]
pub struct Crate {
    items: Vec<usize>,
}

impl Crate {
    pub fn total(&self) -> usize {
        self.items.iter().sum()
    }

    /// `&&` and `||` each add a branch; `&` is arithmetic and does not.
    pub fn fits(&self, a: bool, b: bool) -> bool {
        let mask = 1 & 2;
        (a && b) || mask == 0
    }
}
