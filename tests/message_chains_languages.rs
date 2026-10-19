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
        ("chains.js", "class Client { run(customer) { customer.getAddress().getCountry().getName(); } }"),
        ("chains.ts", "class Client { run(customer: Client) { customer.getAddress().getCountry().getName(); } }"),
    ] {
        let findings = scan(name, source);
        assert_eq!(findings.len(), 1, "{name}");
        assert_eq!(findings[0].class_name, "Client", "{name}");
        assert_eq!(findings[0].line_numbers, [1], "{name}");
    }
}

#[test]
fn non_accessor_fluent_chains_keep_the_existing_rule() {
    assert!(scan(
        "builder.rs",
        "fn build() { builder.append().finish().write(); }"
    )
    .is_empty());
    assert!(scan(
        "builder.js",
        "function build() { builder.append().finish().write(); }"
    )
    .is_empty());
}
