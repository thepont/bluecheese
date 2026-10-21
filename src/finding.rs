//! The report record shape produced by StaticNose (`ReportStruct` in the Java
//! tool). Field names and casing mirror the Java JSON exactly so that a Rust
//! report can be diffed against a Java golden report.

use serde::{Deserialize, Serialize};

/// Findings together with any omissions that make the analysis incomplete.
#[derive(Default)]
pub struct AnalysisReport {
    pub findings: Vec<Finding>,
    pub warnings: Vec<String>,
}

impl AnalysisReport {
    pub fn complete(findings: Vec<Finding>) -> Self {
        Self {
            findings,
            warnings: Vec::new(),
        }
    }

    /// Callers consuming only findings must not silently lose partial coverage.
    pub fn require_complete(self) -> anyhow::Result<Vec<Finding>> {
        anyhow::ensure!(
            self.warnings.is_empty(),
            "incomplete analysis: {}",
            self.warnings.join("; ")
        );
        Ok(self.findings)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Finding {
    #[serde(rename = "smellName")]
    pub smell_name: String,

    #[serde(rename = "filePath")]
    pub file_path: String,

    #[serde(rename = "className")]
    pub class_name: String,

    #[serde(rename = "lineNumbers")]
    pub line_numbers: Vec<usize>,

    #[serde(rename = "relativeFilePath")]
    pub relative_file_path: String,

    #[serde(rename = "additionalInfo")]
    pub additional_info: String,
}

impl Finding {
    /// Key used to compare a Rust finding against a Java golden finding.
    /// Line order is not stable across implementations, so it is sorted here.
    pub fn key(&self) -> (String, String, String, Vec<usize>) {
        let mut lines = self.line_numbers.clone();
        lines.sort_unstable();
        lines.dedup();
        (
            self.smell_name.clone(),
            self.relative_file_path.clone(),
            self.class_name.clone(),
            lines,
        )
    }
}
