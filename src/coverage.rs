//! Account for detector support separately from the loader's parser support.

use std::collections::BTreeMap;

use anyhow::{Result, ensure};

use crate::lang::{LanguageId, SourceFile};

/// Languages accepted and skipped by one detector for a particular input.
pub struct AnalysisCoverage {
    analyzed: BTreeMap<LanguageId, usize>,
    unsupported: BTreeMap<LanguageId, usize>,
}

impl AnalysisCoverage {
    pub(crate) fn new(files: &[SourceFile], smell: &str, languages: &[LanguageId]) -> Result<Self> {
        let mut coverage = Self {
            analyzed: BTreeMap::new(),
            unsupported: BTreeMap::new(),
        };
        for file in files {
            let counts = if languages.contains(&file.language) {
                &mut coverage.analyzed
            } else {
                &mut coverage.unsupported
            };
            *counts.entry(file.language).or_default() += 1;
        }
        ensure!(
            files.is_empty() || coverage.analyzed_files() > 0,
            "detector '{smell}' does not support these files ({}); supported languages: {}; 0 files analysed",
            coverage.unsupported_summary(),
            languages
                .iter()
                .map(|language| format!("{language:?}"))
                .collect::<Vec<_>>()
                .join(", ")
        );
        Ok(coverage)
    }

    pub fn analyzed_files(&self) -> usize {
        self.analyzed.values().sum()
    }

    pub fn unsupported_files(&self) -> usize {
        self.unsupported.values().sum()
    }

    pub fn unsupported_summary(&self) -> String {
        describe(&self.unsupported)
    }
}

impl std::fmt::Display for AnalysisCoverage {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "analysed {} files ({})",
            self.analyzed_files(),
            describe(&self.analyzed)
        )
    }
}

fn describe(counts: &BTreeMap<LanguageId, usize>) -> String {
    counts
        .iter()
        .map(|(language, count)| format!("{count} {language:?}"))
        .collect::<Vec<_>>()
        .join(", ")
}
