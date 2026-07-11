//! Golden parity: replay every committed `golden/commands/*.json` through the
//! same canonical `command_json` surface every binding uses, and assert the
//! response is byte-for-byte identical to `golden/expected/<name>.json`. This is
//! the Rust anchor of the cross-language determinism guarantee; the ten bindings
//! assert the same bytes. It also checks the semantic promise: every blessed
//! `run_case`/`run_suite` fixture reproduces (`passed`/`hash_match`, `failed:0`).

use benchmark_core::Benchmark;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

fn golden_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../golden")
}

#[test]
fn golden_commands_are_byte_identical() {
    let dir = golden_dir();
    let benchmark = Benchmark::new();
    let mut count = 0;
    let mut run_case_seen = 0;
    let mut suite_run_seen = false;

    for entry in fs::read_dir(dir.join("commands")).unwrap() {
        let cmd_path = entry.unwrap().path();
        if cmd_path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let name = cmd_path.file_name().unwrap().to_string_lossy().into_owned();
        let cmd_json = fs::read_to_string(&cmd_path).unwrap();

        // Drive the exact canonical command surface the bindings use.
        let got = benchmark.command_json(&cmd_json).unwrap();

        let expected = fs::read_to_string(dir.join("expected").join(&name)).unwrap();
        assert_eq!(
            got.trim(),
            expected.trim(),
            "golden response mismatch for {name}"
        );

        // Semantic checks per command kind.
        let envelope: Value = serde_json::from_str(&cmd_json).unwrap();
        let response: Value = serde_json::from_str(&got).unwrap();
        match envelope["cmd"].as_str().unwrap() {
            "run_case" => {
                assert_eq!(response["passed"], true, "{name} must pass");
                assert_eq!(response["hash_match"], true, "{name} hash must match");
                run_case_seen += 1;
            }
            "run_suite" => {
                assert_eq!(response["failed"], 0, "{name} suite must have no failures");
                assert!(response["passed"].as_u64().unwrap() > 0);
                suite_run_seen = true;
            }
            _ => {}
        }
        count += 1;
    }

    assert!(
        count >= 8,
        "expected at least eight golden fixtures, got {count}"
    );
    assert_eq!(run_case_seen, 5, "expected five run_case fixtures");
    assert!(suite_run_seen, "expected a run_suite fixture");
}
