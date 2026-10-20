struct Service;

impl Service {
    fn first(&self) {}
    fn second(&self) {}
}

struct Facade {
    service: Service,
}

impl Facade {
    fn first(&self) { self.service.first(); }
    fn second(&self) { self.service.second(); }
}
