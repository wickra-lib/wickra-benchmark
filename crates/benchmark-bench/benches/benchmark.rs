//! Criterion benchmarks for the benchmark core.
//!
//! `run_suite` is measured across suite sizes {10, 100, 1000 cases}, so the
//! report captures how recomputing and hash-checking a whole suite scales with
//! case count. Each case is a genuine blessed case — its expectation comes from
//! a real engine run — so the benchmark reflects production work exactly. Build
//! with `--no-default-features` to measure the sequential runner instead of the
//! parallel (rayon) one; the reports are byte-identical, so the delta is pure
//! scheduling.

use benchmark_core::{canonicalize, hash, run_suite_inline, BenchmarkCase, Candle, Suite};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use serde_json::{json, Value};
use std::collections::BTreeMap;

fn strategy() -> Value {
    json!({
        "symbol": "BENCH",
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

/// A deterministic, non-degenerate 128-bar candle universe seeded by `seed` so
/// distinct cases exercise distinct price paths.
fn candles(seed: usize) -> Vec<Candle> {
    let closes: Vec<f64> = (0..128)
        .map(|i| 100.0 + 10.0 * ((i + seed) as f64 * 0.1).sin())
        .collect();
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

/// Bless `n` cases over distinct universes and return the suite plus the inline
/// dataset map keyed by each case's `dataset_ref`.
fn blessed_suite(n: usize) -> (Suite, BTreeMap<String, Vec<Candle>>) {
    let mut cases = Vec::with_capacity(n);
    let mut datasets = BTreeMap::new();
    for i in 0..n {
        let bars = candles(i);
        let spec = serde_json::from_value(strategy()).unwrap();
        let report = wickra_backtest_core::run(&spec, &bars).unwrap();
        let recomputed = serde_json::to_value(&report).unwrap();
        let expected_hash = hash(&canonicalize(&recomputed).unwrap());
        let dataset_ref = format!("bench-{i:04}.csv");
        cases.push(BenchmarkCase {
            id: format!("bench-{i:04}"),
            description: "bench".to_string(),
            strategy: strategy(),
            dataset_ref: dataset_ref.clone(),
            expected: recomputed,
            expected_hash,
        });
        datasets.insert(dataset_ref, bars);
    }
    (
        Suite {
            name: "bench".to_string(),
            cases,
        },
        datasets,
    )
}

fn bench_run_suite(c: &mut Criterion) {
    let mut group = c.benchmark_group("run_suite");
    for &n in &[10usize, 100, 1000] {
        let (suite, datasets) = blessed_suite(n);
        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, _| {
            b.iter(|| {
                let report = run_suite_inline(&suite, &datasets).unwrap();
                assert_eq!(report.failed, 0);
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_run_suite);
criterion_main!(benches);
