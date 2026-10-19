// The after half of the pair whose before half is
// tests/fixtures/smelly/extract_me.rs, and frozen.
//
// Extract Method: the same work, in two short methods that each say what they
// do. Both halves leave the same total in the field and return the same labels
// for the fifteen values they are written for, so the claim that only the
// division into methods changed is one a reader can check by reading the two
// files side by side. Nothing here is compiled or run.

pub struct ExtractMe {
    total: f64,
}

impl ExtractMe {
    pub fn build(&mut self, values: &[f64], names: &[&str]) -> String {
        self.sum(values);
        labels(names)
    }

    fn sum(&mut self, values: &[f64]) {
        self.total = 0.0;
        for value in values.iter().take(15) {
            self.total += *value;
        }
    }

    pub fn total(&self) -> f64 {
        self.total
    }
}

fn labels(names: &[&str]) -> String {
    let mut report = String::new();
    for name in names.iter().take(15) {
        report.push_str(name);
    }
    report
}
