// Two statements plus a tail expression: below the three-statement threshold.
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

struct AtThreshold;

impl AtThreshold {
    fn at_threshold(value: i32) -> i32 {
        let next = value + 1;
        let doubled = next * 2;
        let result = doubled - 3;
        result
    }

    fn same_at_threshold(value: i32) -> i32 {
        let next = value + 1;
        let doubled = next * 2;
        let result = doubled - 3;
        result
    }
}

struct AboveThreshold;

impl AboveThreshold {
    fn above_threshold(value: i32) -> i32 {
        let next = value + 1;
        let doubled = next * 2;
        let adjusted = doubled - 3;
        let result = adjusted / 4;
        result
    }

    fn same_above_threshold(value: i32) -> i32 {
        let next = value + 1;
        let doubled = next * 2;
        let adjusted = doubled - 3;
        let result = adjusted / 4;
        result
    }
}

// Both meet the minimum, but have different statement/expression counts.
struct DifferentCounts;

impl DifferentCounts {
    fn three_statements(value: i32) -> i32 {
        let next = value + 1;
        let doubled = next * 2;
        let result = doubled - 3;
        result
    }

    fn four_statements(value: i32) -> i32 {
        let next = value + 1;
        let doubled = next * 2;
        let adjusted = doubled - 3;
        let result = adjusted / 4;
        result
    }
}

// Matching counts in separate impl blocks do not form a same-owner pair.
struct SeparateImpls;

impl SeparateImpls {
    fn first_impl(value: i32) -> i32 {
        let next = value + 1;
        let doubled = next * 2;
        let result = doubled - 3;
        result
    }
}

impl SeparateImpls {
    fn second_impl(value: i32) -> i32 {
        let next = value + 1;
        let doubled = next * 2;
        let result = doubled - 3;
        result
    }
}
