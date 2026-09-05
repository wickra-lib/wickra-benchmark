//! Pin the indicator behaviour the corpus depends on.
//!
//! Nothing here computes indicators. The recompute path goes
//! `run_case` -> `wickra-backtest-core` -> its own registry, which resolves a
//! strategy's indicator names internally; `benchmark-core` never sees an
//! `Indicator`. But every frozen `expected_hash` in `cases/` is downstream of
//! this arithmetic, so when a hash moves after a dependency bump there are two
//! candidates and no way to tell them apart from the report alone:
//!
//!   * the engine changed how it turns signals into trades, or
//!   * an indicator changed the numbers those signals are made of.
//!
//! Diffing a recomputed report answers neither. This does: it drives the four
//! indicator families the committed cases actually name, straight from
//! `wickra-core`, against hand-checkable inputs. If it fails alongside a moved
//! hash, the cause is the indicator core. If it passes, the cause is above it.
//!
//! Deliberately only those four. A test that pinned the whole catalogue would
//! fail on indicators no case here uses, which is somebody else's regression
//! reported in the wrong repository. The list grows when a case introduces a
//! family, and `assert_families_are_covered` is what makes that happen: it reads
//! `cases/` and fails if a case names a family this file does not pin.

// Every float compared here is exact by construction, not the result of
// accumulated arithmetic: Donchian *selects* an input high or low rather than
// computing one, and the moving averages are checked on inputs whose means are
// exactly representable ((1+2+3)/3, and the mean of a constant). An epsilon
// would be the weaker assertion -- it would pass on a value that drifted.
#![allow(clippy::float_cmp)]

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;
use wickra_core::{Candle, Donchian, Ema, Indicator, Rsi, Sma};

/// Every family this file pins. Kept beside the tests that pin them so the two
/// cannot drift apart silently.
const PINNED: [&str; 4] = ["Donchian", "Ema", "Rsi", "Sma"];

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// A flat series is the one input whose every indicator output can be reasoned
/// about without a spreadsheet: a moving average of a constant is that constant,
/// and a series that never moves has no gains and no losses.
const FLAT: f64 = 100.0;

#[test]
fn sma_averages_its_window() {
    let mut sma = Sma::new(3).expect("period 3 is valid");
    assert_eq!(sma.warmup_period(), 3);
    assert_eq!(sma.update(1.0), None, "no value before the window is full");
    assert_eq!(sma.update(2.0), None);
    // (1 + 2 + 3) / 3
    assert_eq!(sma.update(3.0), Some(2.0));
    // The window slides: (2 + 3 + 4) / 3
    assert_eq!(sma.update(4.0), Some(3.0));

    let mut flat = Sma::new(10).expect("period 10 is valid");
    for _ in 0..9 {
        flat.update(FLAT);
    }
    assert_eq!(
        flat.update(FLAT),
        Some(FLAT),
        "the mean of a constant is that constant"
    );
}

#[test]
fn ema_starts_at_the_seed_and_weights_the_newest_input() {
    let mut ema = Ema::new(5).expect("period 5 is valid");
    assert_eq!(ema.warmup_period(), 5);

    // A constant series must stay at that constant, whatever the smoothing.
    for _ in 0..4 {
        ema.update(FLAT);
    }
    assert_eq!(ema.update(FLAT), Some(FLAT));

    // Then a step up moves the average toward it without reaching it: the
    // property the crossover cases in the corpus depend on.
    let stepped = ema.update(200.0).expect("ready");
    assert!(
        stepped > FLAT && stepped < 200.0,
        "an EMA must move toward a step without reaching it, got {stepped}"
    );
}

#[test]
fn rsi_is_bounded_and_saturates() {
    let mut rsi = Rsi::new(14).expect("period 14 is valid");
    // 15, not 14: RSI(14) is computed from 14 deltas, and 14 deltas need 15
    // prices. Worth pinning precisely because it is the one warm-up here that
    // does not equal its period -- a case sized against the period alone would
    // be one bar short.
    assert_eq!(rsi.warmup_period(), 15);

    // A strictly rising series has no losses, so RSI pins to its ceiling.
    let mut last = None;
    for i in 0..40 {
        last = rsi.update(100.0 + f64::from(i));
    }
    let value = last.expect("ready after 40 inputs");
    assert!(
        (value - 100.0).abs() < 1e-9,
        "a series with no losses must sit at 100, got {value}"
    );

    // And the range holds on a series that moves both ways.
    let mut mixed = Rsi::new(14).expect("period 14 is valid");
    for i in 0..60 {
        if let Some(v) = mixed.update(100.0 + (f64::from(i) * 0.4).sin() * 8.0) {
            assert!((0.0..=100.0).contains(&v), "RSI left its range: {v}");
        }
    }
}

#[test]
fn donchian_tracks_the_extremes_of_its_window() {
    let mut donchian = Donchian::new(3).expect("period 3 is valid");
    assert_eq!(donchian.warmup_period(), 3);

    // Candle is non-exhaustive and validated, so it is built through its
    // constructor rather than as a struct literal.
    let candle = |high: f64, low: f64| {
        Candle::new(low, high, low, high, 0.0, 0).expect("high >= low is a valid candle")
    };

    assert!(donchian.update(candle(10.0, 5.0)).is_none());
    assert!(donchian.update(candle(12.0, 4.0)).is_none());
    let out = donchian
        .update(candle(11.0, 6.0))
        .expect("ready on the third");
    assert_eq!(out.upper, 12.0, "upper is the highest high in the window");
    assert_eq!(out.lower, 4.0, "lower is the lowest low in the window");

    // The window slides, so the old extremes leave it.
    let out = donchian.update(candle(9.0, 7.0)).expect("ready");
    assert_eq!(out.upper, 12.0);
    assert_eq!(out.lower, 4.0);
    let out = donchian.update(candle(9.5, 7.5)).expect("ready");
    assert_eq!(out.upper, 11.0, "the 12.0 bar has left the window");
    assert_eq!(out.lower, 6.0, "so has the 4.0 bar");
}

#[test]
fn reset_returns_an_indicator_to_its_starting_state() {
    // A case is recomputed from a fresh handle every time, so this is the
    // property that lets the suite runner reuse nothing between cases.
    let mut sma = Sma::new(3).expect("period 3 is valid");
    for value in [1.0, 2.0, 3.0] {
        sma.update(value);
    }
    assert!(sma.is_ready());
    sma.reset();
    assert!(!sma.is_ready(), "reset must undo readiness");
    assert_eq!(sma.update(1.0), None, "and the window with it");
}

#[test]
fn assert_families_are_covered() {
    let dir = repo_root().join("cases");
    let mut named: BTreeSet<String> = BTreeSet::new();

    for entry in fs::read_dir(&dir).expect("cases/") {
        let path = entry.expect("dir entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let text = fs::read_to_string(&path).expect("read case");
        let value: Value = serde_json::from_str(&text).expect("parse case");
        // Both a bare case and suite.json, which nests them under `cases`.
        let strategies = value.get("strategy").map_or_else(
            || {
                value
                    .get("cases")
                    .and_then(Value::as_array)
                    .map(|cases| cases.iter().filter_map(|c| c.get("strategy")).collect())
                    .unwrap_or_default()
            },
            |s| vec![s],
        );
        for strategy in strategies {
            let Some(indicators) = strategy.get("indicators").and_then(Value::as_object) else {
                continue;
            };
            for indicator in indicators.values() {
                if let Some(kind) = indicator.get("type").and_then(Value::as_str) {
                    named.insert(kind.to_string());
                }
            }
        }
    }

    assert!(!named.is_empty(), "no indicators found under cases/");
    let unpinned: Vec<&String> = named
        .iter()
        .filter(|k| !PINNED.contains(&k.as_str()))
        .collect();
    assert!(
        unpinned.is_empty(),
        "cases/ names indicator families this file does not pin: {unpinned:?}. \
         Add a test for each, then list it in PINNED -- otherwise a change to one \
         of them moves a committed hash with nothing here to say so."
    );
}
