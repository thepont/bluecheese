//! Language-independent coupling counts over adapter-resolved call edges.

use std::collections::BTreeSet;

use super::{SourceFile, model_for};

pub struct MethodCoupling {
    pub file_path: String,
    pub class_name: String,
    pub line: usize,
    pub callers: usize,
    pub calling_classes: usize,
    pub fan_out: usize,
}

pub struct Callable {
    pub owner: usize,
    pub file_path: String,
    pub class_name: String,
    pub line: usize,
    pub reportable: bool,
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
        if methods[caller].owner != methods[target].owner {
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
                callers: incoming[index].len(),
                calling_classes: classes.len(),
                fan_out: outgoing[index].len(),
            }
        })
        .collect()
}
