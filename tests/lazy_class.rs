mod common;

use bluecheese::finding::Finding;
use common::parse;

fn scan(source: &str) -> Vec<Finding> {
    let file = parse("services.java", source);
    bluecheese::detect(&[file], "lazy-class").expect("run Lazy Class")
}

#[test]
fn reports_a_fieldless_class_with_one_forwarding_method() {
    let findings = scan(
        "class PassThrough {\n\
            Result execute(Request request) { return worker.execute(request); }\n\
        }",
    );

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].class_name, "PassThrough");
    assert_eq!(findings[0].line_numbers, [1]);
}

#[test]
fn ignores_classes_with_multiple_methods_or_state() {
    let findings = scan(
        "class Useful {\n\
            private int count;\n\
            int current() { return count; }\n\
            int next() { return count + 1; }\n\
        }",
    );

    assert!(findings.is_empty());
}
