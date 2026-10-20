pub fn choose(value: Option<i32>) -> i32 {
    match value {
        Some(number) => number,
        None => 0,
    }
}

pub fn plain(value: i32) -> i32 {
    value
}
