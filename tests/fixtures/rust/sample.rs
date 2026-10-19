// Fixture for the Rust adapter. There is no reference implementation for Rust,
// so the expectations in tests/rust_adapter.rs are reasoned by hand from this
// source, not proven against another tool.

pub struct Analyzer {
    total: i64,
}

impl Analyzer {
    pub fn new() -> Self {
        Analyzer { total: 0 }
    }

    pub fn total(&self) -> i64 {
        self.total
    }

    pub fn accumulate(&mut self, values: &[i64]) {
        let mut running = 0;
        let mut count = 0;
        for value in values {
            if *value > 0 {
                let doubled = *value * 2;
                running += doubled;
                count += 1;
            } else if *value < 0 {
                let halved = *value / 2;
                running += halved;
                count += 1;
            } else {
                count += 1;
            }
        }
        let average = if count == 0 { 0 } else { running / count };
        let mut bucket = 0;
        while bucket < 10 {
            bucket += 1;
        }
        loop {
            if bucket == 0 {
                break;
            }
            bucket -= 1;
        }
        match average {
            0 => self.total += 0,
            1 => self.total += 1,
            2 => self.total += 2,
            _ => self.total += average,
        }
        self.total = self.total.wrapping_add(running);
        self.total = self.total.wrapping_add(count);
        self.total = self.total.wrapping_add(bucket);
        self.total = self.total.wrapping_add(average);
        let check = self.total > 0;
        if check {
            self.total += 1;
        }
        let label = if check { "positive" } else { "zero" };
        self.total += label.len() as i64;
    }
}

pub enum Status {
    Idle,
    Busy,
}

pub fn free_short() -> bool {
    true
}
