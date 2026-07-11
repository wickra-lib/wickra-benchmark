#![no_main]
//! Fuzz the run contract with genuine inputs. The fuzz bytes drive a bounded,
//! always-valid candle universe under a fixed strategy; the report is recomputed
//! with the real engine and frozen as a blessed case, which must always pass and
//! match its own hash, and be deterministic on repeat. This pins the core
//! invariant — a blessed case reproduces against its own inputs — across an
//! unbounded range of price paths.

use benchmark_core::{canonicalize, hash, run_case, BenchmarkCase, Candle};
use libfuzzer_sys::fuzz_target;
use serde_json::{json, Value};
use wickra_backtest_core::{run, StrategySpec};

fn strategy_value() -> Value {
    json!({
        "symbol": "F",
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

fuzz_target!(|data: &[u8]| {
    // Need enough bars to warm the slow EMA; pad the fuzz bytes deterministically.
    let mut closes: Vec<f64> = data.iter().map(|&b| 50.0 + f64::from(b)).collect();
    while closes.len() < 16 {
        closes.push(100.0);
    }

    let candles: Vec<Candle> = closes
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
        .collect();

    // Bless: recompute the report with the real engine and freeze it + its hash.
    let spec: StrategySpec = serde_json::from_value(strategy_value()).unwrap();
    let report = run(&spec, &candles).expect("a bounded, valid universe always runs");
    let recomputed = serde_json::to_value(&report).unwrap();
    let expected_hash = hash(&canonicalize(&recomputed).unwrap());

    let case = BenchmarkCase {
        id: "fuzz-01".to_string(),
        description: "fuzz".to_string(),
        strategy: strategy_value(),
        dataset_ref: "fuzz.csv".to_string(),
        expected: recomputed,
        expected_hash: expected_hash.clone(),
    };

    let result = run_case(&case, &candles).expect("a blessed case runs");
    assert!(result.passed, "a blessed case reproduces its own report");
    assert!(result.hash_match, "a blessed case matches its own hash");
    assert_eq!(result.hash, expected_hash, "hash is stable");

    // Determinism: re-running is byte-identical.
    let again = run_case(&case, &candles).unwrap();
    assert_eq!(result.hash, again.hash, "run_case is deterministic");
    assert_eq!(result.recomputed, again.recomputed);
});
