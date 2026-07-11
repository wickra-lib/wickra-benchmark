//! Serde conformance for the public wire types: `BenchmarkCase`/`Suite`/
//! `CaseResult`/`SuiteReport` round-trip through JSON (and TOML where the CLI
//! accepts it), the embedded `strategy` `Value` survives a round-trip untouched,
//! canonicalization is key-order invariant, and the structural guards
//! (`deny_unknown_fields`, hex hash shape, object-only strategy/expected,
//! duplicate ids) reject bad input rather than silently accepting it.

mod common;

use benchmark_core::{canonicalize, BenchmarkCase, CaseResult, Suite, SuiteReport};

#[test]
fn case_json_round_trips() {
    let case = common::sample_case();
    let json = serde_json::to_string(&case).unwrap();
    let back = BenchmarkCase::from_json(&json).unwrap();
    assert_eq!(case, back);
}

#[test]
fn strategy_value_passes_through_untouched() {
    // benchmark-core keeps `strategy` as raw JSON; a round-trip must not perturb
    // it (no key reordering that would matter, no numeric coercion).
    let case = common::sample_case();
    let original = case.strategy.clone();
    let json = serde_json::to_string(&case).unwrap();
    let back = BenchmarkCase::from_json(&json).unwrap();
    assert_eq!(back.strategy, original);
    assert_eq!(back.strategy["indicators"]["ema_fast"]["params"][0], 3);
}

#[test]
fn case_toml_round_trips() {
    let toml = r#"
id = "sma-01"
description = "SMA smoke"
dataset_ref = "sma.csv"
expected_hash = "0000000000000000000000000000000000000000000000000000000000000000"

[strategy]
symbol = "TEST"
timeframe = "1h"

[expected]
schema_version = 1
"#;
    let case = BenchmarkCase::from_toml(toml).unwrap();
    assert_eq!(case.id, "sma-01");
    assert_eq!(case.dataset_ref, "sma.csv");
    assert!(case.strategy.is_object());
    assert!(case.expected.is_object());
}

#[test]
fn suite_json_round_trips() {
    let suite = Suite {
        name: "round-trip suite".to_string(),
        cases: vec![common::sample_case()],
    };
    let json = serde_json::to_string(&suite).unwrap();
    let back = Suite::from_json(&json).unwrap();
    assert_eq!(suite, back);
    assert_eq!(back.case_ids(), vec!["ema-cross-01".to_string()]);
}

#[test]
fn case_result_and_suite_report_round_trip() {
    let result = CaseResult {
        id: "ema-cross-01".to_string(),
        passed: true,
        hash_match: true,
        recomputed: serde_json::json!({ "schema_version": 1 }),
        hash: "a".repeat(64),
    };
    let back: CaseResult = serde_json::from_str(&serde_json::to_string(&result).unwrap()).unwrap();
    assert_eq!(result, back);

    let report = SuiteReport {
        results: vec![result],
        passed: 1,
        failed: 0,
    };
    let back: SuiteReport = serde_json::from_str(&serde_json::to_string(&report).unwrap()).unwrap();
    assert_eq!(report, back);
}

#[test]
fn bad_expected_hash_is_rejected() {
    // A hash that is not 64 lowercase hex characters fails validation.
    let mut case = common::sample_case();
    case.expected_hash = "not-a-hash".to_string();
    let json = serde_json::to_string(&case).unwrap();
    assert!(BenchmarkCase::from_json(&json).is_err());

    // Uppercase hex is also rejected (digests are lowercase).
    let mut case = common::sample_case();
    case.expected_hash = "A".repeat(64);
    let json = serde_json::to_string(&case).unwrap();
    assert!(BenchmarkCase::from_json(&json).is_err());
}

#[test]
fn duplicate_case_id_is_rejected() {
    // The ids are the sort and tie key; a collision makes the report order
    // ambiguous, so the suite refuses to validate.
    let case = common::sample_case();
    let suite = Suite {
        name: "dup".to_string(),
        cases: vec![case.clone(), case],
    };
    let json = serde_json::to_string(&suite).unwrap();
    assert!(Suite::from_json(&json).is_err());
}

#[test]
fn unknown_case_field_is_rejected() {
    // `deny_unknown_fields`: an extra top-level key fails to parse.
    let json = r#"{
        "id": "x",
        "description": "d",
        "strategy": {},
        "dataset_ref": "x.csv",
        "expected": {},
        "expected_hash": "0000000000000000000000000000000000000000000000000000000000000000",
        "surprise": 1
    }"#;
    assert!(BenchmarkCase::from_json(json).is_err());
}

#[test]
fn non_object_strategy_and_expected_are_rejected() {
    let zeros = "0".repeat(64);
    // The strategy must be a StrategySpec object.
    let bad_strategy = format!(
        r#"{{"id":"x","description":"d","strategy":42,"dataset_ref":"x.csv","expected":{{}},"expected_hash":"{zeros}"}}"#
    );
    assert!(BenchmarkCase::from_json(&bad_strategy).is_err());
    // The expected report must be a BacktestReport object.
    let bad_expected = format!(
        r#"{{"id":"x","description":"d","strategy":{{}},"dataset_ref":"x.csv","expected":7,"expected_hash":"{zeros}"}}"#
    );
    assert!(BenchmarkCase::from_json(&bad_expected).is_err());
}

#[test]
fn empty_id_and_dataset_ref_are_rejected() {
    let zeros = "0".repeat(64);
    let empty_id = format!(
        r#"{{"id":"","description":"d","strategy":{{}},"dataset_ref":"x.csv","expected":{{}},"expected_hash":"{zeros}"}}"#
    );
    assert!(BenchmarkCase::from_json(&empty_id).is_err());
    let empty_ref = format!(
        r#"{{"id":"x","description":"d","strategy":{{}},"dataset_ref":"","expected":{{}},"expected_hash":"{zeros}"}}"#
    );
    assert!(BenchmarkCase::from_json(&empty_ref).is_err());
}

#[test]
fn canonicalize_is_key_order_invariant() {
    let a = serde_json::json!({ "b": 1, "a": { "z": 2, "y": 3 }, "c": [1, 2] });
    let b = serde_json::json!({ "c": [1, 2], "a": { "y": 3, "z": 2 }, "b": 1 });
    assert_eq!(canonicalize(&a).unwrap(), canonicalize(&b).unwrap());
}

#[test]
fn canonicalize_collapses_non_finite_to_null() {
    // JSON has no NaN/Infinity: serde collapses a non-finite number to `null`,
    // deterministically, so canonicalization never panics on one.
    assert_eq!(canonicalize(&f64::NAN).unwrap(), "null");
    assert_eq!(canonicalize(&f64::INFINITY).unwrap(), "null");
    assert_eq!(canonicalize(&f64::NEG_INFINITY).unwrap(), "null");
}
