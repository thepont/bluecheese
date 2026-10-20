trait Work {
    fn run(&self, value: u8) { let _ = value; }
    fn no_default(&self);
    fn ordinary(&self);
}

struct Worker;

impl Work for Worker {
    fn run(&self, value: u8) { todo!() }
    fn no_default(&self) { unimplemented!() }
    fn ordinary(&self) { let _ = 1; }
}
