struct Sample;

impl Sample {
    fn first(value: i32) -> i32 {
        let next = value + 1;
        let doubled = next * 2;
        doubled
    }

    fn second(value: i32) -> i32 {
        let next = value + 1;
        let doubled = next * 2;
        doubled
    }
}
