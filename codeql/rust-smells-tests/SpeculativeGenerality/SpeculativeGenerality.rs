trait UnusedTrait {}

trait UsedTrait { fn run(&self); }

struct Worker;

impl UsedTrait for Worker {
    fn run(&self) {}
}
