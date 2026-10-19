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

/// How many extensions the skip notice names before summarising the rest. A
/// project that vendors an entire distribution can have hundreds, and a wall of
/// them hides the ones that matter.
const NAMED_EXTENSIONS: usize = 4;

/// Describe skipped files by extension, commonest first, without listing an
/// unbounded number of extensions.
fn describe(by_extension: &BTreeMap<String, usize>) -> String {
    let mut counts: Vec<(&String, &usize)> = by_extension.iter().collect();
    counts.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));

    let named = counts
        .iter()
        .take(NAMED_EXTENSIONS)
        .map(|(extension, count)| format!("{count} .{extension}"))
        .collect::<Vec<_>>()
        .join(", ");

    match counts.len().saturating_sub(NAMED_EXTENSIONS) {
        0 => named,
        remaining => format!("{named}, and {remaining} more extensions"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn counts(pairs: &[(&str, usize)]) -> BTreeMap<String, usize> {
        pairs
            .iter()
            .map(|(extension, count)| ((*extension).to_string(), *count))
            .collect()
    }

    #[test]
    fn names_the_commonest_extensions_and_summarises_the_rest() {
        let described = describe(&counts(&[
            ("js", 127),
            ("ts", 57),
            ("py", 3),
            ("rb", 2),
            ("hs", 1),
        ]));

        assert_eq!(
            described,
            "127 .js, 57 .ts, 3 .py, 2 .rb, and 1 more extensions"
        );
    }

    #[test]
    fn lists_every_extension_when_there_are_few() {
        assert_eq!(describe(&counts(&[("py", 2), ("rb", 1)])), "2 .py, 1 .rb");
    }
}
