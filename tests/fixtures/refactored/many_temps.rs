// The after half of the pair whose before half is
// tests/fixtures/smelly/many_temps.rs, and frozen.
//
// Replace Method with Method Object: the locals become fields of a new type, and
// the method that used to hold the body is one line.
//
// This refactoring does NOT silence the detector, and this fixture exists to make
// sure nobody makes it look as though it does. The long body has moved into the
// new type's compute(), where it is still long — thirty-three statements against
// the same limit of 30 — and it is now reported under ManyTempsCalculator
// instead of ManyTemps. The book is explicit that the method
// object is a step and not the cure: the cure is the Extract Method you then
// apply inside compute(). A detector that went quiet here would be lying about
// where the smell went.

pub struct ManyTemps;

impl ManyTemps {
    pub fn average(values: &[f64]) -> f64 {
        ManyTempsCalculator::new(values).compute()
    }
}

struct ManyTempsCalculator {
    values: Vec<f64>,
    sum: f64,
    count: i32,
    highest: f64,
    lowest: f64,
    spread: f64,
}

impl ManyTempsCalculator {
    fn new(values: &[f64]) -> Self {
        ManyTempsCalculator {
            values: values.to_vec(),
            sum: 0.0,
            count: 0,
            highest: 0.0,
            lowest: 0.0,
            spread: 0.0,
        }
    }

    fn compute(&mut self) -> f64 {
        self.sum += self.values[0];
        self.sum += self.values[1];
        self.sum += self.values[2];
        self.sum += self.values[3];
        self.sum += self.values[4];
        self.sum += self.values[5];
        self.sum += self.values[6];
        self.sum += self.values[7];
        self.sum += self.values[8];
        self.sum += self.values[9];
        self.sum += self.values[10];
        self.sum += self.values[11];
        self.sum += self.values[12];
        self.count += 1;
        self.count += 1;
        self.count += 1;
        self.count += 1;
        self.count += 1;
        self.count += 1;
        self.count += 1;
        self.count += 1;
        self.highest = self.values[0];
        self.highest = self.values[1];
        self.highest = self.values[2];
        self.highest = self.values[3];
        self.lowest = self.values[4];
        self.lowest = self.values[5];
        self.lowest = self.values[6];
        self.lowest = self.values[7];
        self.spread = self.highest - self.lowest;
        self.spread += 1.0;
        self.spread += 2.0;
        (self.sum / self.count as f64) + self.spread
    }
}
