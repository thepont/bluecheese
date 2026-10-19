//! Language-independent coupling counts over adapter-resolved call edges.

use std::collections::BTreeSet;

use super::{SourceFile, model_for};
use crate::finding::Finding;

pub struct MethodCoupling {
    pub file_path: String,
    pub class_name: String,
    pub line: usize,
    pub counts: CouplingCounts,
}

pub struct CouplingCounts {
    callers: usize,
    calling_classes: usize,
    fan_out: usize,
}

impl CouplingCounts {
    pub fn is_shared_across_classes(&self) -> bool {
        self.callers >= 2 && self.calling_classes >= 2
    }
}

impl std::fmt::Display for CouplingCounts {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "[CM={}, CC={}, Fan-Out={}]",
            self.callers, self.calling_classes, self.fan_out
        )
    }
}

impl MethodCoupling {
    pub fn into_finding(self, smell_name: &str) -> Finding {
        Finding {
            smell_name: smell_name.into(),
            file_path: self.file_path.clone(),
            relative_file_path: self.file_path,
            class_name: self.class_name,
            line_numbers: vec![self.line],
            additional_info: format!("{smell_name} {}", self.counts),
        }
    }
}

pub struct Callable {
    pub owner: usize,
    pub file_path: String,
    pub class_name: String,
    pub line: usize,
    pub reportable: bool,
}

impl Callable {
    fn is_external_to(&self, other: &Self) -> bool {
        self.owner != other.owner
    }
}

/// Resolve within each adapter's supported language family, then combine the
/// independent results. The adapters own name resolution; this owns counts.
pub fn analyse(files: &[SourceFile]) -> Vec<MethodCoupling> {
    let languages: BTreeSet<_> = files.iter().map(|file| file.language).collect();
    languages
        .into_iter()
        .flat_map(|language| model_for(language).method_coupling(files))
        .collect()
}

pub fn count(methods: &[Callable], edges: &[(usize, usize)]) -> Vec<MethodCoupling> {
    let mut incoming = vec![BTreeSet::new(); methods.len()];
    let mut outgoing = vec![BTreeSet::new(); methods.len()];
    for &(caller, target) in edges {
        if methods[caller].is_external_to(&methods[target]) {
            incoming[target].insert(caller);
            outgoing[caller].insert(target);
        }
    }
    methods
        .iter()
        .enumerate()
        .filter(|(_, method)| method.reportable)
        .map(|(index, method)| {
            let classes: BTreeSet<_> = incoming[index]
                .iter()
                .map(|caller| methods[*caller].owner)
                .collect();
            MethodCoupling {
                file_path: method.file_path.clone(),
                class_name: method.class_name.clone(),
                line: method.line,
                counts: CouplingCounts {
                    callers: incoming[index].len(),
                    calling_classes: classes.len(),
                    fan_out: outgoing[index].len(),
                },
            }
        })
        .collect()
}
