//! Debug helper: print the types found in a file and the parse tree roots.
//! Run: `cargo run --example dump_types -- <dir-or-file>`

use std::path::Path;

fn main() {
    let root = std::env::args().nth(1).expect("usage: dump_types <dir>");
    let files = bluecheese::java::load_project(Path::new(&root)).expect("load");
    for file in &files {
        println!("== {}", file.rel);
        for ty in bluecheese::types::all_types(file) {
            println!(
                "   TYPE {} name={} body={}",
                ty.kind(),
                bluecheese::types::simple_name(file, ty).unwrap_or("?"),
                bluecheese::types::body(ty)
                    .map(|b| b.kind())
                    .unwrap_or("none"),
            );
        }
    }
}
