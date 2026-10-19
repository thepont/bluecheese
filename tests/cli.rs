//! Behaviour at the command-line boundary.
//!
//! The loader's behaviour is covered by `loading.rs`; these tests exist because
//! the CLI is where an incomplete analysis is actually presented to a person,
//! and "no findings" must never be reported as if it meant "no smells".

mod common;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use bluecheese::finding::Finding;
use common::TempTree;

fn binary() -> Command {
    Command::new(env!("CARGO_BIN_EXE_bluecheese"))
}

fn run(folder: &Path) -> Output {
    run_smell(folder, "long-method", &[])
}

/// Run the tool on `folder`, with `extra` appended. The smell is a parameter
/// rather than an extra, because it cannot be given twice.
fn run_smell(folder: &Path, smell: &str, extra: &[&str]) -> Output {
    binary()
        .args([
            "--folder",
            folder.to_str().expect("utf-8 path"),
            "--smell",
            smell,
        ])
        .args(extra)
        .output()
        .expect("run the tool")
}

/// The paths the report mentions, in the order it lists them.
fn reported(output: &Output) -> Vec<String> {
    let findings: Vec<Finding> =
        serde_json::from_slice(&output.stdout).expect("the report is a JSON array of findings");
    findings
        .into_iter()
        .map(|finding| finding.relative_file_path)
        .collect()
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

#[test]
fn a_folder_with_nothing_it_can_read_says_what_it_saw() {
    // The refusal for an empty folder and the refusal for a folder full of
    // JavaScript must not be the same message: the second one is a project the
    // tool cannot help with, and saying only "no source files found" would send
    // its owner looking for the wrong problem.
    let tree = TempTree::new("cli-only-python");
    tree.write("script.py", "print('hello')\n");

    let output = run(tree.path());

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("no source files found"),
        "stderr was: {stderr}"
    );
    assert!(
        stderr.contains("saw 1 .py"),
        "the refusal must name what it did see; stderr was: {stderr}"
    );
}

#[test]
fn an_unported_smell_is_refused_by_name() {
    // A smell of the book with no detector yet must not come back as an empty
    // report, which is what clean code looks like.
    let output = run_smell(&fixtures().join("rust"), "feature-envy", &[]);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("no detector for smell 'feature-envy'"),
        "stderr was: {stderr}"
    );
    assert!(
        stderr.contains("long-method"),
        "and it must say what can be run instead; stderr was: {stderr}"
    );
}

#[test]
fn switch_statements_can_be_requested_from_the_cli() {
    let output = run_smell(&fixtures().join("smelly"), "switch-stmts", &[]);

    assert!(
        output.status.success(),
        "stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let findings: Vec<Finding> = serde_json::from_slice(&output.stdout).expect("JSON findings");
    assert!(findings.iter().any(|finding| {
        finding.smell_name == "Switch Statements"
            && finding.relative_file_path == "SwitchOnKind.java"
            && finding.line_numbers == [24]
    }));
}

#[test]
fn message_chains_can_be_requested_from_the_cli() {
    let output = run_smell(&fixtures().join("smellbench"), "message-chains", &[]);

    assert!(
        output.status.success(),
        "stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let findings: Vec<Finding> = serde_json::from_slice(&output.stdout).expect("JSON findings");
    assert!(findings.iter().any(|finding| {
        finding.smell_name == "Message Chains"
            && finding.relative_file_path == "main/java/messagechains/case1/MCBE1.java"
            && finding.line_numbers == [96]
    }));
}

#[test]
fn comments_can_be_requested_from_the_cli() {
    let output = run_smell(&fixtures().join("smellbench"), "comments", &[]);

    assert!(
        output.status.success(),
        "stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let findings: Vec<Finding> = serde_json::from_slice(&output.stdout).expect("JSON findings");
    assert!(findings.iter().any(|finding| {
        finding.smell_name == "Comments"
            && finding.relative_file_path == "main/java/comments/case2/CBE2.java"
            && finding.line_numbers == [14]
    }));
}

#[test]
fn duplicate_code_can_be_requested_from_the_cli() {
    let output = run_smell(&fixtures().join("smellbench"), "dup-code", &[]);

    assert!(
        output.status.success(),
        "stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let findings: Vec<Finding> = serde_json::from_slice(&output.stdout).expect("JSON findings");
    assert!(findings.iter().any(|finding| {
        finding.smell_name == "Duplicate Code"
            && finding
                .relative_file_path
                .ends_with("dupcode/case1/DUPCBE1.java")
    }));
}

#[test]
fn speculative_generality_can_be_requested_from_the_cli() {
    let output = run_smell(&fixtures().join("smellbench"), "spec-gen", &[]);

    assert!(
        output.status.success(),
        "stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let findings: Vec<Finding> = serde_json::from_slice(&output.stdout).expect("JSON findings");
    assert!(findings.iter().any(|finding| {
        finding.smell_name == "Speculative Generality"
            && finding.class_name == "AccomodationBad"
            && finding
                .relative_file_path
                .ends_with("specgen/case1/SGBE1.java")
    }));
}

#[test]
fn a_label_that_is_not_a_smell_at_all_says_so() {
    // This is the mistake a person makes from memory, so it must not be
    // answered as if the smell were merely unported: `dead-code` was a detector
    // in the reference tool's source but it is not one of the book's smells.
    let output = run_smell(&fixtures().join("rust"), "dead-code", &[]);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("unknown smell 'dead-code'"),
        "stderr was: {stderr}"
    );
    assert!(
        stderr.contains("22 smells of 'Bad Smells in Code'"),
        "and it must count what it does know; stderr was: {stderr}"
    );
    assert!(
        stderr.contains("switch-stmts"),
        "and list it; stderr was: {stderr}"
    );
}

#[test]
fn a_malformed_exclude_pattern_is_refused_naming_the_pattern() {
    // The pattern is checked in the loader, which is several layers below the
    // flag, so this is where the two are shown to be connected: a typo in
    // `--exclude` must stop the run, not quietly exclude nothing.
    let output = run_smell(&fixtures().join("rust"), "long-method", &["--exclude", "["]);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("invalid exclude pattern '['"),
        "stderr was: {stderr}"
    );
}

#[test]
fn excluding_a_path_leaves_it_out_of_the_report() {
    // The end-to-end version of `scope.rs`: the same folder, scanned twice, with
    // the only difference the exclusion a person would type to keep their own
    // fixtures out of their report.
    let folder = fixtures();
    let unscoped = run(&folder);
    let scoped = run_smell(&folder, "long-method", &["--exclude", "smelly/"]);

    let before = reported(&unscoped);
    assert!(
        before.iter().any(|path| path.starts_with("smelly/")),
        "the corpus must be reported without the flag, or the flag proves nothing: {before:?}"
    );

    let after = reported(&scoped);
    assert!(
        after.iter().all(|path| !path.starts_with("smelly/")),
        "the exclusion must drop the corpus: {after:?}"
    );
    assert!(
        after.iter().any(|path| path.starts_with("smellbench/")),
        "and must drop nothing else: {after:?}"
    );
}
