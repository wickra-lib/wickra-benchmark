//! [`Suite`] — the curated collection of cases — and the [`CaseResult`] /
//! [`SuiteReport`] the runner produces.

use crate::case::BenchmarkCase;
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeSet;

/// A curated suite of cases. The case order in the file is irrelevant — the
/// runner sorts results by `id` — but every `id` must be unique.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Suite {
    /// A human-readable suite name (for example `"wickra-benchmark v0.1 core suite"`).
    #[serde(default)]
    pub name: String,
    /// The curated cases.
    pub cases: Vec<BenchmarkCase>,
}

impl Suite {
    /// Parse a `Suite` from JSON.
    pub fn from_json(s: &str) -> Result<Self> {
        let suite: Self = serde_json::from_str(s).map_err(|e| Error::BadCase(e.to_string()))?;
        suite.validate()?;
        Ok(suite)
    }

    /// Parse a `Suite` from TOML.
    pub fn from_toml(s: &str) -> Result<Self> {
        let suite: Self = toml::from_str(s).map_err(|e| Error::Parse(e.to_string()))?;
        suite.validate()?;
        Ok(suite)
    }

    /// Validate the suite: every case is individually valid and all `id`s are
    /// unique. A duplicate `id` is a `BadCase` — the ids are the sort and tie
    /// key, so a collision would make the report order ambiguous.
    pub(crate) fn validate(&self) -> Result<()> {
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for case in &self.cases {
            case.validate()?;
            if !seen.insert(case.id.as_str()) {
                return Err(Error::BadCase(format!("duplicate case id: {}", case.id)));
            }
        }
        Ok(())
    }

    /// The case ids, sorted ascending — the deterministic answer to `list_cases`.
    #[must_use]
    pub fn case_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self.cases.iter().map(|c| c.id.clone()).collect();
        ids.sort();
        ids
    }
}

/// The outcome of running one case: whether the recomputed report matches the
/// expectation (`passed`) and its frozen hash (`hash_match`), plus the recomputed
/// report and its hash for inspection.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CaseResult {
    /// The case `id`.
    pub id: String,
    /// The recomputed report is byte-exact equal to `case.expected`.
    pub passed: bool,
    /// `hash(recomputed) == case.expected_hash`.
    pub hash_match: bool,
    /// The freshly recomputed report as JSON (for diffing against the
    /// expectation). The engine's `BacktestReport` is serialize-only, so it
    /// travels as a JSON object.
    pub recomputed: Value,
    /// The canonical blake3 hash of `recomputed`.
    pub hash: String,
}

/// The result of running a whole suite: the per-case results (sorted by `id`)
/// and the pass/fail tally. A case counts as passing only when it both `passed`
/// and `hash_match`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SuiteReport {
    /// The per-case results, always sorted ascending by `id`.
    pub results: Vec<CaseResult>,
    /// The number of cases that both `passed` and `hash_match`.
    pub passed: usize,
    /// `results.len() - passed`.
    pub failed: usize,
}
