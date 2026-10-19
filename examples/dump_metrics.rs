//! Debug helper: dump per-method LLOC and cyclomatic complexity in the same
//! TSV shape as the Java `Probe`, so the two can be diffed directly.
//!
//! Run: `cargo run --example dump_metrics -- <dir>`

use bluecheese::{java, metrics, types};

fn main() {
    let root = std::env::args().nth(1).expect("usage: dump_metrics <dir>");
    let files = java::load_project(std::path::Path::new(&root)).expect("load project");
    for file in &files {
        for ty in types::all_types(file) {
            let Some(class_name) = types::simple_name(file, ty) else {
                continue;
            };
            for m in types::methods(ty) {
                let Some(body) = m.child_by_field_name("body") else {
                    continue;
                };
                println!(
                    "{}\t{}\t{}\t{}\t{}\t{}\t{}",
                    file.path.display(),
                    class_name,
                    m.child_by_field_name("name")
                        .map(|n| file.text(n))
                        .unwrap_or(""),
                    types::decl_line(file, m),
                    file.line(body),
                    metrics::lloc(file, body),
                    metrics::cyclomatic_complexity(file, m),
                );
            }
        }
    }
}
