//! The runner: recompute a case's report and check it against the expectation.
//!
//! `run_case` is the atom. `run_suite` (path-based, for the CLI) and
//! `run_suite_inline` (data supplied inline, for the FFI boundary) both fan out
//! over cases and then re-sort the results by `id`, so a `SuiteReport` is
//! byte-identical regardless of case order, data source, or — when the
//! `parallel` feature is on — rayon's scheduling.

use crate::case::{BenchmarkCase, Candle, StrategySpec};
use crate::error::{Error, Result};
use crate::hash::{canonicalize, hash};
use crate::suite::{CaseResult, Suite, SuiteReport};
use std::collections::BTreeMap;
use std::path::Path;
use wickra_backtest_core::run;

/// Recompute `case`'s report from its strategy and the supplied candle `data`,
/// then check it against the frozen expectation. Returns a [`CaseResult`] with
/// two independent booleans: `passed` (the recomputed report is byte-exact equal
/// to `case.expected`) and `hash_match` (its canonical hash equals
/// `case.expected_hash`).
pub fn run_case(case: &BenchmarkCase, data: &[Candle]) -> Result<CaseResult> {
    case.validate()?;
    let spec: StrategySpec =
        serde_json::from_value(case.strategy.clone()).map_err(|e| Error::BadSpec(e.to_string()))?;
    let report = run(&spec, data).map_err(|e| Error::Backtest(e.to_string()))?;
    let recomputed = serde_json::to_value(&report).map_err(|e| Error::BadCase(e.to_string()))?;

    // Canonicalize the recomputed report once: its canonical form is both the
    // hash input and the byte-exact comparison axis (the same axis the
    // cross-language golden uses). `passed` and `hash_match` are independent —
    // they diverge when a case's `expected` and `expected_hash` disagree.
    let recomputed_canon = canonicalize(&recomputed)?;
    let hash = hash(&recomputed_canon);
    let hash_match = hash == case.expected_hash;
    let passed = recomputed_canon == canonicalize(&case.expected)?;

    Ok(CaseResult {
        id: case.id.clone(),
        passed,
        hash_match,
        recomputed,
        hash,
    })
}

/// Run every case in `suite`, loading each case's dataset from
/// `data_root/<dataset_ref>` as CSV. Used by the CLI, which has filesystem
/// access.
pub fn run_suite(suite: &Suite, data_root: &Path) -> Result<SuiteReport> {
    suite.validate()?;
    let mut loaded: Vec<(&BenchmarkCase, Vec<Candle>)> = Vec::with_capacity(suite.cases.len());
    for case in &suite.cases {
        let candles = load_candles(&data_root.join(&case.dataset_ref))?;
        loaded.push((case, candles));
    }
    Ok(tally(execute(&loaded)?))
}

/// Run every case in `suite` against datasets supplied inline, keyed by
/// `dataset_ref`. Used across the FFI boundary, which has no filesystem. Must
/// produce the same report as [`run_suite`] for the same data.
pub fn run_suite_inline(
    suite: &Suite,
    datasets: &BTreeMap<String, Vec<Candle>>,
) -> Result<SuiteReport> {
    suite.validate()?;
    let mut loaded: Vec<(&BenchmarkCase, Vec<Candle>)> = Vec::with_capacity(suite.cases.len());
    for case in &suite.cases {
        let candles = datasets
            .get(&case.dataset_ref)
            .ok_or_else(|| Error::Data(format!("no dataset supplied for {}", case.dataset_ref)))?
            .clone();
        loaded.push((case, candles));
    }
    Ok(tally(execute(&loaded)?))
}

/// Sort the results by `id` and tally the pass count. A case passes only when it
/// both `passed` and `hash_match`.
fn tally(mut results: Vec<CaseResult>) -> SuiteReport {
    results.sort_by(|a, b| a.id.cmp(&b.id));
    let passed = results.iter().filter(|r| r.passed && r.hash_match).count();
    SuiteReport {
        failed: results.len() - passed,
        passed,
        results,
    }
}

#[cfg(feature = "parallel")]
fn execute(loaded: &[(&BenchmarkCase, Vec<Candle>)]) -> Result<Vec<CaseResult>> {
    use rayon::prelude::*;
    loaded
        .par_iter()
        .map(|(case, candles)| run_case(case, candles))
        .collect()
}

#[cfg(not(feature = "parallel"))]
fn execute(loaded: &[(&BenchmarkCase, Vec<Candle>)]) -> Result<Vec<CaseResult>> {
    loaded
        .iter()
        .map(|(case, candles)| run_case(case, candles))
        .collect()
}

/// Load a candle CSV (`time,open,high,low,close,volume`, an optional header row,
/// `volume` optional per row). Rows are trusted repo data; a malformed row is a
/// [`Error::Data`].
pub fn load_candles(path: &Path) -> Result<Vec<Candle>> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| Error::Data(format!("{}: {e}", path.display())))?;
    parse_candles_csv(&text).map_err(|e| Error::Data(format!("{}: {e}", path.display())))
}

/// Parse a candle CSV body into candles. A leading non-numeric row is treated as
/// a header and skipped; any later non-numeric `time` is an error.
fn parse_candles_csv(text: &str) -> std::result::Result<Vec<Candle>, String> {
    let mut out = Vec::new();
    for (idx, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        let cols: Vec<&str> = line.split(',').map(str::trim).collect();
        if cols.len() < 5 {
            return Err(format!("line {}: expected >= 5 columns", idx + 1));
        }
        let Ok(time) = cols[0].parse::<i64>() else {
            // A non-numeric time in the first non-empty row is the header.
            if out.is_empty() {
                continue;
            }
            return Err(format!("line {}: non-numeric time {:?}", idx + 1, cols[0]));
        };
        let field = |i: usize, name: &str| -> std::result::Result<f64, String> {
            cols[i]
                .parse::<f64>()
                .map_err(|_| format!("line {}: bad {name} {:?}", idx + 1, cols[i]))
        };
        let volume = if cols.len() >= 6 {
            field(5, "volume")?
        } else {
            0.0
        };
        out.push(Candle {
            time,
            open: field(1, "open")?,
            high: field(2, "high")?,
            low: field(3, "low")?,
            close: field(4, "close")?,
            volume,
        });
    }
    Ok(out)
}
