use std::path::Path;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    assert_eq!(args.len(), 3, "probe ROOT MODE");
    let root = Path::new(&args[1]);
    let project = bluecheese::lang::load_project(root).expect("load pinned public source");
    let errors: Vec<_> = project
        .files
        .iter()
        .filter(|f| f.root().has_error())
        .map(|f| &f.rel)
        .collect();
    let result = match args[2].as_str() {
        "history" => bluecheese::history::detect(root, &project.files),
        "source" => project.detect_source("shotgun"),
        "reference" => bluecheese::detect(&project.files, "shotgun"),
        "default" => project.detect("shotgun"),
        _ => panic!("unknown mode"),
    };
    let (findings, error) = match result {
        Ok(findings) => (findings, None),
        Err(error) => (Vec::new(), Some(format!("{error:#}"))),
    };
    let findings: Vec<_> = findings
        .into_iter()
        .map(|finding| {
            serde_json::json!({
                "smellName": finding.smell_name, "relativeFilePath": finding.relative_file_path,
                "className": finding.class_name, "lineNumbers": finding.line_numbers,
                "additionalInfo": finding.additional_info,
            })
        })
        .collect();
    let output = serde_json::json!({
        "root": args[1], "mode": args[2], "source_files": project.files.len(),
        "languages": project.language_summary(), "parse_errors": errors,
        "findings": findings, "error": error,
    });
    println!("{}", serde_json::to_string_pretty(&output).unwrap());
}
