//! Repeated reads and explicit case handling of resolved types.

use std::collections::{BTreeMap, BTreeSet};

use anyhow::{Result, ensure};

use crate::finding::Finding;
use crate::lang::SourceFile;
use crate::lang::projections::Projection;

/// Ordinary bundles exclude isolated reads and pairs, even on tiny types.
/// Explicit handling of the same discriminator uses a two-case minimum;
/// consumer-count and overlap settings apply to both evidence paths.
pub struct BundleRules {
    min_members: usize,
    min_consumers: usize,
    min_overlap_percent: u8,
}

impl Default for BundleRules {
    fn default() -> Self {
        Self {
            min_members: 3,
            min_consumers: 3,
            min_overlap_percent: 60,
        }
    }
}

impl BundleRules {
    /// Overlap is the common bundle's share of each participating projection,
    /// not a percentage of the type's possibly incomplete declared surface.
    pub fn new(min_members: usize, min_consumers: usize, min_overlap_percent: u8) -> Result<Self> {
        ensure!(
            min_members >= 3,
            "member bundles require at least three distinct members"
        );
        ensure!(
            min_consumers >= 2,
            "member bundles require at least two independent consumers"
        );
        ensure!(
            (1..=100).contains(&min_overlap_percent),
            "overlap percentage must be in 1..=100"
        );
        Ok(Self {
            min_members,
            min_consumers,
            min_overlap_percent,
        })
    }
}

pub(super) fn detect(files: &[SourceFile], rules: &BundleRules) -> Result<Vec<Finding>> {
    let mut by_type = BTreeMap::<_, Vec<_>>::new();
    for projection in crate::lang::projections::collect(files) {
        let min_members = projection.kind.minimum_members(rules.min_members);
        if projection.members.len() < min_members
            || super::source::generated(projection.consumer.file())
        {
            continue;
        }
        by_type
            .entry((
                projection.target.file().rel.clone(),
                projection.target.node().start_byte(),
                projection.kind.clone(),
            ))
            .or_default()
            .push(projection);
    }
    let mut findings = Vec::new();
    for projections in by_type.into_values() {
        findings.extend(type_findings(&projections, rules)?);
    }
    Ok(findings)
}

fn type_findings(projections: &[Projection<'_>], rules: &BundleRules) -> Result<Vec<Finding>> {
    let min_members = projections[0].kind.minimum_members(rules.min_members);
    let cores = common_cores(projections, min_members)?;
    let mut findings = BTreeMap::new();
    for common in cores {
        let consumers: BTreeMap<_, _> = projections
            .iter()
            .filter(|projection| projection.shares(&common, rules.min_overlap_percent))
            .map(|projection| {
                (
                    (
                        projection.consumer.file().rel.clone(),
                        projection.consumer.node().start_byte(),
                    ),
                    projection,
                )
            })
            .collect();
        if consumers.len() < rules.min_consumers {
            continue;
        }
        let locations = consumers
            .values()
            .map(|projection| {
                format!(
                    "{}:{}",
                    projection.consumer.file().rel,
                    projection.consumer.line()
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        for (key, projection) in &consumers {
            findings
                .entry(key.clone())
                .or_insert_with(|| finding(projection, &common, &locations));
        }
    }
    Ok(findings.into_values().collect())
}

/// Bound intersection closure explicitly: incomplete analysis must be an error,
/// not an empty or partial clean report on combinatorial input.
const MAX_CORES_PER_TYPE: usize = 4096;

fn common_cores(
    projections: &[Projection<'_>],
    min_members: usize,
) -> Result<BTreeSet<BTreeSet<String>>> {
    let mut pending: BTreeSet<_> = projections
        .iter()
        .map(|projection| projection.members.clone())
        .collect();
    let mut cores = BTreeSet::new();
    while let Some(core) = pending.pop_first() {
        cores.insert(core.clone());
        for projection in projections {
            let intersection: BTreeSet<_> =
                core.intersection(&projection.members).cloned().collect();
            if intersection.len() >= min_members && !cores.contains(&intersection) {
                pending.insert(intersection);
            }
        }
        ensure!(
            cores.len() + pending.len() <= MAX_CORES_PER_TYPE,
            "member-bundle analysis exceeded {MAX_CORES_PER_TYPE} common sets for {}; analysis is incomplete",
            projections[0].target_name
        );
    }
    Ok(cores)
}

fn finding(projection: &Projection<'_>, common: &BTreeSet<String>, locations: &str) -> Finding {
    let source = projection.consumer;
    let evidence = projection.kind.evidence_label();
    Finding {
        smell_name: super::SMELL_NAME.into(),
        file_path: source.file().path.to_string_lossy().into_owned(),
        relative_file_path: source.file().rel.clone(),
        class_name: projection.target_name.clone(),
        line_numbers: vec![source.line()],
        additional_info: format!(
            "Source evidence: {evidence} of {} ({}:{}), common members {:?}; {}/{} members in this projection. Independent consumers: {locations}. These consumers may require coordinated edits; review intentional handling. History was not evaluated.",
            projection.target_name,
            projection.target.file().rel,
            projection.target.line(),
            common,
            common.len(),
            projection.members.len(),
        ),
    }
}
