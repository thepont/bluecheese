mod common;

use bluecheese::finding::Finding;
use common::parse;

fn scan(source: &str) -> Vec<Finding> {
    let file = parse("invoice.java", source);
    bluecheese::detect(&[file], "feature-envy").expect("run Feature Envy")
}

#[test]
fn reports_a_method_reading_many_fields_from_one_other_object() {
    let findings = scan(
        "class Invoice {\n\
            double total(Customer customer) {\n\
                return customer.balance + customer.discount + customer.tax;\n\
            }\n\
        }",
    );

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].class_name, "Invoice");
    assert_eq!(findings[0].line_numbers, [2]);
}

#[test]
fn ignores_methods_that_read_their_own_fields() {
    let findings = scan(
        "class Invoice {\n\
            int balance; int discount; int tax;\n\
            int total() { return this.balance + this.discount + this.tax; }\n\
        }",
    );

    assert!(findings.is_empty());
}
