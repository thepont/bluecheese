struct Batch;

impl Batch {
    fn create(&self, start: usize, end: usize) {}
    fn update(&self, start: usize, end: usize) {}
    fn compare(&self, start: &str, end: &str) {}
}

struct Other;

impl Other {
    fn first(&self, name: String, age: usize) {}
    fn second(&self, name: String, active: bool) {}
}
