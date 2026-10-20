trait FourParameters {
    fn declared(&self, a: i32, b: i32, c: i32, d: i32);
}

struct Example;

impl Example {
    fn at_limit(&self, a: i32, b: i32, c: i32) {}

    fn over_limit(&self, a: i32, b: i32, c: i32, d: i32) {}
}

impl FourParameters for Example {
    fn declared(&self, a: i32, b: i32, c: i32, d: i32) {}
}

fn free_function(a: i32, b: i32, c: i32, d: i32) {}
