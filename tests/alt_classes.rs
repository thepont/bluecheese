mod common;

use bluecheese::finding::Finding;
use common::parse;

fn scan(source: &str) -> Vec<Finding> {
    let file = parse("converters.java", source);
    bluecheese::detect(&[file], "alt-classes").expect("run Alternative Classes")
}

#[test]
fn reports_classes_with_two_methods_of_the_same_shape() {
    let findings = scan(
        "class Celsius {\n\
            double convert(double value) { return value * 1.8 + 32; }\n\
            double limit(double value) { return Math.max(0, value); }\n\
        }\n\
        class Fahrenheit {\n\
            double transform(double amount) { return amount * 2.0 + 10; }\n\
            double bound(double amount) { return Math.max(1, amount); }\n\
        }",
    );

    assert_eq!(
        findings
            .iter()
            .map(|finding| finding.class_name.as_str())
            .collect::<Vec<_>>(),
        ["Celsius", "Fahrenheit"]
    );
}

#[test]
fn does_not_report_classes_with_unrelated_method_shapes() {
    let findings = scan(
        "class Printer { void print(String text) { System.out.println(text); } }\n\
        class Counter { int count(int start) { return start + 1; } }",
    );

    assert!(findings.is_empty());
}
