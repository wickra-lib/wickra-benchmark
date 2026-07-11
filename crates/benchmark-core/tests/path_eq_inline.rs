//! `run_suite` (path-based, for the CLI) and `run_suite_inline` (data supplied
//! inline, for the FFI boundary) must produce a byte-identical `SuiteReport` for
//! the same data. This pins the promise that the CLI and every binding agree,
//! regardless of whether the datasets came off disk or over the wire.

use benchmark_core::{load_candles, run_suite, run_suite_inline, Candle, Suite};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn path_and_inline_suite_reports_are_byte_identical() {
    let root = repo_root();
    let data_root = root.join("datasets");
    let suite =
        Suite::from_json(&std::fs::read_to_string(root.join("cases/suite.json")).unwrap()).unwrap();

    // Path-based: the runner loads each dataset from disk.
    let via_path = run_suite(&suite, &data_root).unwrap();

    // Inline: load the datasets ourselves and hand them over as a map.
    let mut datasets: BTreeMap<String, Vec<Candle>> = BTreeMap::new();
    for case in &suite.cases {
        datasets
            .entry(case.dataset_ref.clone())
            .or_insert_with(|| load_candles(&data_root.join(&case.dataset_ref)).unwrap());
    }
    let via_inline = run_suite_inline(&suite, &datasets).unwrap();

    // Byte-identical serialization is the strongest form of "same report".
    assert_eq!(
        serde_json::to_string(&via_path).unwrap(),
        serde_json::to_string(&via_inline).unwrap(),
    );

    // And the committed suite self-passes, both ways.
    assert_eq!(via_path.failed, 0);
    assert_eq!(via_path.passed, suite.cases.len());
    assert_eq!(via_inline.failed, 0);
}
