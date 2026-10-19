mod common;

use bluecheese::finding::Finding;
use common::parse;

fn scan(name: &str, source: &str) -> Vec<Finding> {
    let file = parse(name, source);
    bluecheese::detect(&[file], "spec-gen").expect("run Speculative Generality")
}

#[test]
fn unreferenced_rust_traits_and_typescript_abstract_types_are_reported() {
    for (name, source, expected) in [
        ("lonely.rs", "trait Lonely {}", "Lonely"),
        ("lonely.ts", "interface Lonely {}", "Lonely"),
        ("abstract.ts", "abstract class Lonely {}", "Lonely"),
    ] {
        let findings = scan(name, source);
        assert_eq!(findings.len(), 1, "{name}");
        assert_eq!(findings[0].class_name, expected, "{name}");
        assert_eq!(findings[0].line_numbers, [1], "{name}");
    }
}
