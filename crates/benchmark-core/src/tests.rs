//! Inline tests for benchmark-core: case/suite validation, the honest and
//! fudged run-case outcomes, deterministic suite sorting, the path-equals-inline
//! guarantee, canonicalization stability, and the command-JSON boundary.

use crate::{
    canonicalize, hash, hash_report, run_case, run_suite, run_suite_inline, Benchmark,
    BenchmarkCase, Candle, Config, StrategySpec, Suite,
};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fmt::Write as _;
use wickra_backtest_core::run;

/// A small, valid EMA-cross strategy (the shape wickra-backtest accepts).
fn strategy() -> Value {
    json!({
        "symbol": "BTCUSDT",
        "timeframe": "1h",
        "indicators": {
            "ema_fast": { "type": "Ema", "params": [5] },
            "ema_slow": { "type": "Ema", "params": [15] }
        },
        "entry": { "cross_above": ["ema_fast", "ema_slow"] },
        "exit": { "cross_below": ["ema_fast", "ema_slow"] },
        "sizing": { "type": "fixed_fraction", "fraction": 0.95 },
        "costs": { "taker_bps": 5, "slippage": { "type": "fixed_bps", "bps": 2 } },
        "risk": { "trailing_stop_pct": 5.0 }
    })
}

/// A deterministic oscillating candle series long enough to warm up EMA(15) and
/// produce at least one crossing.
fn candles() -> Vec<Candle> {
    (0..40)
        .map(|i| {
            let t = f64::from(i);
            let base = 100.0 + (t * 0.4).sin() * 8.0;
            Candle {
                time: 1_700_000_000 + i64::from(i) * 3600,
                open: base,
                high: base + 1.0,
                low: base - 1.0,
                close: base + 0.5,
                volume: 1000.0,
            }
        })
        .collect()
}

/// The honest recomputed report for `strategy()` on `candles()`, as JSON.
fn recomputed_report() -> Value {
    let spec: StrategySpec = serde_json::from_value(strategy()).unwrap();
    let report = run(&spec, &candles()).unwrap();
    serde_json::to_value(&report).unwrap()
}

/// A fully-blessed case: `expected` and `expected_hash` are both derived from the
/// real recompute, so an honest run passes on both axes.
fn blessed_case(id: &str, dataset_ref: &str) -> BenchmarkCase {
    let expected = recomputed_report();
    let expected_hash = hash(&canonicalize(&expected).unwrap());
    BenchmarkCase {
        id: id.to_string(),
        description: "blessed test case".to_string(),
        strategy: strategy(),
        dataset_ref: dataset_ref.to_string(),
        expected,
        expected_hash,
    }
}

fn datasets(dataset_ref: &str) -> BTreeMap<String, Vec<Candle>> {
    let mut m = BTreeMap::new();
    m.insert(dataset_ref.to_string(), candles());
    m
}

#[test]
fn honest_case_passes_on_both_axes() {
    let case = blessed_case("sma-01", "d.csv");
    let result = run_case(&case, &candles()).unwrap();
    assert_eq!(result.id, "sma-01");
    assert!(result.passed);
    assert!(result.hash_match);
    assert_eq!(result.hash, case.expected_hash);
    assert_eq!(result.recomputed, case.expected);
}

#[test]
fn fudged_expected_fails_passed_but_not_hash() {
    // Someone edits the expected report but leaves the hash alone: `passed`
    // catches it, `hash_match` does not (the hash is over the recompute).
    let mut case = blessed_case("sma-01", "d.csv");
    case.expected["fees_paid"] = json!(999_999.0);
    let result = run_case(&case, &candles()).unwrap();
    assert!(!result.passed);
    assert!(result.hash_match);
}

#[test]
fn fudged_hash_fails_hash_match_but_not_passed() {
    // The mirror case: the expectation is honest but the hash is wrong.
    let mut case = blessed_case("sma-01", "d.csv");
    case.expected_hash = "0".repeat(64);
    let result = run_case(&case, &candles()).unwrap();
    assert!(result.passed);
    assert!(!result.hash_match);
}

#[test]
fn suite_results_are_sorted_and_tallied() {
    // Cases supplied out of order must come back sorted by id.
    let suite = Suite {
        name: "core".to_string(),
        cases: vec![
            blessed_case("z-case", "d.csv"),
            blessed_case("a-case", "d.csv"),
        ],
    };
    let report = run_suite_inline(&suite, &datasets("d.csv")).unwrap();
    let ids: Vec<&str> = report.results.iter().map(|r| r.id.as_str()).collect();
    assert_eq!(ids, ["a-case", "z-case"]);
    assert_eq!(report.passed, 2);
    assert_eq!(report.failed, 0);
}

#[test]
fn path_run_equals_inline_run() {
    let dir = std::env::temp_dir().join(format!("wbench-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let mut body = String::from("time,open,high,low,close,volume\n");
    for c in candles() {
        writeln!(
            body,
            "{},{},{},{},{},{}",
            c.time, c.open, c.high, c.low, c.close, c.volume
        )
        .unwrap();
    }
    std::fs::write(dir.join("d.csv"), body).unwrap();

    let suite = Suite {
        name: "core".to_string(),
        cases: vec![blessed_case("sma-01", "d.csv")],
    };
    let path_report = run_suite(&suite, &dir).unwrap();
    let inline_report = run_suite_inline(&suite, &datasets("d.csv")).unwrap();
    assert_eq!(path_report, inline_report);
    assert_eq!(path_report.passed, 1);

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn missing_dataset_is_a_data_error() {
    let suite = Suite {
        name: "core".to_string(),
        cases: vec![blessed_case("sma-01", "missing.csv")],
    };
    let err = run_suite_inline(&suite, &datasets("d.csv")).unwrap_err();
    assert!(matches!(err, crate::Error::Data(_)));
}

#[test]
fn case_validation_rejects_bad_input() {
    let mut case = blessed_case("sma-01", "d.csv");
    case.id = String::new();
    assert!(BenchmarkCase::from_json(&serde_json::to_string(&case).unwrap()).is_err());

    let mut case = blessed_case("sma-01", "d.csv");
    case.expected_hash = "NOTHEX".to_string();
    assert!(BenchmarkCase::from_json(&serde_json::to_string(&case).unwrap()).is_err());

    let mut case = blessed_case("sma-01", "d.csv");
    case.dataset_ref = String::new();
    assert!(BenchmarkCase::from_json(&serde_json::to_string(&case).unwrap()).is_err());
}

#[test]
fn suite_rejects_duplicate_ids() {
    let suite = Suite {
        name: "core".to_string(),
        cases: vec![blessed_case("dup", "d.csv"), blessed_case("dup", "d.csv")],
    };
    let json = serde_json::to_string(&suite).unwrap();
    assert!(Suite::from_json(&json).is_err());
}

#[test]
fn command_json_run_case_round_trips() {
    let bench = Benchmark::new();
    let req =
        json!({ "cmd": "run_case", "case": blessed_case("sma-01", "d.csv"), "data": candles() });
    let out = bench.command_json(&req.to_string()).unwrap();
    let v: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(v["passed"], json!(true));
    assert_eq!(v["hash_match"], json!(true));
    assert_eq!(v["id"], json!("sma-01"));
}

#[test]
fn command_json_run_suite_and_list_cases() {
    let bench = Benchmark::new();
    let suite = Suite {
        name: "core".to_string(),
        cases: vec![blessed_case("sma-01", "d.csv")],
    };
    let run = json!({ "cmd": "run_suite", "suite": suite, "datasets": datasets("d.csv") });
    let out = bench.command_json(&run.to_string()).unwrap();
    let v: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(v["passed"], json!(1));

    let list = json!({ "cmd": "list_cases", "suite": suite });
    let out = bench.command_json(&list.to_string()).unwrap();
    let v: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(v["ids"], json!(["sma-01"]));
}

#[test]
fn command_json_version_and_unknown() {
    let bench = Benchmark::new();
    let out = bench.command_json(r#"{"cmd":"version"}"#).unwrap();
    let v: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(v["version"], json!(Benchmark::version()));
    assert!(v["engine_version"].is_string());

    let out = bench.command_json(r#"{"cmd":"nope"}"#).unwrap();
    let v: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(v["ok"], json!(false));
    assert!(v["error"].as_str().unwrap().contains("nope"));
}

#[test]
fn canonicalization_is_idempotent_and_key_order_invariant() {
    let a = json!({ "b": 2, "a": 1, "nested": { "y": [1.0, 2.5], "x": true } });
    let b = json!({ "a": 1, "nested": { "x": true, "y": [1.0, 2.5] }, "b": 2 });
    let ca = canonicalize(&a).unwrap();
    assert_eq!(ca, canonicalize(&b).unwrap());
    // Re-parsing the canonical form and re-canonicalizing is a fixed point.
    let reparsed: Value = serde_json::from_str(&ca).unwrap();
    assert_eq!(ca, canonicalize(&reparsed).unwrap());
}

#[test]
fn hash_report_matches_manual_hash() {
    let report = recomputed_report();
    assert_eq!(
        hash_report(&report).unwrap(),
        hash(&canonicalize(&report).unwrap())
    );
}

#[test]
fn config_round_trips() {
    let cfg = Config::default();
    let s = serde_json::to_string(&cfg).unwrap();
    let back: Config = serde_json::from_str(&s).unwrap();
    assert_eq!(cfg, back);
    assert!(serde_json::from_str::<Config>(r#"{"unknown":1}"#).is_err());
}
