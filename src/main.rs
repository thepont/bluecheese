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
    let files = bluecheese::lang::load_project(&args.folder)?;
    let findings = bluecheese::detect(&files, &args.smell);
    println!("{}", serde_json::to_string_pretty(&findings)?);
    Ok(())
}
