// SMELLY ON PURPOSE, and frozen: see tests/refactored.rs for what this pins.
//
// The before half of a Replace Method with Method Object pair: a long body that
// leans on many locals, which is the shape the book uses to introduce that
// refactoring. The Java corpus has a pair of the same name with the same
// arithmetic; this is the same refactoring applied to Rust, not a translation.
//
// Thirty-eight statements against a limit of 30, and nothing branches.

pub struct ManyTemps;

impl ManyTemps {
    pub fn average(values: &[f64]) -> f64 {
        let mut sum = 0.0;
        let mut count = 0;
        let mut highest = 0.0;
        let mut lowest = 0.0;
        let mut spread = 0.0;
        sum += values[0];
        sum += values[1];
        sum += values[2];
        sum += values[3];
        sum += values[4];
        sum += values[5];
        sum += values[6];
        sum += values[7];
        sum += values[8];
        sum += values[9];
        sum += values[10];
        sum += values[11];
        sum += values[12];
        count += 1;
        count += 1;
        count += 1;
        count += 1;
        count += 1;
        count += 1;
        count += 1;
        count += 1;
        highest = values[0];
        highest = values[1];
        highest = values[2];
        highest = values[3];
        lowest = values[4];
        lowest = values[5];
        lowest = values[6];
        lowest = values[7];
        spread = highest - lowest;
        spread += 1.0;
        spread += 2.0;
        (sum / count as f64) + spread
    }
}
