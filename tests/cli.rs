//! Behaviour at the command-line boundary.
//!
//! The loader's behaviour is covered by `loading.rs`; these tests exist because
//! the CLI is where an incomplete analysis is actually presented to a person,
//! and "no findings" must never be reported as if it meant "no smells".

use std::path::PathBuf;
use std::process::{Command, Output};

fn binary() -> Command {
    Command::new(env!("CARGO_BIN_EXE_bluecheese"))
}

fn run(folder: &PathBuf) -> Output {
    binary()
        .args([
            "--folder",
            folder.to_str().expect("utf-8 path"),
            "--smell",
            "long-method",
        ])
        .output()
        .expect("run the tool")
}

fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

/// A directory that exists but holds nothing to analyse.
fn empty_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("bluecheese-cli-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create directory");
    dir
}

#[test]
fn refuses_to_report_when_nothing_matched() {
    let folder = empty_dir("empty");

    let output = run(&folder);

    assert!(
        !output.status.success(),
        "an empty scan must not exit successfully; stdout was: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("no source files found"),
        "the refusal must say why; stderr was: {stderr}"
    );

    let _ = std::fs::remove_dir_all(&folder);
}

#[test]
fn reports_what_it_could_not_read() {
    let output = run(&fixtures().join("unsupported"));

    assert!(output.status.success(), "fixture should analyse cleanly");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("analysed 1 files (1 Rust)"),
        "stderr was: {stderr}"
    );
    assert!(
        stderr.contains("skipped 1 files with no adapter: 1 .py"),
        "an unread file must be accounted for, not silently dropped; stderr was: {stderr}"
    );
}
