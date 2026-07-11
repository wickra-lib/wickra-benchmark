//! The single error type of the benchmark core.
//!
//! Every fallible entry point returns [`Result`]. The variants map one-to-one to
//! the ways running a case can fail before a [`CaseResult`] is even reached: a
//! malformed case, a strategy that does not deserialize, unusable data, the
//! backtest engine refusing to run, or a bad command envelope. A case whose
//! recomputed report simply *disagrees* with its expectation is **not** an error
//! — that is a successful [`CaseResult`] with `passed == false`.
//!
//! [`CaseResult`]: crate::CaseResult

use thiserror::Error;

/// The result of a benchmark-core operation.
pub type Result<T> = std::result::Result<T, Error>;

/// Everything that can go wrong while running a case or a suite.
#[derive(Debug, Error)]
pub enum Error {
    /// A case (or suite) is structurally invalid: an empty `id`, an empty
    /// `dataset_ref`, a malformed `expected_hash`, or a duplicate case `id`.
    #[error("invalid case: {0}")]
    BadCase(String),

    /// The embedded `strategy` JSON does not deserialize into a
    /// wickra-backtest `StrategySpec`.
    #[error("invalid strategy spec: {0}")]
    BadSpec(String),

    /// The referenced candle data is missing, empty, or could not be read.
    #[error("data error: {0}")]
    Data(String),

    /// The backtest engine refused to run on the given strategy and data.
    #[error("backtest engine error: {0}")]
    Backtest(String),

    /// A command envelope (or its embedded JSON) could not be parsed.
    #[error("parse error: {0}")]
    Parse(String),
}
