//! A green CodeQL snapshot must demonstrate detection for every shipped rule.
//! CodeQL itself checks the exact results; this gate checks the positive controls.

mod common;

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail, ensure};

fn queries(root: &Path) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    for entry in ignore::WalkBuilder::new(root)
        .filter_entry(|entry| entry.path().extension().is_none_or(|ext| ext != "testproj"))
        .build()
    {
        let entry = entry?;
        if entry.file_type().is_some_and(|kind| kind.is_file())
            && entry.path().extension().is_some_and(|ext| ext == "ql")
        {
            paths.push(entry.into_path());
        }
    }
    paths.sort();
    Ok(paths)
}

fn declaration(source: &str, prefix: &str) -> Result<String> {
    let values: Vec<_> = source
        .lines()
        .filter_map(|line| line.trim().strip_prefix(prefix))
        .collect();
    ensure!(
        values.len() == 1,
        "expected exactly one {prefix} declaration"
    );
    let value = values[0].trim();
    ensure!(
        !value.is_empty() && value.split_whitespace().count() == 1,
        "expected a single query ID after {prefix}"
    );
    Ok(value.to_owned())
}

fn positive_controls(root: &Path) -> Result<usize> {
    let mut shipped = BTreeSet::new();
    let mut covered = BTreeSet::new();
    for entry in fs::read_dir(root)? {
        let pack = entry?.path();
        if !pack.is_dir() {
            continue;
        }
        let name = pack.file_name().unwrap().to_string_lossy();
        if name.ends_with("-smells-tests") {
            ensure!(
                pack.with_file_name(name.trim_end_matches("-tests"))
                    .is_dir(),
                "missing production pack for {}",
                pack.display()
            );
        }
        if !name.ends_with("-smells") {
            continue;
        }
        let production = queries(&pack)?;
        ensure!(!production.is_empty(), "{} has no queries", pack.display());
        let mut pack_ids = BTreeSet::new();
        for query in production {
            let id = declaration(&fs::read_to_string(&query)?, "* @id ")
                .with_context(|| query.display().to_string())?;
            ensure!(
                shipped.insert(id.clone()),
                "duplicate production query ID: {id}"
            );
            pack_ids.insert(id);
        }
        let tests = pack.with_file_name(format!(
            "{}-tests",
            pack.file_name().unwrap().to_string_lossy()
        ));
        ensure!(tests.is_dir(), "missing test pack: {}", tests.display());
        for query in queries(&tests)? {
            let source = fs::read_to_string(&query)?;
            let id =
                declaration(&source, "// @tests ").with_context(|| query.display().to_string())?;
            ensure!(
                pack_ids.contains(&id),
                "{} names unknown query {id}",
                query.display()
            );
            let expectation = fs::read_to_string(query.with_extension("expected"))
                .with_context(|| format!("missing expectation for {}", query.display()))?;
            if source
                .lines()
                .any(|line| line.trim() == "// @negative-only")
            {
                ensure!(
                    expectation.trim().is_empty(),
                    "negative-only test has findings: {}",
                    query.display()
                );
            } else {
                ensure!(
                    !expectation.trim().is_empty(),
                    "positive test expects no findings: {}",
                    query.display()
                );
                covered.insert(id);
            }
        }
    }
    ensure!(!shipped.is_empty(), "no production queries found");
    let missing: Vec<_> = shipped.difference(&covered).collect();
    if !missing.is_empty() {
        bail!("queries without a positive test: {missing:?}");
    }
    Ok(shipped.len())
}

#[test]
fn every_production_query_has_a_positive_control() {
    let count = positive_controls(&common::repo().join("codeql"))
        .expect("each production rule must have a test expecting at least one finding");
    println!("{count} production queries have positive controls");
}

fn passing_corpus() -> common::TempTree {
    let tree = common::TempTree::new("codeql-controls");
    tree.write("example-smells/Rule.ql", "/**\n * @id example/rule\n */\n")
        .write(
            "example-smells-tests/Rule/RuleTest.ql",
            "// @tests example/rule\n",
        )
        .write(
            "example-smells-tests/Rule/RuleTest.expected",
            "| detected |\n",
        );
    tree
}

#[test]
fn positive_control_is_required_even_when_negative_only_tests_exist() {
    let tree = passing_corpus();
    tree.write(
        "example-smells-tests/Rule/NegativeTest.ql",
        "// @tests example/rule\n// @negative-only\n",
    )
    .write("example-smells-tests/Rule/NegativeTest.expected", "\n");
    assert_eq!(positive_controls(tree.path()).unwrap(), 1);

    tree.write(
        "example-smells-tests/Rule/RuleTest.ql",
        "// @tests example/rule\n// @negative-only\n",
    )
    .write("example-smells-tests/Rule/RuleTest.expected", "");
    let error = positive_controls(tree.path()).unwrap_err().to_string();
    assert!(error.contains("queries without a positive test"), "{error}");
}

#[test]
fn invalid_coverage_cannot_pass_the_gate() {
    let cases = [
        ("RuleTest.expected", "", "positive test expects no findings"),
        (
            "RuleTest.expected",
            " \t\n",
            "positive test expects no findings",
        ),
        (
            "RuleTest.ql",
            "// @tests unknown/rule\n",
            "names unknown query",
        ),
        ("RuleTest.ql", "", "expected exactly one // @tests"),
        (
            "RuleTest.ql",
            "// @tests example/rule\n// @tests example/rule\n",
            "expected exactly one // @tests",
        ),
        (
            "RuleTest.ql",
            "// @tests example/rule extra\n",
            "expected a single query ID",
        ),
        (
            "RuleTest.ql",
            "// @tests \n",
            "expected exactly one // @tests",
        ),
        (
            "RuleTest.ql",
            "// @tests example/rule\n// @negative-only\n",
            "negative-only test has findings",
        ),
    ];
    for (file, contents, expected) in cases {
        let tree = passing_corpus();
        tree.write(&format!("example-smells-tests/Rule/{file}"), contents);
        let error = format!("{:#}", positive_controls(tree.path()).unwrap_err());
        assert!(error.contains(expected), "{file} {contents:?}: {error}");
    }
}

#[test]
fn adding_a_production_query_requires_its_own_positive_control() {
    let tree = passing_corpus();
    tree.write(
        "example-smells/Second.ql",
        "/**\n * @id example/second\n */\n",
    );
    let error = positive_controls(tree.path()).unwrap_err().to_string();
    assert!(error.contains("example/second"), "{error}");
}

#[test]
fn duplicate_production_ids_cannot_hide_an_untested_query() {
    let tree = passing_corpus();
    tree.write(
        "example-smells/Second.ql",
        "/**\n * @id example/rule\n */\n",
    );
    let error = positive_controls(tree.path()).unwrap_err().to_string();
    assert!(error.contains("duplicate production query ID"), "{error}");
}

#[test]
fn missing_expectation_is_an_error() {
    let tree = passing_corpus();
    tree.write(
        "example-smells-tests/Rule/MissingTest.ql",
        "// @tests example/rule\n",
    );
    let error = positive_controls(tree.path()).unwrap_err().to_string();
    assert!(error.contains("missing expectation"), "{error}");
}

#[test]
fn an_empty_scan_cannot_claim_coverage() {
    let tree = common::TempTree::new("no-codeql-queries");
    let error = positive_controls(tree.path()).unwrap_err().to_string();
    assert!(error.contains("no production queries"), "{error}");
}

#[test]
fn extraction_artifacts_are_not_test_definitions() {
    let tree = passing_corpus();
    tree.write("example-smells-tests/Rule/Rule.testproj/generated.ql", "");
    assert_eq!(positive_controls(tree.path()).unwrap(), 1);
}

#[test]
fn orphan_test_packs_cannot_silently_escape_validation() {
    let tree = passing_corpus();
    tree.write(
        "orphan-smells-tests/Rule/RuleTest.ql",
        "// @tests missing/rule\n",
    );
    let error = positive_controls(tree.path()).unwrap_err().to_string();
    assert!(error.contains("missing production pack"), "{error}");
}

#[test]
fn missing_test_packs_cannot_silently_escape_validation() {
    let tree = passing_corpus();
    tree.write(
        "untested-smells/Rule.ql",
        "/**\n * @id untested/rule\n */\n",
    );
    let error = positive_controls(tree.path()).unwrap_err().to_string();
    assert!(error.contains("missing test pack"), "{error}");
}
