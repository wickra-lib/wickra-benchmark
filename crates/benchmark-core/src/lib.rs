//! # benchmark-core
//!
//! The deterministic core of `wickra-benchmark`: a curated suite of
//! `(strategy, dataset, expected report, hash)` cases you recompute and confirm
//! byte-for-byte. Given a [`BenchmarkCase`], [`run_case`] recomputes the report
//! from the embedded strategy and candle data with the pinned `wickra-backtest`
//! engine, hashes it with the same canonicalization `wickra-proof` uses, and
//! reports two independent booleans: `passed` (the recomputed report is
//! byte-exact equal to the frozen `expected`) and `hash_match` (its canonical
//! hash equals `expected_hash`).
//!
//! [`run_suite`] (path-based, for the CLI) and [`run_suite_inline`] (data
//! supplied inline, for the FFI boundary) fan out over a [`Suite`] and re-sort
//! the results by `id`, so a [`SuiteReport`] is byte-identical across all ten
//! language bindings and between the parallel (rayon) and sequential (WASM)
//! runners. [`Benchmark`] exposes the `command_json` boundary the bindings
//! forward verbatim; [`canonicalize`]/[`hash`] are the single source of hash
//! stability.

mod benchmark;
mod case;
mod config;
mod error;
mod hash;
mod runner;
mod suite;

pub use benchmark::Benchmark;
pub use case::{BacktestReport, BenchmarkCase, Candle, StrategySpec};
pub use config::Config;
pub use error::{Error, Result};
pub use hash::{canonicalize, hash, hash_report};
pub use runner::{load_candles, run_case, run_suite, run_suite_inline};
pub use suite::{CaseResult, Suite, SuiteReport};

/// The benchmark-core crate version.
#[must_use]
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests;
