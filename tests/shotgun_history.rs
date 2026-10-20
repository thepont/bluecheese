mod common;

use bluecheese::finding::Finding;
use bluecheese::lang::load_project;
use common::TempTree;
use std::process::Command;

struct Repository(TempTree);

impl Repository {
    fn new() -> Self {
        let repo = Self(TempTree::new("shotgun-history"));
        repo.git(&["init", "--quiet"]);
        repo.write(1, 1, 1);
        repo.commit();
        repo
    }

    fn git(&self, args: &[&str]) -> String {
        let result = Command::new("git")
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .args([
                "-c",
                "user.name=Test",
                "-c",
                "user.email=test@example.invalid",
            ])
            .args(args)
            .current_dir(self.0.path())
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        String::from_utf8(result.stdout).unwrap().trim().to_owned()
    }

    fn write(&self, policy: usize, first: usize, second: usize) {
        self.0.write(
            "Policy.java",
            &format!("class Policy {{ int check() {{ return {policy}; }} }}"),
        );
        self.0.write(
            "A.java",
            &format!("class A {{ int run(Policy p) {{ return p.check() + {first}; }} }}"),
        );
        self.0.write(
            "B.java",
            &format!("class B {{ int run(Policy p) {{ return p.check() + {second}; }} }}"),
        );
    }

    fn commit(&self) -> String {
        self.git(&["add", "--all"]);
        self.git(&["commit", "--quiet", "-m", "fixture change"]);
        self.git(&["rev-parse", "HEAD"])
    }

    fn scan(&self) -> Vec<Finding> {
        let project = load_project(self.0.path()).unwrap();
        project.detect("shotgun").unwrap()
    }
}

#[test]
fn healthy_shared_helpers_do_not_become_history_findings() {
    let repo = Repository::new();
    let project = load_project(repo.0.path()).unwrap();
    assert_eq!(
        bluecheese::detect(&project.files, "shotgun").unwrap().len(),
        1
    );
    assert!(repo.scan().is_empty());
}

#[test]
fn changes_to_a_callee_and_two_existing_callers_carry_commit_evidence() {
    let repo = Repository::new();
    repo.write(2, 2, 2);
    let commit = repo.commit();
    let findings = repo.scan();
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].class_name, "Policy");
    assert!(findings[0].additional_info.contains(&commit));
}

#[test]
fn methods_on_the_same_line_keep_distinct_symbol_identities() {
    let repo = Repository::new();
    repo.0.write(
        "Policy.java",
        "class Policy { int check() { return 1; } int other() { return 9; } }",
    );
    repo.commit();
    repo.write(2, 2, 2);
    repo.0.write(
        "Policy.java",
        "class Policy { int check() { return 2; } int other() { return 9; } }",
    );
    repo.commit();
    assert_eq!(repo.scan().len(), 1);
}

#[test]
fn a_single_changed_caller_is_not_scattered_change() {
    let repo = Repository::new();
    repo.write(2, 2, 1);
    repo.commit();
    assert!(repo.scan().is_empty());
}

#[test]
fn formatting_and_comments_are_not_body_change_evidence() {
    let repo = Repository::new();
    repo.0.write(
        "Policy.java",
        "class Policy { int check() { /* explanation */ return\n1; } }",
    );
    repo.write_callers();
    repo.commit();
    assert!(repo.scan().is_empty());
}

impl Repository {
    fn write_callers(&self) {
        self.0.write(
            "A.java",
            "class A { int run(Policy p) { return p.check() + 2; } }",
        );
        self.0.write(
            "B.java",
            "class B { int run(Policy p) { return p.check() + 2; } }",
        );
    }
}

#[test]
fn missing_history_is_an_error_instead_of_a_clean_report() {
    let tree = TempTree::new("shotgun-no-history");
    tree.write("Policy.java", "class Policy { void check() {} }");
    let project = load_project(tree.path()).unwrap();
    assert!(project.detect("shotgun").is_err());
}

#[test]
fn adding_callers_does_not_count_as_changing_existing_callers() {
    let repo = Repository::new();
    repo.git(&["rm", "A.java", "B.java"]);
    repo.commit();
    repo.write(2, 2, 2);
    repo.commit();
    assert!(repo.scan().is_empty());
}

#[test]
fn signature_migrations_are_not_body_co_change_evidence() {
    let repo = Repository::new();
    repo.0.write(
        "Policy.java",
        "class Policy { int check(int value) { return value + 2; } }",
    );
    repo.0.write(
        "A.java",
        "class A { int run(Policy p) { return p.check(2) + 2; } }",
    );
    repo.0.write(
        "B.java",
        "class B { int run(Policy p) { return p.check(2) + 2; } }",
    );
    repo.commit();
    assert!(repo.scan().is_empty());
}

#[test]
fn string_literal_whitespace_is_preserved_as_body_change_evidence() {
    let repo = Repository::new();
    for version in [1, 2] {
        let spaces = " ".repeat(version);
        repo.0.write(
            "Policy.java",
            &format!("class Policy {{ String check() {{ return \"one{spaces}two\"; }} }}"),
        );
        repo.0.write(
            "A.java",
            &format!("class A {{ String run(Policy p) {{ return p.check() + \"A{version}\"; }} }}"),
        );
        repo.0.write(
            "B.java",
            &format!("class B {{ String run(Policy p) {{ return p.check() + \"B{version}\"; }} }}"),
        );
        repo.commit();
    }
    assert_eq!(repo.scan().len(), 1);
}

#[test]
fn history_evidence_works_for_rust_and_mixed_javascript_typescript_projects() {
    for sources in [
        [
            (
                "policy.rs",
                "struct Policy; impl Policy { fn check(&self) -> i32 { $VALUE } }",
            ),
            (
                "a.rs",
                "struct First; impl First { fn run(p: &Policy) -> i32 { p.check() + $VALUE } }",
            ),
            (
                "b.rs",
                "struct Second; impl Second { fn run(p: &Policy) -> i32 { p.check() + $VALUE } }",
            ),
        ],
        [
            (
                "policy.ts",
                "export class Policy { check(): number { return $VALUE; } }",
            ),
            (
                "a.js",
                "import { Policy } from './policy'; export class First { run() { return new Policy().check() + $VALUE; } }",
            ),
            (
                "b.ts",
                "import { Policy } from './policy'; export class Second { run(p: Policy) { return p.check() + $VALUE; } }",
            ),
        ],
    ] {
        let repo = Repository::new();
        for version in ["1", "2"] {
            for (path, source) in sources {
                repo.0.write(path, &source.replace("$VALUE", version));
            }
            repo.commit();
        }
        let findings = repo.scan();
        assert_eq!(findings.len(), 1, "{}", sources[0].0);
        assert_eq!(findings[0].relative_file_path, sources[0].0);
    }
}

#[test]
fn an_unborn_repository_is_not_reported_as_clean() {
    let tree = TempTree::new("shotgun-unborn");
    tree.write("Policy.java", "class Policy { void check() {} }");
    let result = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(tree.path())
        .output()
        .unwrap();
    assert!(result.status.success());
    let project = load_project(tree.path()).unwrap();
    assert!(project.detect("shotgun").is_err());
}

#[test]
fn cli_defaults_to_history_and_retains_an_explicit_source_only_comparison() {
    let repo = Repository::new();
    let run = |source_only: bool| {
        let mut command = Command::new(env!("CARGO_BIN_EXE_bluecheese"));
        command.arg("-f").arg(repo.0.path()).args(["-s", "shotgun"]);
        if source_only {
            command.arg("--source-only");
        }
        command.output().unwrap()
    };
    let historical = run(false);
    assert!(historical.status.success());
    assert!(
        serde_json::from_slice::<Vec<Finding>>(&historical.stdout)
            .unwrap()
            .is_empty()
    );
    let source = run(true);
    assert!(source.status.success());
    assert_eq!(
        serde_json::from_slice::<Vec<Finding>>(&source.stdout)
            .unwrap()
            .len(),
        1
    );
}
