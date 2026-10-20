struct Service;

impl Service {
    fn serve(&self) {}
}

struct FirstClient { service: Service }

impl FirstClient {
    fn first(&self) { self.service.serve(); }
}

struct SecondClient { service: Service }

impl SecondClient {
    fn second(&self) { self.service.serve(); }
}

struct OnlyClient;

impl OnlyClient {
    fn local(&self) { self.helper(); }
    fn helper(&self) {}
}
