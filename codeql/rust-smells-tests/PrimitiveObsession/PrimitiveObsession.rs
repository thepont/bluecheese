struct PrimitiveObsession {
    count: i32,
    balance: f64,
    active: bool,
    category: char,
}

struct NotPrimitiveObsession {
    count: i32,
    name: String,
    value: Option<i32>,
}
