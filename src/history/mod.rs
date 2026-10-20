//! History-backed coupling candidates. Co-change is evidence for review, not a
//! proof that a shared responsibility is misplaced.

mod git;
mod symbols;

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::sync::Arc;

use anyhow::{Context, Result, ensure};

use crate::finding::Finding;
use crate::lang::{
    LanguageId, SourceFile,
    coupling::{self, MethodChanges, MethodCoupling},
};
use symbols::{SymbolKey, Symbols};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ObjectId(String);

impl ObjectId {
    fn parse(raw: &str) -> Result<Self> {
        ensure!(
            matches!(raw.len(), 40 | 64) && raw.bytes().all(|byte| byte.is_ascii_hexdigit()),
            "invalid Git object ID"
        );
        Ok(Self(raw.to_owned()))
    }
    fn as_str(&self) -> &str {
        &self.0
    }
}

struct Change {
    id: ObjectId,
    parent: ObjectId,
    paths: BTreeSet<String>,
}

impl Change {
    fn id(&self) -> &str {
        self.id.as_str()
    }
    fn includes(&self, path: &str) -> bool {
        self.paths.contains(path)
    }

    fn body_changed(
        &self,
        snapshots: &mut Snapshots<'_>,
        path: &str,
        key: &SymbolKey,
    ) -> Result<bool> {
        let before = snapshots.read(&self.parent, path)?;
        let after = snapshots.read(&self.id, path)?;
        Ok(before
            .zip(after)
            .is_some_and(|(before, after)| before.body_changed(&after, key)))
    }
}

struct Blob {
    id: ObjectId,
    source: String,
}

trait History {
    fn changes(&mut self) -> Result<Vec<Change>>;
    fn blob(&mut self, revision: &ObjectId, path: &str) -> Result<Option<Blob>>;
}

pub fn detect(root: &Path, files: &[SourceFile]) -> Result<Vec<Finding>> {
    let mut history = git::GitHistory::open(root, files)?;
    analyse(files, &mut history)
}

fn analyse(files: &[SourceFile], history: &mut dyn History) -> Result<Vec<Finding>> {
    ensure!(
        files.iter().all(|file| !file.root().has_error()),
        "current source contains parse errors; history evidence would be incomplete"
    );
    let symbols: BTreeMap<_, _> = files
        .iter()
        .map(|file| (file.rel.as_str(), Symbols::from_file(file)))
        .collect();
    let candidates: Vec<_> = coupling::analyse(files)
        .into_iter()
        .filter(|method| method.counts.is_shared_across_classes())
        .collect();
    let changes = history.changes()?;
    let mut snapshots = Snapshots::new(history, files);
    let mut evidence = Evidence {
        symbols: &symbols,
        snapshots: &mut snapshots,
    };
    let mut findings = Vec::new();
    for method in candidates {
        let commits = evidence.commits(&method, &changes)?;
        if commits.is_empty() {
            continue;
        }
        let mut finding = method.into_finding(crate::detectors::shotgun::SMELL_NAME);
        finding.additional_info.push_str(&format!(
            "; body co-change candidate; commits={}",
            commits.join(",")
        ));
        findings.push(finding);
    }
    Ok(findings)
}

struct Evidence<'a, 'history> {
    symbols: &'a BTreeMap<&'history str, Symbols>,
    snapshots: &'a mut Snapshots<'history>,
}

impl Evidence<'_, '_> {
    fn commits(&mut self, method: &MethodCoupling, changes: &[Change]) -> Result<Vec<String>> {
        let mut commits = Vec::new();
        for change in changes {
            let mut changed = ChangedMethods {
                change,
                symbols: self.symbols,
                snapshots: &mut *self.snapshots,
            };
            if method.changes_with_callers(&mut changed)? {
                commits.push(change.id().to_owned());
            }
        }
        Ok(commits)
    }
}

struct ChangedMethods<'a, 'history> {
    change: &'a Change,
    symbols: &'a BTreeMap<&'history str, Symbols>,
    snapshots: &'a mut Snapshots<'history>,
}

impl MethodChanges for ChangedMethods<'_, '_> {
    fn body_changed(&mut self, path: &str, offset: usize) -> Result<bool> {
        if !self.change.includes(path) {
            return Ok(false);
        }
        let Some(key) = self
            .symbols
            .get(path)
            .and_then(|symbols| symbols.at_offset(offset))
        else {
            return Ok(false);
        };
        self.change.body_changed(self.snapshots, path, key)
    }
}

struct Snapshots<'a> {
    history: &'a mut dyn History,
    languages: BTreeMap<&'a str, LanguageId>,
    revisions: BTreeMap<(ObjectId, String), Option<Arc<Symbols>>>,
    blobs: BTreeMap<(ObjectId, LanguageId), Arc<Symbols>>,
}

impl<'a> Snapshots<'a> {
    fn new(history: &'a mut dyn History, files: &'a [SourceFile]) -> Self {
        Self {
            history,
            languages: files
                .iter()
                .map(|file| (file.rel.as_str(), file.at(file.root()).model().id()))
                .collect(),
            revisions: BTreeMap::new(),
            blobs: BTreeMap::new(),
        }
    }

    fn read(&mut self, revision: &ObjectId, path: &str) -> Result<Option<Arc<Symbols>>> {
        let lookup = (revision.clone(), path.to_owned());
        if let Some(known) = self.revisions.get(&lookup) {
            return Ok(known.clone());
        }
        let symbols = if let Some(blob) = self.history.blob(revision, path)? {
            let language = *self
                .languages
                .get(path)
                .context("unknown source language")?;
            let blob_key = (blob.id, language);
            if let Some(known) = self.blobs.get(&blob_key) {
                Some(known.clone())
            } else {
                let symbols = Arc::new(
                    Symbols::parse(blob.source, language)
                        .with_context(|| format!("analysing historical source {path}"))?,
                );
                self.blobs.insert(blob_key, symbols.clone());
                Some(symbols)
            }
        } else {
            None
        };
        self.revisions.insert(lookup, symbols.clone());
        Ok(symbols)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn object_ids_accept_only_complete_git_hashes() {
        for raw in ["", "HEAD", "--help", &"g".repeat(40), &"a".repeat(39)] {
            assert!(ObjectId::parse(raw).is_err());
        }
        for length in [40, 64] {
            assert!(ObjectId::parse(&"a".repeat(length)).is_ok());
        }
    }

    struct BrokenHistory;

    impl History for BrokenHistory {
        fn changes(&mut self) -> Result<Vec<Change>> {
            anyhow::bail!("history unavailable")
        }
        fn blob(&mut self, _: &ObjectId, _: &str) -> Result<Option<Blob>> {
            panic!("no blobs should be requested after a history error")
        }
    }

    #[test]
    fn a_history_port_failure_cannot_become_an_empty_report() {
        assert!(analyse(&[], &mut BrokenHistory).is_err());
    }
}
