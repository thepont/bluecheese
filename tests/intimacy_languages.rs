mod common;

use bluecheese::finding::Finding;
use common::parse;

fn scan(name: &str, source: &str) -> Vec<Finding> {
    let file = parse(name, source);
    bluecheese::detect(&[file], "intimacy").expect("run Inappropriate Intimacy")
}

#[test]
fn rust_external_field_accesses_are_reported() {
    let findings = scan(
        "access.rs",
        "struct Guest { secret: i32 }\nstruct Reader;\nimpl Reader { fn read(&self, guest: &Guest) -> i32 { guest.secret } }\n",
    );
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].class_name, "Reader");
    assert_eq!(findings[0].line_numbers, [3]);
}

#[test]
fn javascript_and_typescript_external_field_accesses_are_reported() {
    for (name, source) in [
        ("access.js", "class Guest { secret = 1; } class Reader { read() { const guest = new Guest(); return guest.secret; } }"),
        ("access.ts", "class Guest { secret: number; } class Reader { read(guest: Guest) { return guest.secret; } }"),
    ] {
        let findings = scan(name, source);
        assert_eq!(findings.len(), 1, "{name}");
        assert_eq!(findings[0].class_name, "Reader", "{name}");
        assert_eq!(findings[0].line_numbers, [1], "{name}");
    }
}
