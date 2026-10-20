mod common;

use bluecheese::finding::Finding;
use common::parse;

fn scan(name: &str, source: &str) -> Vec<Finding> {
    let file = parse(name, source);
    bluecheese::detect(&[file], "switch-stmts").expect("run Switch Statements")
}

#[test]
fn javascript_and_typescript_switch_statements_are_reported() {
    for (name, source) in [
        (
            "choice.js",
            "class Choice { pick(value) { switch (value) { case 1: return 'one'; default: return 'other'; } } }",
        ),
        (
            "choice.ts",
            "class Choice { pick(value: number) { switch (value) { case 1: return 'one'; default: return 'other'; } } }",
        ),
    ] {
        let findings = scan(name, source);
        assert_eq!(findings.len(), 1, "{name}");
        assert_eq!(findings[0].class_name, "Choice", "{name}");
        assert_eq!(findings[0].line_numbers, [1], "{name}");
    }
}

#[test]
fn exhaustive_unit_enum_strategy_forwarding_is_not_a_conditional_algorithm() {
    assert!(
        scan(
            "choice.rs",
            "enum Backend { A, B, C }
        fn select(backend: Backend, input: Input) -> Output {
            match backend { Backend::A => first(input), Backend::B | Backend::C => second(input) }
        }"
        )
        .is_empty()
    );
}

#[test]
fn strategy_dispatch_does_not_exempt_branch_specific_logic() {
    for arms in [
        "Backend::A => first(input + 1), Backend::B => second(input)",
        "Backend::A => first(input), Backend::B => second(other)",
        "Backend::A if ready => first(input), Backend::B => second(input)",
        "Backend::A => first(input), _ => second(input)",
        "Backend::A => { prepare(); first(input) }, Backend::B => second(input)",
        "Backend::A => input + 1, Backend::B => input + 2",
    ] {
        assert_eq!(
            scan(
                "choice.rs",
                &format!(
                    "enum Backend {{ A, B }}
            fn select(backend: Backend, input: Input) -> Output {{ match backend {{ {arms} }} }}"
                )
            )
            .len(),
            1,
            "{arms}"
        );
    }
}

#[test]
fn forwarding_without_a_known_exhaustive_unit_enum_is_still_reported() {
    for declaration in [
        "",
        "enum Backend { A, B, C }",
        "enum Backend { A(Data), B }",
    ] {
        assert_eq!(
            scan(
                "choice.rs",
                &format!(
                    "{declaration}
            fn select(backend: Backend, input: Input) -> Output {{
                match backend {{ Backend::A => first(input), Backend::B => second(input) }}
            }}"
                )
            )
            .len(),
            1,
            "{declaration}"
        );
    }
}

#[test]
fn strategy_dispatch_requires_an_unambiguous_enum_declaration() {
    let files = [
        parse("one.rs", "enum Backend { A, B }"),
        parse("two.rs", "enum Backend { A, B }"),
        parse(
            "choice.rs",
            "fn select(backend: Backend, input: Input) -> Output {
            match backend { Backend::A => first(input), Backend::B => second(input) }
        }",
        ),
    ];
    assert_eq!(bluecheese::detect(&files, "switch-stmts").unwrap().len(), 1);
}

#[test]
fn strategy_dispatch_can_use_an_enum_declared_in_another_file() {
    let files = [
        parse("backend.rs", "enum Backend { A, B }"),
        parse(
            "choice.rs",
            "fn select(backend: Backend, input: Input) -> Output {
            match backend { Backend::A => first(input), Backend::B => second(input) }
        }",
        ),
    ];
    assert!(
        bluecheese::detect(&files, "switch-stmts")
            .unwrap()
            .is_empty()
    );
}
