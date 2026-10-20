use external::Client;

trait FirstExtension {
    fn load(&self) -> i32;
}

trait SecondExtension {
    fn load(&self) -> i32;
}

impl FirstExtension for Client {
    fn load(&self) -> i32 {
        let value = 1;
        value
    }
}

impl SecondExtension for Client {
    fn load(&self) -> i32 {
        let value = 1;
        value
    }
}
