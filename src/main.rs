use std::collections::BTreeMap;
use std::path::PathBuf;

use clap::Parser;

/// Detect code smell candidates using source and local Git history.
#[derive(Parser)]
#[command(name = "bluecheese", version, about)]
struct Args {
    /// The folder to detect code smells in.
    #[arg(short = 'f', long = "folder")]
    folder: PathBuf,

    /// The code smell to detect.
    #[arg(short = 's', long = "smell")]
    smell: String,

    /// Use source-only reference heuristics, including static coupling for Shotgun Surgery.
    #[arg(long)]
    source_only: bool,

    /// Report format.
    #[arg(short = 'r', long = "report-format", default_value = "json")]
    format: String,

    /// Skip paths matching this gitignore-style pattern, relative to the folder.
    /// Repeatable. Use it for code that is tracked and legitimate but should not
    /// be analysed here, such as fixtures full of deliberate smells.
    #[arg(short = 'x', long = "exclude")]
    exclude: Vec<String>,

    /// Analyse paths matching this pattern even where a .gitignore or .ignore
    /// file excludes them. Repeatable, and takes precedence over `--exclude`.
    /// A pattern reaching into a directory must name the directory too, as
    /// `vendor/` plus `vendor/**`.
    #[arg(short = 'i', long = "include")]
    include: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let scope = bluecheese::lang::Scope {
        include: args.include,
        exclude: args.exclude,
    };
    let project = bluecheese::lang::load_project_with(&args.folder, &scope)?;

    print_inventory(&project, &args.folder)?;

    if args.smell == "shotgun" {
        eprintln!(
            "shotgun evidence: {}",
            if args.source_only {
                "source-only coupling heuristic; no history validation"
            } else {
                "body co-change in up to 128 non-merge commits; candidates require review"
            }
        );
    }

    let findings = if args.source_only {
        bluecheese::detect(&project.files, &args.smell)?
    } else {
        bluecheese::detect_project(&args.folder, &project.files, &args.smell)?
    };
    println!("{}", serde_json::to_string_pretty(&findings)?);
    Ok(())
}

/// Report the scan's coverage before any findings. Empty input is an error;
/// unsupported files are counted so partial coverage cannot look complete.
fn print_inventory(
    project: &bluecheese::lang::Project,
    folder: &std::path::Path,
) -> anyhow::Result<()> {
    if project.files.is_empty() {
        let tail = if project.skipped.is_empty() {
            String::new()
        } else {
            format!("; saw {}", describe(&project.skipped))
        };
        anyhow::bail!(
            "no source files found under {} (understood extensions: {}){tail}",
            folder.display(),
            bluecheese::lang::supported_extensions().join(", ")
        );
    }

    let summary = project.language_summary();
    eprintln!("analysed {} files ({summary})", project.files.len());

    if !project.skipped.is_empty() {
        eprintln!(
            "skipped {} files with no adapter: {}",
            project.skipped.values().sum::<usize>(),
            describe(&project.skipped)
        );
    }

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

    let remaining = counts.len().saturating_sub(NAMED_EXTENSIONS);
    if remaining == 0 {
        named
    } else {
        format!("{named}, and {remaining} more extensions")
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

    /// The notice is printed on every run, so equal counts must have stable
    /// ordering across machines and runs to avoid unrelated output churn.
    #[test]
    fn breaks_ties_by_extension_so_the_notice_is_stable() {
        assert_eq!(describe(&counts(&[("rb", 2), ("py", 2)])), "2 .py, 2 .rb");
    }
}
