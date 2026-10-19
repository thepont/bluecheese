mod common;

use bluecheese::finding::Finding;
use common::parse;

fn scan(source: &str) -> Vec<Finding> {
    let file = parse("calculator.java", source);
    bluecheese::detect(&[file], "temp-field").expect("run Temporary Field")
}

#[test]
fn reports_fields_used_only_inside_a_conditional_branch() {
    let findings = scan(
        "class Calculator {\n\
            int scratch;\n\
            int calculate(boolean shortcut) {\n\
                if (shortcut) { scratch = 1; return scratch; }\n\
                return 0;\n\
            }\n\
        }",
    );

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].class_name, "Calculator");
    assert_eq!(findings[0].line_numbers, [2]);
}

#[test]
fn ignores_fields_used_unconditionally() {
    let findings = scan(
        "class Calculator {\n\
            int total;\n\
            int calculate() { total += 1; return total; }\n\
        }",
    );

    assert!(findings.is_empty());
}
