//! Debug helper: dump per-method LLOC and cyclomatic complexity in the same
//! TSV shape as the Java `Probe`, so the two can be diffed directly.
//!
//! Run: `cargo run --example dump_metrics -- <dir>`

use std::path::Path;

use bluecheese::lang::{load_project, model_for};

fn main() {
    let root = std::env::args().nth(1).expect("usage: dump_metrics <dir>");
    let project = load_project(Path::new(&root)).expect("load project");

    for file in &project.files {
        let model = model_for(file.language);
        for ty in model.types(file) {
            let Some(class_name) = model.type_name(file.at(ty)) else {
                continue;
            };
            for method in model.methods(file.at(ty)) {
                let Some(body) = model.body(method) else {
                    continue;
                };
                let name = method
                    .child_by_field_name("name")
                    .map(|n| file.text(n))
                    .unwrap_or("");
                println!(
                    "{}\t{}\t{}\t{}\t{}\t{}\t{}",
                    file.path.display(),
                    class_name,
                    name,
                    model.decl_line(method),
                    file.line(body),
                    model.lloc(file.at(body)),
                    model.complexity(file.at(method)),
                );
            }
        }
    }
}
