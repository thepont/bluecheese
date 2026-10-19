use std::collections::BTreeMap;
use std::path::PathBuf;

use clap::Parser;

/// Detect Fowler code smells using static analysis.
#[derive(Parser)]
#[command(name = "bluecheese", version, about)]
struct Args {
    /// The folder to detect code smells in.
    #[arg(short = 'f', long = "folder")]
    folder: PathBuf,

    /// The code smell to detect.
    #[arg(short = 's', long = "smell")]
    smell: String,

    /// Report format.
    #[arg(short = 'r', long = "report-format", default_value = "json")]
    format: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let project = bluecheese::lang::load_project(&args.folder)?;

    // An empty report from a working analyser and an empty report from an
    // analyser that understood nothing are the same bytes. Refuse to produce
    // one without saying which happened.
    if project.files.is_empty() {
        let tail = if project.skipped.is_empty() {
            String::new()
        } else {
            format!("; saw {}", describe(&project.skipped))
        };
        anyhow::bail!(
            "no source files found under {} (understood extensions: {}){tail}",
            args.folder.display(),
            bluecheese::lang::supported_extensions().join(", ")
        );
    }

    let mut counts: BTreeMap<bluecheese::lang::LanguageId, usize> = BTreeMap::new();
    for file in &project.files {
        *counts.entry(file.language).or_default() += 1;
    }
    let summary = counts
        .iter()
        .map(|(language, count)| format!("{count} {language:?}"))
        .collect::<Vec<_>>()
        .join(", ");
    eprintln!("analysed {} files ({summary})", project.files.len());

    // Files with no adapter are invisible in the report otherwise: pointing a
    // Rust-only tool at a half-JavaScript project would look like a clean run.
    if !project.skipped.is_empty() {
        eprintln!(
            "skipped {} files with no adapter: {}",
            project.skipped.values().sum::<usize>(),
            describe(&project.skipped)
        );
    }

    let findings = bluecheese::detect(&project.files, &args.smell);
    println!("{}", serde_json::to_string_pretty(&findings)?);
    Ok(())
}

fn describe(by_extension: &BTreeMap<String, usize>) -> String {
    by_extension
        .iter()
        .map(|(extension, count)| format!("{count} .{extension}"))
        .collect::<Vec<_>>()
        .join(", ")
}
