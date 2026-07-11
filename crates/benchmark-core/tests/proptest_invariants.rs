//! Property tests: over a wide range of bounded, valid case suites the runner
//! never panics, `results.len() == cases.len()`, `passed + failed == len`, the
//! results are always sorted ascending by `id`, and a suite of blessed cases
//! passes in full. Canonicalization is idempotent and key-order invariant.

mod common;

use benchmark_core::{canonicalize, run_suite_inline, BenchmarkCase, Candle, Suite};
use proptest::prelude::*;
use serde_json::{json, Value};
use std::collections::BTreeMap;

/// Build a suite of `n` blessed cases over independently-shaped universes, with
/// unique ids (so the suite validates) and one dataset per case.
fn blessed_suite(shapes: &[Vec<f64>]) -> (Suite, BTreeMap<String, Vec<Candle>>) {
    let mut cases: Vec<BenchmarkCase> = Vec::new();
    let mut datasets: BTreeMap<String, Vec<Candle>> = BTreeMap::new();
    for (i, closes) in shapes.iter().enumerate() {
        let id = format!("case-{i:03}");
        let candles = common::candles_from(closes);
        let case = common::bless(&id, "prop case", common::strategy_json(), &candles);
        datasets.insert(case.dataset_ref.clone(), candles);
        cases.push(case);
    }
    (
        Suite {
            name: "prop suite".to_string(),
            cases,
        },
        datasets,
    )
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(32))]

    /// The runner never panics and the report invariants always hold.
    #[test]
    fn suite_report_invariants(
        shapes in prop::collection::vec(
            prop::collection::vec(50.0f64..200.0f64, 16..40),
            1..5,
        ),
    ) {
        let (suite, datasets) = blessed_suite(&shapes);
        let report = run_suite_inline(&suite, &datasets).unwrap();

        // One result per case, and the tally is exact.
        prop_assert_eq!(report.results.len(), suite.cases.len());
        prop_assert_eq!(report.passed + report.failed, report.results.len());

        // Results are always sorted ascending by id.
        let ids: Vec<&str> = report.results.iter().map(|r| r.id.as_str()).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        prop_assert_eq!(ids, sorted);

        // Blessed cases self-pass: every case passed and matched its hash.
        prop_assert_eq!(report.failed, 0);
        prop_assert_eq!(report.passed, suite.cases.len());
        prop_assert!(report.results.iter().all(|r| r.passed && r.hash_match));
    }

    /// Reordering the case list does not change the report (results re-sorted).
    #[test]
    fn case_order_does_not_matter(
        shapes in prop::collection::vec(
            prop::collection::vec(50.0f64..200.0f64, 16..32),
            2..5,
        ),
    ) {
        let (mut suite, datasets) = blessed_suite(&shapes);
        let forward = run_suite_inline(&suite, &datasets).unwrap();
        suite.cases.reverse();
        let reversed = run_suite_inline(&suite, &datasets).unwrap();
        prop_assert_eq!(
            serde_json::to_string(&forward).unwrap(),
            serde_json::to_string(&reversed).unwrap(),
        );
    }

    /// Canonicalization is idempotent: re-canonicalizing a canonical string's
    /// parse yields the identical string.
    #[test]
    fn canonicalize_is_idempotent(closes in prop::collection::vec(50.0f64..200.0f64, 16..40)) {
        let (report, _) = common::recompute(&common::strategy_json(), &common::candles_from(&closes));
        let once = canonicalize(&report).unwrap();
        let reparsed: Value = serde_json::from_str(&once).unwrap();
        let twice = canonicalize(&reparsed).unwrap();
        prop_assert_eq!(once, twice);
    }
}

#[test]
fn canonicalize_is_key_order_invariant() {
    let a = json!({ "z": 1, "a": { "n": 2, "m": 3 }, "k": [3, 2, 1] });
    let b = json!({ "k": [3, 2, 1], "a": { "m": 3, "n": 2 }, "z": 1 });
    assert_eq!(canonicalize(&a).unwrap(), canonicalize(&b).unwrap());
}
