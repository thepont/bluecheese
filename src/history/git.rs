//! Read bounded local Git history without checking out or executing source.

use std::collections::{BTreeMap, BTreeSet};
use std::io::Read;
use std::path::{Component, Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail, ensure};

use super::{Blob, Change, History, ObjectId};
use crate::lang::SourceFile;

const HISTORY_LIMIT: usize = 128;

pub struct GitHistory {
    reader: GitReader,
    project_paths: BTreeMap<String, String>,
}

impl GitHistory {
    pub fn open(root: &Path, files: &[SourceFile]) -> Result<Self> {
        let mut reader = GitReader::new(root);
        let output = reader.run(&["rev-parse", "--show-toplevel"])
            .context("Shotgun Surgery requires local Git history; use --source-only for the static coupling heuristic")?;
        let output = String::from_utf8(output)?;
        let repo = PathBuf::from(
            output
                .strip_suffix('\n')
                .context("unterminated Git repository path")?,
        );
        let mut project_paths = BTreeMap::new();
        for file in files {
            let absolute = file.path.canonicalize()?;
            let relative = absolute
                .strip_prefix(&repo)
                .context("source lies outside the Git repository")?;
            let path = relative
                .to_str()
                .context("Git source paths must be UTF-8")?
                .to_owned();
            validate_path(&path)?;
            project_paths.insert(file.rel.clone(), path);
        }
        Ok(Self {
            reader: reader.in_repository(repo),
            project_paths,
        })
    }
}

impl History for GitHistory {
    fn changes(&mut self) -> Result<Vec<Change>> {
        let maximum = format!("--max-count={HISTORY_LIMIT}");
        let output = self
            .reader
            .run(&[
                "rev-list",
                &maximum,
                "--no-merges",
                "--parents",
                "HEAD",
                "--",
            ])
            .context("reading Shotgun Surgery history (a committed HEAD is required)")?;
        let text = String::from_utf8(output)?;
        let mut changes = Vec::new();
        for line in text.lines() {
            let ids: Result<Vec<_>> = line.split_whitespace().map(ObjectId::parse).collect();
            let ids = ids?;
            if ids.len() == 1 {
                continue;
            }
            ensure!(ids.len() == 2, "unexpected merge in non-merge history");
            let paths = self.reader.run(&[
                "diff-tree",
                "--no-commit-id",
                "--name-only",
                "-z",
                "-r",
                "--no-renames",
                "--no-ext-diff",
                "--no-textconv",
                ids[1].as_str(),
                ids[0].as_str(),
                "--",
            ])?;
            let changed = changed_paths(&paths, &self.project_paths)?;
            changes.push(Change {
                id: ids[0].clone(),
                parent: ids[1].clone(),
                paths: changed,
            });
        }
        Ok(changes)
    }

    fn blob(&mut self, revision: &ObjectId, path: &str) -> Result<Option<Blob>> {
        let path = self
            .project_paths
            .get(path)
            .context("unknown project source path")?;
        let output = self.reader.run(&[
            "--literal-pathspecs",
            "ls-tree",
            "-z",
            revision.as_str(),
            "--",
            path,
        ])?;
        if output.is_empty() {
            return Ok(None);
        }
        let record = output
            .strip_suffix(&[0])
            .context("invalid Git tree record")?;
        let (metadata, recorded_path) = record.split_at(
            record
                .iter()
                .position(|byte| *byte == b'\t')
                .context("missing Git tree path")?,
        );
        ensure!(
            recorded_path[1..] == *path.as_bytes(),
            "unexpected Git tree path"
        );
        let metadata = std::str::from_utf8(metadata)?;
        let fields: Vec<_> = metadata.split_whitespace().collect();
        ensure!(
            fields.len() == 3 && fields[1] == "blob" && matches!(fields[0], "100644" | "100755"),
            "source is not a regular Git blob"
        );
        let id = ObjectId::parse(fields[2])?;
        let source = String::from_utf8(self.reader.run(&["cat-file", "blob", id.as_str()])?)
            .context("historical source is not UTF-8")?;
        Ok(Some(Blob { id, source }))
    }
}

fn changed_paths(
    output: &[u8],
    project_paths: &BTreeMap<String, String>,
) -> Result<BTreeSet<String>> {
    ensure!(
        output.is_empty() || output.last() == Some(&0),
        "unterminated Git path list"
    );
    let mut paths = BTreeSet::new();
    for path in output
        .split(|byte| *byte == 0)
        .filter(|path| !path.is_empty())
    {
        let path = std::str::from_utf8(path).context("Git paths must be UTF-8")?;
        validate_path(path)?;
        paths.extend(
            project_paths
                .iter()
                .filter(|(_, repo_path)| *repo_path == path)
                .map(|(project_path, _)| project_path.clone()),
        );
    }
    Ok(paths)
}

fn validate_path(path: &str) -> Result<()> {
    ensure!(
        !path.is_empty() && !path.contains('\0') && !Path::new(path).is_absolute(),
        "invalid Git source path"
    );
    ensure!(
        Path::new(path)
            .components()
            .all(|part| matches!(part, Component::Normal(_))),
        "Git source path must stay inside the repository"
    );
    Ok(())
}

struct GitReader {
    root: PathBuf,
    deadline: Instant,
}

impl GitReader {
    fn new(root: &Path) -> Self {
        Self {
            root: root.to_owned(),
            deadline: Instant::now() + Duration::from_secs(60),
        }
    }

    fn in_repository(mut self, root: PathBuf) -> Self {
        self.root = root;
        self
    }

    fn run(&mut self, args: &[&str]) -> Result<Vec<u8>> {
        const OUTPUT_LIMIT: u64 = 16 * 1024 * 1024;
        ensure!(
            Instant::now() < self.deadline,
            "Git history analysis exceeded 60 seconds"
        );
        let mut child = Command::new("git")
            .arg("--no-pager")
            .arg("-C")
            .arg(&self.root)
            .args(args)
            .env("GIT_OPTIONAL_LOCKS", "0")
            .env_remove("GIT_DIR")
            .env_remove("GIT_WORK_TREE")
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .context("running local Git")?;
        let stdout = child.stdout.take().context("reading local Git output")?;
        std::thread::scope(|scope| {
            let reader = scope.spawn(move || {
                let mut bytes = Vec::new();
                stdout
                    .take(OUTPUT_LIMIT + 1)
                    .read_to_end(&mut bytes)
                    .map(|_| bytes)
            });
            let status = wait_for_git(&mut child, self.deadline);
            let bytes = reader
                .join()
                .map_err(|_| anyhow::anyhow!("Git output reader failed"))??;
            ensure!(
                bytes.len() as u64 <= OUTPUT_LIMIT,
                "Git output exceeds 16 MiB limit"
            );
            ensure!(
                status?.success(),
                "local Git command failed ({})",
                args.first().unwrap_or(&"git")
            );
            Ok(bytes)
        })
    }
}

fn wait_for_git(child: &mut Child, deadline: Instant) -> Result<ExitStatus> {
    loop {
        match child.try_wait() {
            Ok(Some(status)) => return Ok(status),
            Ok(None) if Instant::now() < deadline => std::thread::sleep(Duration::from_millis(2)),
            outcome => {
                let _ = child.kill();
                let _ = child.wait();
                if let Err(error) = outcome {
                    return Err(error.into());
                }
                bail!("Git history analysis exceeded 60 seconds");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_expired_deadline_kills_and_reaps_a_blocked_git_process() {
        let mut child = Command::new("git")
            .args(["hash-object", "--stdin"])
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        assert!(wait_for_git(&mut child, Instant::now()).is_err());
        assert!(child.try_wait().unwrap().is_some());
    }

    #[test]
    fn invalid_repository_paths_are_rejected() {
        for path in ["", "/absolute", "../outside", "a/../outside", "a\0b"] {
            assert!(validate_path(path).is_err(), "{path:?}");
        }
        assert!(validate_path("folder with spaces/[Policy].java").is_ok());
    }
}
