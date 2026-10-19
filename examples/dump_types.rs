//! Debug helper: list the types an adapter finds in a tree.
//!
//! Run: `cargo run --example dump_types -- <dir>`

use std::path::Path;

use bluecheese::lang::{load_project, model_for};

fn main() {
    let root = std::env::args().nth(1).expect("usage: dump_types <dir>");
    let project = load_project(Path::new(&root)).expect("load");

    for file in &project.files {
        let model = model_for(file.language);
        println!("== {}", file.rel);
        for ty in model.types(file) {
            println!(
                "   TYPE {} name={} body={}",
                ty.kind(),
                model.type_name(file, ty).unwrap_or("?"),
                model.body(ty).map(|b| b.kind()).unwrap_or("none"),
            );
        }
    }
}
