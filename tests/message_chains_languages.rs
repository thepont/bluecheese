mod common;

use bluecheese::finding::Finding;
use common::parse;

fn scan(name: &str, source: &str) -> Vec<Finding> {
    let file = parse(name, source);
    bluecheese::detect(&[file], "message-chains").expect("run Message Chains")
}

#[test]
fn rust_method_chains_are_reported_on_the_call_line() {
    let findings = scan(
        "chains.rs",
        "struct Client;\nimpl Client { fn run(&self, customer: &Client) { customer.get_address().get_country().get_name(); } }",
    );
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].class_name, "Client");
    assert_eq!(findings[0].line_numbers, [2]);
}

#[test]
fn javascript_and_typescript_accessor_chains_are_reported() {
    for (name, source) in [
        (
            "chains.js",
            "class Client { run(customer) { customer.getAddress().getCountry().getName(); } }",
        ),
        (
            "chains.ts",
            "class Client { run(customer: Client) { customer.getAddress().getCountry().getName(); } }",
        ),
    ] {
        let findings = scan(name, source);
        assert_eq!(findings.len(), 1, "{name}");
        assert_eq!(findings[0].class_name, "Client", "{name}");
        assert_eq!(findings[0].line_numbers, [1], "{name}");
    }
}

#[test]
fn non_accessor_fluent_chains_keep_the_existing_rule() {
    assert!(
        scan(
            "builder.rs",
            "fn build() { builder.append().finish().write(); }"
        )
        .is_empty()
    );
    assert!(
        scan(
            "builder.js",
            "function build() { builder.append().finish().write(); }"
        )
        .is_empty()
    );
}

#[test]
fn has_prefix_counts_only_in_javascript_and_typescript() {
    for (name, source, expected) in [
        (
            "chains.java",
            "class Client { void run() { customer.hasAccount().getOwner().getName(); } }",
            0,
        ),
        (
            "chains.rs",
            "fn run() { customer.hasAccount().getOwner().getName(); }",
            0,
        ),
        (
            "chains.js",
            "function run() { customer.hasAccount().getOwner().getName(); }",
            1,
        ),
        (
            "chains.ts",
            "function run() { customer.hasAccount().getOwner().getName(); }",
            1,
        ),
        (
            "chains.tsx",
            "function run() { customer.hasAccount().getOwner().getName(); }",
            1,
        ),
    ] {
        assert_eq!(scan(name, source).len(), expected, "{name}");
    }
}

#[test]
fn parentheses_preserve_chains_and_ordinary_calls_interrupt_them() {
    for (name, prefix) in [
        ("chains.rs", "fn run()"),
        ("chains.js", "function run()"),
        ("chains.ts", "function run()"),
        ("chains.tsx", "function run()"),
    ] {
        for (expression, expected) in [
            ("((customer.getAddress())).getCountry().getName()", 1),
            ("customer.getAddress().getCountry()", 0),
            ("customer.getAddress().save().getCountry().getName()", 0),
        ] {
            let source = format!("{prefix} {{ {expression}; }}");
            assert_eq!(scan(name, &source).len(), expected, "{name}: {expression}");
        }
    }
}
