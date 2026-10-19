mod common;

use bluecheese::finding::Finding;
use common::{file, parse};

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

#[test]
fn matches_smellbench_monthly_calculation_and_refactoring() {
    let bad = file("smellbench/main/java/tempfield/case1", "TFBE1.java");
    let findings = bluecheese::detect(&[bad], "temp-field").expect("run detector");
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].class_name, "LoanBad");
    assert_eq!(findings[0].line_numbers, [8, 9]);

    let good = file("smellbench/main/java/tempfield/case1", "TFGE1.java");
    let findings = bluecheese::detect(&[good], "temp-field").expect("run detector");
    assert!(findings.is_empty());
}

#[test]
fn matches_nullable_field_and_solver_smellbench_examples() {
    let bad = file("smellbench/main/java/tempfield/case2", "TFBE2.java");
    let findings = bluecheese::detect(&[bad], "temp-field").expect("run detector");
    assert!(
        findings
            .iter()
            .any(|finding| finding.class_name == "VerizonUserManagement"),
        "nullable cable customer collection should be reported"
    );

    let good = file("smellbench/main/java/tempfield/case2", "TFGE2.java");
    let findings = bluecheese::detect(&[good], "temp-field").expect("run detector");
    assert!(findings.is_empty(), "eager Null Object collection is clean");

    let bad = file("smellbench/main/java/tempfield/case3", "TFBE3.java");
    let findings = bluecheese::detect(&[bad], "temp-field").expect("run detector");
    assert!(
        findings
            .iter()
            .any(|finding| finding.class_name == "PolynomialSolver"),
        "quadratic and cubic coefficients should be reported"
    );

    let good = file("smellbench/main/java/tempfield/case3", "TFGE3.java");
    let findings = bluecheese::detect(&[good], "temp-field").expect("run detector");
    assert!(findings.is_empty(), "split solver classes are clean");
}
