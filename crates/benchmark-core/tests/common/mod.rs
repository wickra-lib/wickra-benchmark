//! Shared fixtures for the benchmark-core integration tests: a valid embedded
//! strategy, deterministic candle universes, and a `bless` helper that turns a
//! `(strategy, candles)` pair into a self-passing [`BenchmarkCase`] by taking
//! the real engine's recomputed report and hash as the frozen expectation —
//! exactly the bless flow the CLI uses.
//!
//! Each integration-test binary pulls this module in and uses a different subset
//! of these helpers, so unused items in any single binary are expected.
#![allow(dead_code)]

use benchmark_core::{canonicalize, hash, BenchmarkCase, Candle, StrategySpec};
use serde_json::{json, Value};

/// The symbol the sample strategy trades.
pub const SYMBOL: &str = "TEST";

/// A valid embedded `StrategySpec` (EMA cross) that trades [`SYMBOL`].
pub fn strategy_json() -> Value {
    json!({
        "symbol": SYMBOL,
        "timeframe": "1h",
        "indicators": {
            "ema_fast": { "type": "Ema", "params": [3] },
            "ema_slow": { "type": "Ema", "params": [8] }
        },
        "entry": { "cross_above": ["ema_fast", "ema_slow"] },
        "exit": { "cross_below": ["ema_fast", "ema_slow"] },
        "sizing": { "type": "fixed_fraction", "fraction": 0.95 },
        "costs": { "taker_bps": 5, "slippage": { "type": "fixed_bps", "bps": 2 } }
    })
}

/// Build a candle series from a list of closes; `open` is the previous close,
/// `high`/`low` are `max`/`min(open, close) ± 1`, volume is constant.
pub fn candles_from(closes: &[f64]) -> Vec<Candle> {
    closes
        .iter()
        .enumerate()
        .map(|(i, &c)| {
            let o = if i == 0 { c } else { closes[i - 1] };
            Candle {
                time: 1_700_000_000 + i64::try_from(i).unwrap() * 3600,
                open: o,
                high: o.max(c) + 1.0,
                low: o.min(c) - 1.0,
                close: c,
                volume: 1000.0,
            }
        })
        .collect()
}

/// A deterministic 40-bar V-shaped universe (down to bar 10, then up) so the EMA
/// cross fires at least once.
pub fn sample_closes() -> Vec<f64> {
    (0..40)
        .map(|i| {
            if i <= 10 {
                120.0 - 2.0 * f64::from(i)
            } else {
                100.0 + 2.0 * f64::from(i - 10)
            }
        })
        .collect()
}

/// The sample candle universe.
pub fn sample_candles() -> Vec<Candle> {
    candles_from(&sample_closes())
}

/// Recompute the report the given strategy produces over `candles` with the real
/// engine, and its canonical hash — the raw material a blessed case freezes.
pub fn recompute(strategy: &Value, candles: &[Candle]) -> (Value, String) {
    let spec: StrategySpec = serde_json::from_value(strategy.clone()).expect("valid strategy");
    let report = wickra_backtest_core::run(&spec, candles).expect("engine runs");
    let recomputed = serde_json::to_value(&report).expect("report serializes");
    let canon = canonicalize(&recomputed).expect("canonicalizes");
    let digest = hash(&canon);
    (recomputed, digest)
}

/// Bless a case: run `(strategy, candles)` through the engine and freeze the
/// recomputed report and its hash as the expectation, so the case self-passes.
pub fn bless(id: &str, description: &str, strategy: Value, candles: &[Candle]) -> BenchmarkCase {
    let (expected, expected_hash) = recompute(&strategy, candles);
    BenchmarkCase {
        id: id.to_string(),
        description: description.to_string(),
        strategy,
        dataset_ref: format!("{id}.csv"),
        expected,
        expected_hash,
    }
}

/// The canonical sample case: the EMA-cross strategy blessed over the sample
/// universe.
pub fn sample_case() -> BenchmarkCase {
    bless(
        "ema-cross-01",
        "EMA(3/8) cross over a V-shaped universe.",
        strategy_json(),
        &sample_candles(),
    )
}
