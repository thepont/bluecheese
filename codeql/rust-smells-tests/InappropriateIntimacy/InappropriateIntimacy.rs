struct Other {
    secret: String,
}

struct Intimate {
    other: Other,
}

impl Intimate {
    fn read(&self) -> &String { &self.other.secret }
    fn own_field(&self) -> &Other { &self.other }
}
