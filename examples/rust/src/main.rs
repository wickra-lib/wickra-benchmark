//! A runnable Rust example: load a curated benchmark case and its dataset, then
//! recompute the report with the real engine and confirm it reproduces — both
//! `passed` (the report matches the frozen expectation) and `hash_match` (its
//! canonical hash matches). This is the whole product in one file.
//!
//! ```bash
//! cargo run -p wickra-benchmark-example
//! ```

use std::path::Path;

use benchmark_core::{load_candles, run_case, BenchmarkCase};

fn main() {
    let data_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../data");

    let case = BenchmarkCase::from_json(
        &std::fs::read_to_string(data_dir.join("cases/sma-crossover-01.json"))
            .expect("read the case"),
    )
    .expect("parse the case");

    let candles =
        load_candles(&data_dir.join("datasets").join(&case.dataset_ref)).expect("load the dataset");

    let result = run_case(&case, &candles).expect("run the case");

    println!("wickra-benchmark {}", benchmark_core::version());
    println!(
        "{}: passed={} hash_match={}",
        result.id, result.passed, result.hash_match
    );

    assert!(
        result.passed && result.hash_match,
        "the curated case must reproduce"
    );
    println!("REPRODUCED (passed + hash_match)");
}
