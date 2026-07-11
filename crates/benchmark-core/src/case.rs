//! [`BenchmarkCase`] — one curated `(strategy, dataset, expected report, hash)`
//! tuple: the unit the suite is built from.

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub use wickra_backtest_core::{BacktestReport, Candle, StrategySpec};

/// A single golden-verified case. Its `strategy` is an embedded wickra-backtest
/// `StrategySpec` (kept as raw JSON so benchmark-core stays decoupled from the
/// engine's struct internals across the FFI boundary); its `expected` report and
/// `expected_hash` are frozen when the case is blessed and are byte-exact
/// thereafter. Running the case recomputes the report from `strategy` + the
/// dataset and checks it against both.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct BenchmarkCase {
    /// Stable, unique case key; the sort and tie key (for example
    /// `"sma-crossover-01"`).
    pub id: String,
    /// Human-readable description of what the case exercises.
    pub description: String,
    /// The embedded wickra-backtest `StrategySpec`, as raw JSON.
    pub strategy: Value,
    /// The dataset file this case runs on, relative to the data root (for
    /// example `"sma-uptrend.csv"`).
    pub dataset_ref: String,
    /// The frozen reference report (byte-exact) the recompute is checked
    /// against, kept as raw JSON. The engine's `BacktestReport` is serialize-only
    /// across the FFI boundary, so the expectation travels as a JSON object.
    pub expected: Value,
    /// The lowercase 64-hex blake3 of the canonical `expected` report.
    pub expected_hash: String,
}

/// A 64-character lowercase-hex string is the shape of a blake3 digest.
fn is_hex64_lowercase(s: &str) -> bool {
    s.len() == 64
        && s.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

impl BenchmarkCase {
    /// Parse a `BenchmarkCase` from JSON.
    pub fn from_json(s: &str) -> Result<Self> {
        let case: Self = serde_json::from_str(s).map_err(|e| Error::BadCase(e.to_string()))?;
        case.validate()?;
        Ok(case)
    }

    /// Parse a `BenchmarkCase` from TOML.
    pub fn from_toml(s: &str) -> Result<Self> {
        let case: Self = toml::from_str(s).map_err(|e| Error::Parse(e.to_string()))?;
        case.validate()?;
        Ok(case)
    }

    /// Validate structural invariants: a non-empty `id` and `dataset_ref`, a
    /// well-formed `expected_hash`, and a `strategy` that is a JSON object.
    pub(crate) fn validate(&self) -> Result<()> {
        if self.id.is_empty() {
            return Err(Error::BadCase("case id must not be empty".to_string()));
        }
        if self.dataset_ref.is_empty() {
            return Err(Error::BadCase("dataset_ref must not be empty".to_string()));
        }
        if !is_hex64_lowercase(&self.expected_hash) {
            return Err(Error::BadCase(format!(
                "expected_hash must be 64 lowercase hex characters, got {:?}",
                self.expected_hash
            )));
        }
        if !self.expected.is_object() {
            return Err(Error::BadCase(
                "expected must be a JSON object (a BacktestReport)".to_string(),
            ));
        }
        if !self.strategy.is_object() {
            return Err(Error::BadSpec(
                "strategy must be a JSON object (a StrategySpec)".to_string(),
            ));
        }
        Ok(())
    }
}
