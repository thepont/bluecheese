// SMELLY ON PURPOSE, and frozen: see tests/refactored.rs for what this pins.
//
// The before half of an Extract Method pair, and the book's primary answer to
// Long Method. One method does two jobs: it accumulates and it renders. Its twin
// in tests/fixtures/refactored/ does the same work in short methods.
//
// Thirty-three statements against a limit of 30, and nothing branches, so size is
// the only thing that can trigger the finding.

pub struct ExtractMe {
    total: f64,
}

impl ExtractMe {
    pub fn build(&mut self, values: &[f64], names: &[&str]) -> String {
        let mut report = String::new();
        self.total = 0.0;
        self.total += values[0];
        self.total += values[1];
        self.total += values[2];
        self.total += values[3];
        self.total += values[4];
        self.total += values[5];
        self.total += values[6];
        self.total += values[7];
        self.total += values[8];
        self.total += values[9];
        self.total += values[10];
        self.total += values[11];
        self.total += values[12];
        self.total += values[13];
        self.total += values[14];
        report.push_str(names[0]);
        report.push_str(names[1]);
        report.push_str(names[2]);
        report.push_str(names[3]);
        report.push_str(names[4]);
        report.push_str(names[5]);
        report.push_str(names[6]);
        report.push_str(names[7]);
        report.push_str(names[8]);
        report.push_str(names[9]);
        report.push_str(names[10]);
        report.push_str(names[11]);
        report.push_str(names[12]);
        report.push_str(names[13]);
        report.push_str(names[14]);
        report
    }

    pub fn total(&self) -> f64 {
        self.total
    }
}
